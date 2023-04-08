mod aabb;
mod r#box;
mod camera;
mod constants;
mod hit;
mod material;
mod objects;
mod plane;
mod quaternion;
mod random;
mod ray;
mod sphere;
mod vector3;

extern crate anyhow;

use crate::camera::Camera;
use crate::constants::{
    ANTIALIASING_STRENGTH, ASPECT_RATIO, HORIZONTAL_FOV_DEGREES, IMAGE_HEIGHT, IMAGE_WIDTH,
    RAYS_PER_PIXEL_PER_THREAD, THREADS,
};
use crate::material::Material;
use crate::objects::Object;
use crate::plane::Plane;
use crate::quaternion::Quaternion;
use crate::r#box::BoxObject;
use crate::sphere::Sphere;
use crate::vector3::Vector3;
use anyhow::Context;
use clap::Parser;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar};
use rand::Rng;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::{fs, iter};
use vector3::Vector3 as Color;

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

fn main() -> anyhow::Result<()> {
    // initialize progress bar
    init_progress_bar(THREADS);

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

    // multithreading handles
    let mut handles: Vec<JoinHandle<Vec<Color>>> = Vec::new();

    // world :)
    let objects: [Arc<dyn Object + Send + Sync>; 4] = [
        Arc::new(Sphere::new(
            Vector3::new(-15.0, 0.0, 70.0),
            45.0,
            Material::new(
                Color::new(0.0, 0.0, 0.0),
                0.0,
                Color::new(0.93, 0.95, 0.2),
                2.5,
            ),
        )),
        Arc::new(BoxObject::new(
            Vector3::new(20.0, -10.0, 0.5),
            7.0,
            7.0,
            7.0,
            Quaternion::new_from_angles(45.0, 0.0, 45.0),
            Material::new_lightless(Color::new(0.8, 0.45, 0.45), 0.0),
        )),
        Arc::new(Sphere::new(
            Vector3::new(10.0, 7.0, 0.5),
            4.0,
            Material::new_lightless(Color::new(0.45, 0.45, 0.8), 0.0),
        )),
        Arc::new(Plane::new(
            Vector3::new(0.0, 0.0, -10.0),
            Vector3::new(-0.12, 0.0, 1.0),
            Material::new_lightless(Color::new(0.45, 0.8, 0.45), 0.0),
        )),
    ];

    // used for rendering
    let camera = Camera::new(
        Vector3::new(-10.0, 0.0, 0.0),
        Quaternion::new_from_angles(0.0, 0.0, 0.0),
        HORIZONTAL_FOV_DEGREES,
        ASPECT_RATIO,
    );

    for _ in 0..THREADS {
        let objects_clone = objects.clone();

        handles.push(thread::spawn(move || {
            // used to store the pixel colors for the row
            let mut thread_pixel_colors: Vec<Color> = Vec::new();

            // used for super sampling
            let mut rng = rand::thread_rng();

            for pixel_y in (0..IMAGE_HEIGHT).rev() {
                for pixel_x in 0..IMAGE_WIDTH {
                    let mut pixel_color = Color::default();

                    for _ in 0..RAYS_PER_PIXEL_PER_THREAD {
                        let width_ratio = pixel_x as f64 / (IMAGE_WIDTH - 1) as f64
                            + rng.gen::<f64>() * ANTIALIASING_STRENGTH / IMAGE_WIDTH as f64;
                        let height_ratio = pixel_y as f64 / (IMAGE_HEIGHT - 1) as f64
                            + rng.gen::<f64>() * ANTIALIASING_STRENGTH / IMAGE_HEIGHT as f64;

                        let ray = camera.get_ray(width_ratio, height_ratio);
                        let sample_color = ray.trace(&objects_clone, &mut rng);

                        pixel_color += sample_color;
                    }

                    pixel_color /= RAYS_PER_PIXEL_PER_THREAD as f64;
                    thread_pixel_colors.push(pixel_color);
                }
            }

            inc_progress_bar();
            thread_pixel_colors
        }));
    }

    // where we store the pixel colors
    let mut pixel_colors: Vec<Color> = iter::repeat(Color::default())
        .take(IMAGE_HEIGHT as usize * IMAGE_WIDTH as usize)
        .collect();

    for handle in handles {
        let thread_colors = handle.join().unwrap();

        for (i, color) in thread_colors.into_iter().enumerate() {
            pixel_colors[i] += color;
        }
    }

    finalize_progress_bar();

    for color in pixel_colors {
        let averaged_color = color / THREADS as f64;
        write_color(&mut output_file, &averaged_color)?;
    }

    Ok(())
}
