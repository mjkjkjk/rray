use crate::hittable::{HitRecord, HitResult, Hittable};
use crate::ray::Ray;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult {
        let mut closest_so_far = ray_tmax;
        let mut hit_anything = false;
        let mut hit_record: Option<HitRecord> = None;

        for hittable in self {
            let result = hittable.hit(ray, ray_tmin, closest_so_far);
            if let Some(record) = result.hit_record {
                hit_record = Some(record.clone());
                closest_so_far = record.t;
                hit_anything = true;
            }
        }

        HitResult {
            hit: hit_anything,
            hit_record: hit_record,
        }
    }
}
