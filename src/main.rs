use clap::{command, Parser};
use color::{write_color, Color};
use hittable::Hittable;
use hittable_list::HittableList;
use math::interval::Interval;
use point::Point3;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

mod color;
mod hittable;
mod hittable_list;
mod math;
mod point;
mod ray;
mod sphere;
mod vec3;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_file: String,
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

fn main() -> std::io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = image_width as f64 * viewport_height / image_height as f64;
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let args = Args::parse();

    let mut file = File::create(&args.output_file)?;

    // header
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

    let mut world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(4.0, 2.0, -6.0), 0.3)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // pixel data
    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray = Ray::new(camera_center, pixel_center - camera_center);

            let hit_result = world.hit(&ray, Interval::new(0.0, f64::INFINITY));
            let pixel_color = if let Some(hit_result) = hit_result {
                let normal = hit_result.normal;
                0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0)
            } else {
                ray_color(&ray, &world)
            };

            write_color(&mut file, pixel_color)?;
        }
    }

    println!("Done.");

    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

    println!("{}", ray.origin());
    println!("{}", ray.direction());
    println!("{}", ray.at(0.5));

    Ok(())
}
