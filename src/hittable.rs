use crate::material::ScatterResult;
use crate::math::interval::Interval;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub scatter_result: Option<ScatterResult>,
}

impl HitRecord {
    pub fn calculate_face_normal(ray: &Ray, outward_normal: Vec3) -> Vec3 {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
