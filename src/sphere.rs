use crate::hittable::{HitRecord, Hittable};
use crate::material::{Material, ScatterResult};
use crate::math::interval::Interval;
use crate::point::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let normal = HitRecord::calculate_face_normal(ray, outward_normal);
        let front_face = ray.direction().dot(outward_normal) < 0.0;

        let hit_record = HitRecord {
            point,
            normal,
            t: root,
            front_face,
            scatter_result: None,
        };

        let scatter_result = self.material.scatter(ray, &hit_record);

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
            scatter_result,
        })
    }
}
