use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{reflect, Vec3},
};

#[derive(Clone)]
pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let randomized_direction = hit_record.normal + Vec3::random_unit_vector();
        let scatter_direction = if randomized_direction.near_zero() {
            hit_record.normal
        } else {
            randomized_direction
        };
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(ray.direction(), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}
