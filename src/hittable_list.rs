use crate::hittable::{HitRecord, Hittable};
use crate::math::interval::Interval;
use crate::ray::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.max;
        let mut hit_record = None;

        for object in self {
            if let Some(record) = object.hit(ray, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
