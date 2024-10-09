use smolmatrix::Vector;

use crate::ray::Ray;

pub mod sphere;
pub mod list;

pub trait Object {
    fn hit(&self, ray: &Ray, range: core::ops::Range<f32>) -> Option<HitInfo>;
}

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vector<3>,
    pub front_face: bool,
}

impl HitInfo {
    pub fn correct_normal(mut self, ray: &Ray) -> Self {
        self.front_face = ray.direction().dot(&self.normal) < 0.0;

        if !self.front_face {
            self.normal.map_each_in_place(|e| *e = -*e);
        }

        self
    }
}
