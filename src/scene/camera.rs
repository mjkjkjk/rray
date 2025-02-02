use std::fs::File;
use std::io::Write;

use rayon::prelude::*;

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
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        println!("depth: {}", max_depth);
        println!("samples: {}", samples_per_pixel);
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let center = lookfrom;

        // determine viewport dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList, file: &mut File) {
        // header
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        let pixels: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color += Self::ray_color(&r, self.max_depth, world);
                        }
                        pixel_color * self.pixel_samples_scale
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // all pixels at once
        for pixel_color in pixels {
            write_color(file, pixel_color).unwrap();
        }
    }

    fn ray_color(ray: &Ray, max_depth: u32, world: &HittableList) -> Color {
        if max_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_result) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter_result) = hit_result.scatter_result {
                return scatter_result.attenuation
                    * Self::ray_color(&scatter_result.scattered, max_depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
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

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}
