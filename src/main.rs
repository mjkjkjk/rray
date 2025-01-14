use clap::{command, Parser};
use color::Color;
use point::Point3;
use ray::Ray;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

mod color;
mod point;
mod ray;
mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut file = File::create(&args.output_file)?;

    // header
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    // pixel data
    for j in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel_color = Color::new(r, g, b);

            writeln!(
                file,
                "{} {} {}",
                (255.999 * pixel_color.x()) as u8,
                (255.999 * pixel_color.y()) as u8,
                (255.999 * pixel_color.z()) as u8
            )?;
        }
    }

    println!("Done.");

    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

    println!("{}", ray.origin());
    println!("{}", ray.direction());
    println!("{}", ray.at(0.5));

    Ok(())
}
