mod colors;
mod hit;
mod material;
mod objects;
mod ray;
mod sphere;
mod vector3;

extern crate anyhow;

use crate::material::Material;
use crate::objects::Object;
use crate::sphere::Sphere;
use anyhow::Context;
use clap::Parser;
use colors::write_color;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar};
use ray::Ray;
use std::{
    fs,
    io::{Seek, Write},
    path::PathBuf,
};
use vector3::Vector3 as Color;
use vector3::Vector3;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 450;

const VIEWPORT_WIDTH: f64 = 3.5;
const FOCAL_LENGTH: f64 = 1.0;

const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH / (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

#[derive(Parser)]
#[command(author = "<utbryceh@gmail.com>")]
#[command(version, about, long_about = None)]
struct Cli {
    file: PathBuf,
}

fn ray_color(ray: Ray) -> Color {
    let sphere = Sphere::new(
        Vector3::new(0.0, 0.0, -6.0),
        3.0,
        Material::new(Color::new(0.0, 0.0, 0.0)),
    );

    match sphere.get_hit(&ray) {
        None => {
            let t = 0.5 * (ray.direction().y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
        Some(hit) => {
            0.5 * Color::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            )
        }
    }
}

fn main() -> anyhow::Result<()> {
    // initialize progress bar
    init_progress_bar(IMAGE_HEIGHT as usize);

    // get command-line arguments
    let args = Cli::parse();

    // open the file
    let mut output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args.file)
        .with_context(|| format!("Issue opening file `{}`", args.file.display()))?;

    // clear file
    output_file
        .set_len(0)
        .with_context(|| format!("Issue modifying file `{}`", args.file.display()))?;
    output_file
        .rewind()
        .with_context(|| "Issue seeking beginning of file after clearing")?;

    // output info
    output_file
        .write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .with_context(|| format!("Issue writing to file `{}`", args.file.display()))?;

    // get some helpful vectors
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = Vector3::default()
        - horizontal / 2.0
        - vertical / 2.0
        - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let width_ratio = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let height_ratio = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                Vector3::default(),
                lower_left_corner + width_ratio * horizontal + height_ratio * vertical
                    - Vector3::default(),
            );
            let pixel_color = ray_color(ray);

            write_color(&mut output_file, &pixel_color)?;
        }

        inc_progress_bar();
    }

    finalize_progress_bar();

    Ok(())
}
