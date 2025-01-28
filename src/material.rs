use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = Vec3::reflect(ray.direction().unit_vector(), hit_record.normal);
        let scattered_direction = reflected + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_record.point, scattered_direction.unit_vector());

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit_vector();
        let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
        let scattered = Ray::new(hit_record.point, refracted);

        Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}
