use std::fs::File;
use std::io::Write;

use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::math::interval::Interval;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    aspect_ratio: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64, center: Point3) -> Self {
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
            aspect_ratio,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList, mut file: &mut File) {
        // header
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&ray, world);

                write_color(&mut file, pixel_color);
            }
        }
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        if let Some(hit_result) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            let normal = hit_result.normal;
            0.5 * Color::from_vec3(normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
