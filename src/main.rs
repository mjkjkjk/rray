use clap::{command, Parser};
use color::{write_color, Color};
use point::Point3;
use ray::Ray;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

mod color;
mod point;
mod ray;
mod vec3;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_file: String,
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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

    // pixel data
    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray = Ray::new(camera_center, pixel_center - camera_center);
            let pixel_color = ray_color(&ray);

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
