use crate::point::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct HitResult {
    pub hit: bool,
    pub hit_record: Option<HitRecord>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult;

    fn calculate_face_normal(&self, ray: &Ray, outward_normal: Vec3) -> Vec3;
}
