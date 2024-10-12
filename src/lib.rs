use smolmatrix::*;

pub mod color;
pub mod objects;
pub mod materials;
pub mod ray;
mod utils;

use objects::Object;

pub struct State<'a> {
    pub camera: Camera,
    pub scene: Scene<'a>,
    pub settings: Settings,
}

impl State<'_> {
    pub fn get_color(&self, framing: FramingInfo, x: usize, y: usize) -> color::Color {
        let mut color = Vector::new_zeroed();
        for _ in 0..self.settings.rays_per_px {
            let off_x = fastrand::f32() - 0.5;
            let off_y = fastrand::f32() - 0.5;

            let pixel = framing.pixel_00.clone()
                + &(framing.pixel_du.clone() * (x as f32 + off_x))
                + &(framing.pixel_dv.clone() * (y as f32 + off_y));
            let direction = pixel - &self.camera.center;
            let ray = ray::Ray::new_normalized(direction, self.camera.center.clone());

            color += &self.scene.ray_color(&self.settings, &ray, self.settings.depth).0;
        }

        color::Color(color / self.settings.rays_per_px as f32)
    }
}

pub struct Camera {
    pub center: Vector<3>,
    pub direction: Vector<3>,
    pub focal_length: f32,
    pub viewport_height: f32,

    pub width: usize,
    pub height: usize,
}

#[derive(Clone)]
pub struct FramingInfo {
    pixel_du: Vector<3>,
    pixel_dv: Vector<3>,
    pixel_00: Vector<3>,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            center: vector!(3 [0.0, 0.0, 0.0]),
            direction: vector!(3 [0.0, 0.0, 0.0]), // TODO:
            focal_length: 1.0,
            viewport_height: 2.0,

            width,
            height,
        }
    }

    pub fn get_framing_info(&self) -> FramingInfo {
        let viewport_width = self.viewport_height * (self.width as f32 / self.height as f32);

        let viewport_u = vector!(3 [viewport_width, 0.0, 0.0]);
        let viewport_v = vector!(3 [0.0, -self.viewport_height, 0.0]);

        let pixel_du = viewport_u.clone() / self.width as f32;
        let pixel_dv = viewport_v.clone() / self.height as f32;

        let ul = self.center.clone() - &vector!(3 [0.0, 0.0, self.focal_length]) - &(viewport_u / 2.0) - &(viewport_v / 2.0);
        let pixel_00 = ul + &((pixel_du.clone() + &pixel_dv) * 0.5);

        FramingInfo {
            pixel_du,
            pixel_dv,
            pixel_00,
        }
    }
}

pub struct Scene<'a> {
    pub world: objects::list::List<'a>,
}

impl Scene<'_> {
    fn ray_color(&self, settings: &Settings, ray: &ray::Ray, depth: usize) -> color::Color {
        if depth == 0 {
            return color::Color(Vector::new_zeroed());
        }

        if let Some(hit) = self.world.hit(&ray, 0.0001..f32::INFINITY) {
            // normal
            // return color::Color(h.normal.map_each(|e| *e = 0.5 * (*e + 1.0)));

            let origin = ray.at(hit.distance);

            // https://graphicscompendium.com/gamedev/15-pbr
            use core::f32::consts::{FRAC_1_PI, PI};

            let v = -ray.direction().clone();
            let n_dot_v = hit.normal.dot(&v);
            let alpha = (hit.bsdf.roughness * hit.bsdf.roughness).max(0.01);
            let f0_sqrt = (hit.bsdf.ior - 1.0) / (hit.bsdf.ior + 1.0);
            let f0 = f0_sqrt * f0_sqrt;

            let mut specular = Vector::new_zeroed();
            let mut diffuse = Vector::new_zeroed();

            for _ in 0..settings.rays_per_hit {
                let xi = fastrand::f32();
                let theta = ((alpha * xi.sqrt()) / (1.0 - xi).sqrt()).atan();
                let phi = 2.0 * PI * fastrand::f32();
                let l = vector!(3 [theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin()]);
                let ct = hit.normal.cross(ray.direction()).unit();
                let t = hit.normal.cross(&ct).unit();
                let m = matrix!(3 x 3
                    [ct[0], hit.normal[0], t[0]]
                    [ct[1], hit.normal[1], t[1]]
                    [ct[2], hit.normal[2], t[2]]
                );
                let l = &m * &l;

                let h = (l.clone() + &v).unit();
                let v_dot_h = v.dot(&h);
                let h_dot_n = h.dot(&hit.normal);
                let n_dot_l = hit.normal.dot(&l);

                let sq = alpha / (h_dot_n * h_dot_n * (alpha * alpha - 1.0) + 1.0);
                let d = FRAC_1_PI * sq * sq;

                let g1 = |x_dot_n: f32| 2.0 / (1.0 + (1.0 + alpha * alpha * ((1.0 * x_dot_n * x_dot_n) / (x_dot_n * x_dot_n))).sqrt());
                let g = g1(n_dot_v) * g1(n_dot_l);

                let f = f0 + (1.0 - f0) * (1.0 - v_dot_h).powi(5);

                let r_s = (d * g * f) / (4.0 * n_dot_l * n_dot_v).max(0.001);

                let ray = ray::Ray::new_normalized(l, origin.clone());
                let c = self.ray_color(settings, &ray, depth - 1).0;
                diffuse += &(c.clone() * n_dot_l);

                let p = d * h_dot_n.abs();
                specular += &(c * r_s * n_dot_l / p);
            }

            let specular = specular / settings.rays_per_hit as f32 * hit.bsdf.metallic;
            let diffuse = hit.bsdf.base_color.0.clone() * (1.0 - hit.bsdf.metallic) / PI * &(diffuse / settings.rays_per_hit as f32);

            return color::Color(specular + &diffuse + &(hit.bsdf.emission.color.0.clone() * hit.bsdf.emission.strength));
        }

        let a = 0.5 * (ray.direction()[1] + 1.0);
        color::Color(vector!(3 [1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0]))
    }
}

macro_rules! settings {
    { $($pub:vis $field:ident : $type:ty = $dv:expr),* $(,)? } => {
        pub struct Settings {
            $($pub $field: $type),*
        }

        impl Settings {
            #[allow(unused)]
            $($pub fn $field(mut self, $field: $type) -> Self {
                self.$field = $field;
                self
            })*
        }

        impl Default for Settings {
            fn default() -> Self {
                Self {
                    $($field: $dv),*
                }
            }
        }
    };
}

settings! {
    pub rays_per_px: usize = 16,
    pub rays_per_hit: usize = 2,
    pub depth: usize = 8,
}
