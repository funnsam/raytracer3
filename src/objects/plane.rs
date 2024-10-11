use smolmatrix::*;

use crate::ray::Ray;
use super::*;

pub struct Plane<'a> {
    pub origin: Vector<3>,
    pub normal: Vector<3>,
    pub bsdf: &'a crate::materials::Bsdf,
}

impl<'a> Object<'a> for Plane<'a> {
    fn hit(&self, ray: &Ray, range: core::ops::Range<f32>) -> Option<HitInfo<'a>> {
        let d = self.normal.dot(ray.direction());
        if d <= 0.0001 { return None; }

        let pl = self.origin.clone() - ray.origin();
        let t = pl.dot(&self.normal) / d;

        (t > range.start && range.end > t).then(|| HitInfo {
            distance: t,
            normal: -self.normal.clone(),
            front_face: false,
            bsdf: self.bsdf,
        }.correct_normal(ray))
    }
}
