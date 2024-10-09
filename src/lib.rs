use smolmatrix::*;

pub mod color;
pub mod objects;
pub mod ray;

use objects::Object;

pub struct State {
    pub camera: Camera,
    pub scene: Scene,
}

impl State {
    pub fn get_color(&mut self, x: usize, y: usize) -> color::Color {
        let cache = self.camera.get_cache();

        let pixel = cache.pixel_00 + &(cache.pixel_du * x as f32) + &(cache.pixel_dv * y as f32);
        let direction = pixel - &self.camera.center;
        let ray = ray::Ray::new_normalized(direction, self.camera.center.clone());

        self.scene.ray_color(ray)
    }
}

pub struct Camera {
    pub center: Vector<3>,
    pub direction: Vector<3>,
    pub focal_length: f32,
    pub viewport_height: f32,

    pub width: usize,
    pub height: usize,

    cache: Option<CameraCache>,
}

#[derive(Clone)]
struct CameraCache {
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

            cache: None,
        }
    }

    pub fn update_camera(&mut self) {
        self.cache = None;
    }

    fn get_cache(&mut self) -> CameraCache {
        match self.cache.as_ref() {
            Some(c) => c.clone(),
            None => {
                let viewport_width = self.viewport_height * (self.width as f32 / self.height as f32);

                let viewport_u = vector!(3 [viewport_width, 0.0, 0.0]);
                let viewport_v = vector!(3 [0.0, -self.viewport_height, 0.0]);

                let pixel_du = viewport_u.clone() / self.width as f32;
                let pixel_dv = viewport_v.clone() / self.height as f32;

                let ul = self.center.clone() - &vector!(3 [0.0, 0.0, self.focal_length]) - &(viewport_u / 2.0) - &(viewport_v / 2.0);
                let pixel_00 = ul + &((pixel_du.clone() + &pixel_dv) * 0.5);

                self.cache = Some(CameraCache {
                    pixel_du,
                    pixel_dv,
                    pixel_00,
                });
                self.cache.as_ref().unwrap().clone()
            },
        }
    }
}

pub struct Scene {
    pub world: objects::list::List,
}

impl Scene {
    fn ray_color(&self, ray: ray::Ray) -> color::Color {
        if let Some(h) = self.world.hit(&ray, 0.0..f32::INFINITY) {
            if h.distance > 0.0 {
                return color::Color(h.normal.map_each(|e| *e = 0.5 * (*e + 1.0)));
            }
        }

        let a = 0.5 * (ray.direction()[1] + 1.0);
        color::Color(vector!(3 [1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0]))
    }
}
