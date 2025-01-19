use std::fs::File;
use std::io::Write;

use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::math::interval::Interval;
use crate::math::rng::random_double_range;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        _center: Point3,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let center = Point3::new(0.0, 0.0, 0.0);

        // determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList, mut file: &mut File) {
        // header
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                write_color(&mut file, pixel_color * self.pixel_samples_scale).unwrap();
            }
        }
    }

    fn ray_color(ray: &Ray, max_depth: u32, world: &HittableList) -> Color {
        if max_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_result) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            let direction = Vec3::random_on_hemisphere(hit_result.normal);
            0.5 * Self::ray_color(&Ray::new(hit_result.point, direction), max_depth - 1, world)
        } else {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let origin = self.center;
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            random_double_range(-0.5, 0.5),
            random_double_range(-0.5, 0.5),
            0.0,
        )
    }
}
