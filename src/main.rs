use clap::{command, Parser};
use hittable_list::HittableList;
use point::Point3;
use scene::camera::Camera;
use sphere::Sphere;
use std::fs::File;

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

fn main() -> std::io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 600;

    let args = Args::parse();

    let mut file = File::create(&args.output_file)?;

    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(4.0, 2.0, -6.0), 0.3)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(image_width, aspect_ratio, camera_center, 16, 4);
    camera.render(&world, &mut file);

    println!("Done.");

    Ok(())
}
