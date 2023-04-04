mod camera;
mod constants;
mod hit;
mod material;
mod objects;
mod quaternion;
mod random;
mod ray;
mod sphere;
mod vector3;

extern crate anyhow;

use crate::camera::Camera;
use crate::constants::{
    ANTIALIASING_STRENGTH, ASPECT_RATIO, HORIZONTAL_FOV_DEGREES, IMAGE_HEIGHT, IMAGE_WIDTH,
    RAYS_PER_PIXEL,
};
use crate::material::Material;
use crate::objects::Object;
use crate::quaternion::Quaternion;
use crate::sphere::Sphere;
use anyhow::Context;
use clap::Parser;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar};
use rand::rngs::ThreadRng;
use rand::Rng;
use ray::Ray;
use std::fs;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use vector3::Vector3 as Color;
use vector3::Vector3;

pub fn write_color(file: &mut File, color: &Color) -> anyhow::Result<()> {
    let integer_red = (255.999 * color[0]) as u8;
    let integer_green = (255.999 * color[1]) as u8;
    let integer_blue = (255.999 * color[2]) as u8;

    // write rgb value to file
    file.write(format!("{} {} {}\n", integer_red, integer_green, integer_blue).as_bytes())
        .with_context(|| "Issue writing to file".to_string())?;

    Ok(())
}

#[derive(Parser)]
#[command(author = "<utbryceh@gmail.com>")]
#[command(version, about, long_about = None)]
struct Cli {
    file: PathBuf,
}

fn ray_color(ray: Ray, rng: &mut ThreadRng) -> Color {
    let objects: [&dyn Object; 7] = [
        &Sphere::new(
            Vector3::new(16.0, 0.0, 80.0),
            8.0,
            Material::new(
                Color::new(0.0, 0.0, 0.0),
                0.0,
                Color::new(0.93, 0.95, 0.2),
                8.0,
            ),
        ),
        &Sphere::new(
            Vector3::new(0.0, 0.0, -31.0),
            30.0,
            Material::new_lightless(Color::new(0.8, 0.2, 0.2), 0.0),
        ),
        &Sphere::new(
            Vector3::new(9.0, -10.0, -1.0),
            2.0,
            Material::new_lightless(Color::new(0.2, 0.8, 0.2), 0.0),
        ),
        &Sphere::new(
            Vector3::new(11.0, -5.0, -1.0),
            2.0,
            Material::new_lightless(Color::new(0.4, 0.4, 0.4), 0.5),
        ),
        &Sphere::new(
            Vector3::new(12.0, 0.0, -1.0),
            2.0,
            Material::new_lightless(Color::new(0.6, 0.0, 0.6), 1.0),
        ),
        &Sphere::new(
            Vector3::new(11.0, 5.0, -1.0),
            2.0,
            Material::new_lightless(Color::new(0.4, 0.4, 0.4), 0.5),
        ),
        &Sphere::new(
            Vector3::new(9.0, 10.0, -1.0),
            2.0,
            Material::new_lightless(Color::new(0.2, 0.8, 0.2), 0.0),
        ),
    ];

    ray.trace(&objects, rng)
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

    // output ppm info
    output_file
        .write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .with_context(|| format!("Issue writing to file `{}`", args.file.display()))?;

    // create a camera
    let camera = Camera::new(
        Vector3::new(-4.0, 0.0, 8.0),
        Quaternion::new(Vector3::new(0.0, 1.0, 0.0), 40.0),
        HORIZONTAL_FOV_DEGREES,
        ASPECT_RATIO,
    );

    // multithreading handles
    let mut handles: Vec<JoinHandle<Vec<Color>>> = Vec::new();

    for pixel_y in (0..IMAGE_HEIGHT).rev() {
        let pixel_y_clone = pixel_y;
        let camera_clone = camera;

        handles.push(thread::spawn(move || {
            // used to store the pixel colors for the row
            let mut pixel_colors: Vec<Color> = Vec::new();

            // used for super sampling
            let mut rng = rand::thread_rng();

            for pixel_x in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::default();

                for _ in 0..RAYS_PER_PIXEL {
                    let width_ratio = pixel_x as f64 / (IMAGE_WIDTH - 1) as f64
                        + rng.gen::<f64>() * ANTIALIASING_STRENGTH / IMAGE_WIDTH as f64;
                    let height_ratio = pixel_y_clone as f64 / (IMAGE_HEIGHT - 1) as f64
                        + rng.gen::<f64>() * ANTIALIASING_STRENGTH / IMAGE_HEIGHT as f64;

                    let ray = camera_clone.get_ray(width_ratio, height_ratio);
                    let sample_color = ray_color(ray, &mut rng);

                    pixel_color += sample_color;
                }

                pixel_color /= RAYS_PER_PIXEL as f64;
                pixel_colors.push(pixel_color);
            }

            inc_progress_bar();
            pixel_colors
        }));
    }

    for handle in handles {
        let colors = handle.join().unwrap();

        for color in colors {
            write_color(&mut output_file, &color).with_context(|| "Issue writing to file")?;
        }
    }

    finalize_progress_bar();

    Ok(())
}
