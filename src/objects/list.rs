use crate::ray::Ray;
use super::*;

pub struct List<'a> {
    pub objects: Vec<&'a dyn Object<'a>>,
}

impl<'a> Object<'a> for List<'a> {
    fn hit(&self, ray: &Ray, range: core::ops::Range<f32>) -> Option<HitInfo<'a>> {
        let mut closest = range.end;

        self.objects.iter().fold(None, |best, obj| {
            let h = obj.hit(ray, range.start..closest).or(best);
            if let Some(h) = &h { closest = h.distance; }

            h
        })
    }
}
