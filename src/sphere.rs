use crate::hittable::{HitRecord, HitResult, Hittable};
use crate::point::Point3;
use crate::ray::Ray;
use std::cmp;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return HitResult {
                hit: false,
                hit_record: None,
            };
        }

        // find the nearest root that lies in the acceptable range.
        let mut root = (h - discriminant.sqrt()) / a;
        if root <= ray_tmin || ray_tmax < root {
            root = (h + discriminant.sqrt()) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return HitResult {
                    hit: false,
                    hit_record: None,
                };
            }
        }

        let record = HitRecord {
            point: ray.at(root),
            normal: (ray.at(root) - self.center) / self.radius,
            t: root,
        };

        HitResult {
            hit: true,
            hit_record: Some(record),
        }
    }
}
