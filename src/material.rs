use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{reflect, Vec3},
};

pub type ScatterResult = Option<(Color, Ray)>;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let randomized_direction = hit_record.normal + Vec3::random_unit_vector();
        let scatter_direction = if randomized_direction.near_zero() {
            hit_record.normal
        } else {
            randomized_direction
        };
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let reflected = reflect(ray.direction(), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        Some((self.albedo, scattered))
    }
}
