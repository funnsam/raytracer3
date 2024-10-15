use smolmatrix::*;

pub mod color;
pub mod objects;
pub mod materials;
pub mod ray;

mod lambertian;
mod ggx;
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
    pub look_at: Vector<3>,
    pub v_up: Vector<3>,
    pub v_fov: f32,

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
            look_at: vector!(3 [0.0, 0.0, -1.0]),
            v_up: vector!(3 [0.0, 1.0, 0.0]),
            v_fov: 90.0_f32.to_radians(),

            width,
            height,
        }
    }

    pub fn get_framing_info(&self) -> FramingInfo {
        let focal_length = (self.center.clone() - &self.look_at).length();
        let h = (self.v_fov / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.width as f32 / self.height as f32);

        let w = (self.center.clone() - &self.look_at).unit();
        let u = self.v_up.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        let pixel_du = viewport_u.clone() / self.width as f32;
        let pixel_dv = viewport_v.clone() / self.height as f32;

        let ul = self.center.clone() - &(w * focal_length) - &(viewport_u / 2.0) - &(viewport_v / 2.0);
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
            let v = -ray.direction().clone();
            let n_dot_v = hit.normal.dot(&v);

            let origin = ray.at(hit.distance);

            let ior = if hit.front_face { 1.0 / hit.bsdf.ior } else { hit.bsdf.ior };
            let cos_theta = n_dot_v.min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let f0_sqrt = (ior - 1.0) / (ior + 1.0);
            let f0 = f0_sqrt * f0_sqrt;

            let f_reflectance = f0 + (1.0 - f0) * (1.0 - cos_theta).powi(5);

            let cant_reflect = ior * sin_theta > 1.0 || f_reflectance > fastrand::f32();

            let reflect = if hit.bsdf.transmission.weight < 1.0 || cant_reflect {
                let alpha = (hit.bsdf.roughness * hit.bsdf.roughness).max(0.01);
                let f0 = hit.bsdf.base_color.0.clone() * hit.bsdf.metallic + f0 * (1.0 - hit.bsdf.metallic);
                let onb = utils::get_basis(hit.normal.clone());

                let mut specular = Vector::new_zeroed();
                let mut ks = Vector::new_zeroed();
                for _ in 0..settings.rays_per_specular * (hit.bsdf.metallic > 0.0) as usize {
                    let vtan = &onb.transpose() * &v;
                    let mut htan = ggx::sample_vndf(&vtan, alpha);
                    if vtan.z() <= 0.0 { htan = -htan; }
                    let ltan = utils::reflect(-vtan, htan);
                    let l = &onb * &ltan;

                    let h = (l.clone() + &v).unit();
                    let v_dot_h = v.dot(&h);
                    let h_dot_n = h.dot(&hit.normal);
                    let n_dot_l = hit.normal.dot(&l);

                    let d = ggx::d(alpha, h_dot_n);
                    let g1_n_dot_v = ggx::g1(alpha, n_dot_v);
                    let g = g1_n_dot_v * ggx::g1(alpha, n_dot_l);

                    let f = f0.clone() + &((-f0.clone() + 1.0) * (1.0 - v_dot_h).powi(5));
                    ks += &f;

                    let r_s = (f * d * g) / (4.0 * n_dot_l * n_dot_v).max(0.001);

                    let ray = ray::Ray::new_normalized(l, origin.clone());
                    let c = self.ray_color(settings, &ray, depth - 1).0;
                    let p = ggx::pdf(hit.bsdf.metallic, d, g1_n_dot_v, n_dot_l);
                    specular += &(c * &r_s * n_dot_l / p);
                }

                // ks = (ks / settings.rays_per_specular as f32).map_each(|e| *e = e.max(0.0).min(1.0));

                let mut diffuse = Vector::new_zeroed();
                for _ in 0..settings.rays_per_diffuse * (hit.bsdf.metallic < 1.0) as usize {
                    let l = lambertian::sample(&hit.normal);
                    let h = (l.clone() + &v).unit();
                    let n_dot_l = hit.normal.dot(&l);
                    let h_dot_n = h.dot(&hit.normal);

                    let ray = ray::Ray::new_normalized(l, origin.clone());
                    let c = self.ray_color(settings, &ray, depth - 1).0;

                    let d = ggx::d(alpha, h_dot_n);
                    let g1_n_dot_v = ggx::g1(alpha, n_dot_v);
                    let p = ggx::pdf(hit.bsdf.metallic, d, g1_n_dot_v, n_dot_l);

                    diffuse += &(c * n_dot_l / p);
                }

                // let kd = 1.0 - hit.bsdf.metallic;
                let specular = specular / settings.rays_per_specular as f32;
                let diffuse = hit.bsdf.base_color.0.clone() * &(diffuse / settings.rays_per_diffuse as f32);

                specular + &diffuse
            } else {
                Vector::new_zeroed()
            };

            let refract = if hit.bsdf.transmission.weight > 0.0 && !cant_reflect {
                let perp = (ray.direction().clone() + &(hit.normal.clone() * cos_theta)) * ior;
                let parl = hit.normal.clone() * -(1.0 - perp.length_squared()).abs().sqrt();
                let direction = perp + &parl;

                self.ray_color(settings, &ray::Ray::new(direction, origin), depth - 1).0
            } else {
                Vector::new_zeroed()
            };

            return color::Color(if cant_reflect {
                reflect
            } else {
                reflect * (1.0 - hit.bsdf.transmission.weight) + &(refract * hit.bsdf.transmission.weight)
            } + &(hit.bsdf.emission.color.0.clone() * hit.bsdf.emission.strength));
        }

        color::Color(vector!(3 [1.0, 0.98, 0.99]) * 0.75)
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
    pub rays_per_px: usize = 2,
    pub rays_per_specular: usize = 2,
    pub rays_per_diffuse: usize = 2,
    pub depth: usize = 8,
}
