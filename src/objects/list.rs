use crate::ray::Ray;
use super::*;

pub struct List {
    pub objects: Vec<Box<dyn Object>>,
}

impl Object for List {
    fn hit(&self, ray: &Ray, range: core::ops::Range<f32>) -> Option<HitInfo> {
        let mut closest = range.end;

        self.objects.iter().fold(None, |best, obj| {
            let h = obj.hit(ray, range.start..closest).or(best);
            if let Some(h) = &h { closest = h.distance; }

            h
        })
    }
}
