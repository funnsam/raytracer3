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
            use core::f32::consts::PI;

            let n_dot_v = hit.normal.dot(ray.direction());
            let alpha = (hit.bsdf.roughness * hit.bsdf.roughness).max(0.01);
            let mut specular = Vector::new_zeroed();

            for _ in 0..settings.rays_per_hit {
                let l = utils::random_hemisphere_vector(&hit.normal);
                let h = (l.clone() + ray.direction()).unit();
                let v_dot_h = ray.direction().dot(&h);
                let h_dot_n = h.dot(&hit.normal);
                let n_dot_l = hit.normal.dot(&l);

                let d = (1.0 / (PI * alpha * alpha)) * h_dot_n.powf(2.0 / alpha - 2.0);
                let g = ((2.0 * h_dot_n * n_dot_v) / v_dot_h).min((2.0 * h_dot_n * n_dot_l) / v_dot_h).min(1.0);

                let f0 = 0.0; // (hit.bsdf.refraction - 1.0) / (n + 1.0);
                let f = f0 + (1.0 - f0) * (1.0 - v_dot_h).powi(5);

                let r_s = (d * g * f) / (4.0 * n_dot_l * n_dot_v).max(0.001);

                let ray = ray::Ray::new_normalized(l, origin.clone());
                specular += &(self.ray_color(settings, &ray, depth - 1).0 * r_s * n_dot_l);
            }

            let specular = specular / settings.rays_per_hit as f32 * hit.bsdf.metallic;
            let diffuse = hit.bsdf.base_color.0.clone() * (1.0 - hit.bsdf.metallic) / PI;

            return color::Color(specular + &diffuse);
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
