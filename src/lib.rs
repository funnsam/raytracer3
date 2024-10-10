use smolmatrix::*;

pub mod color;
pub mod objects;
pub mod ray;
mod utils;

use objects::Object;

pub struct State {
    pub camera: Camera,
    pub scene: Scene,
    pub settings: Settings,
}

impl State {
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

            color += &self.scene.ray_color(&ray, self.settings.depth).0;
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

pub struct Scene {
    pub world: objects::list::List,
}

impl Scene {
    fn ray_color(&self, ray: &ray::Ray, depth: usize) -> color::Color {
        if depth == 0 {
            return color::Color(Vector::new_zeroed());
        }

        if let Some(h) = self.world.hit(&ray, 0.0001..f32::INFINITY) {
            // normal
            // return color::Color(h.normal.map_each(|e| *e = 0.5 * (*e + 1.0)));

            let p = ray.at(h.distance);
            let ray = ray::Ray::new(h.normal, p);
            return color::Color(self.ray_color(&ray, depth - 1).0 * 0.95);
        }

        let a = 0.5 * (ray.direction()[1] + 1.0);
        color::Color(vector!(3 [1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0]))
    }
}

pub struct Settings {
    pub rays_per_px: usize,
    pub depth: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rays_per_px: 16,
            depth: 24,
        }
    }
}
