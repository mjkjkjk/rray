use clap::{command, Parser};
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use math::rng::{random_double, random_double_range};
use point::Point3;
use scene::camera::Camera;
use sphere::Sphere;
use std::fs::File;
use vec3::Vec3;

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

    #[arg(short, long)]
    depth: Option<u32>,

    #[arg(short, long)]
    samples: Option<u32>,

    #[arg(short, long)]
    width: Option<u32>,

    #[arg(short, long)]
    height: Option<u32>,
}

fn initialize_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));

    for i in -9..9 {
        for j in -9..9 {
            let radius = random_double_range(0.0, 0.3);
            let choose_mat = random_double();
            let center = Point3::new(
                i as f64 + 0.9 * random_double(),
                radius,
                j as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    Box::new(Metal::new(albedo, 0.5 * random_double()))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                let sphere = Sphere::new(center, radius, sphere_material);
                world.push(Box::new(sphere));
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let image_width: u32 = args.width.unwrap_or(700);
    let image_height: u32 = args.height.unwrap_or(400);

    let mut file = File::create(&args.output_file)?;

    let depth = args.depth.unwrap_or(4);
    let samples = args.samples.unwrap_or(16);

    let world = initialize_world();

    let camera = Camera::new(
        image_width,
        image_height,
        samples,
        depth,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    camera.render(&world, &mut file);

    println!("Done.");

    Ok(())
}
