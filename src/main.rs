use clap::{command, Parser};
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use point::Point3;
use scene::camera::Camera;
use sphere::Sphere;
use std::fs::File;

mod color;
mod hittable;
mod hittable_list;
mod material;
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

fn main() -> std::io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 700;

    let args = Args::parse();

    let mut file = File::create(&args.output_file)?;

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.6, 0.2));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(material_ground))),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(material_center))),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left))),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right))),
    ];

    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(image_width, aspect_ratio, camera_center, 16, 4);
    camera.render(&world, &mut file);

    println!("Done.");

    Ok(())
}
