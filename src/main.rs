use clap::{command, Parser};
use color::{write_color, Color};
use hittable::Hittable;
use hittable_list::HittableList;
use math::interval::Interval;
use point::Point3;
use ray::Ray;
use scene::camera::Camera;
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
mod scene;
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

    let args = Args::parse();

    let mut file = File::create(&args.output_file)?;

    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(4.0, 2.0, -6.0), 0.3)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(image_width, aspect_ratio, camera_center);
    camera.render(&world, &mut file);

    println!("Done.");

    Ok(())
}
