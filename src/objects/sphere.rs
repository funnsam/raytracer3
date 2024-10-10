use smolmatrix::*;

use crate::ray::Ray;
use super::*;

pub struct Sphere {
    pub center: Vector<3>,
    pub radius: f32,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, range: core::ops::Range<f32>) -> Option<HitInfo> {
        let oc = self.center.clone() - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let d = h * h - a * c;

        (d >= 0.0).then(|| {
            let sqrt_d = d.sqrt();

            let mut root = (h - sqrt_d) / a;
            if root <= range.start || range.end <= root {
                root = (h + sqrt_d) / a;
                if root <= range.start || range.end <= root {
                    return None;
                }
            }

            Some(HitInfo {
                distance: root,
                normal: (ray.at(root) - &self.center).unit(),
                front_face: false,
            }.correct_normal(ray))
        }).flatten()
    }
}
