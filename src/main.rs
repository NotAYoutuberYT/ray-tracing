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
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::{fs, iter};
use std::ops::Deref;
use vector3::Vector3 as Color;

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
    let output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args.file)
        .with_context(|| format!("Issue opening file `{}`", args.file.display()))?;
    let file_writer = BufWriter::new(output_file);

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

    // get the png output data
    let mut image_data = Box::new([0; IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize * 3]);
    for (i, color) in pixel_colors.iter().enumerate() {
        let averaged_color = *color / THREADS as f64;

        let red = (255.999 * averaged_color[0]) as u8;
        let green = (255.999 * averaged_color[1]) as u8;
        let blue = (255.999 * averaged_color[2]) as u8;

        image_data[i * 3] = red;
        image_data[i * 3 + 1] = green;
        image_data[i * 3 + 2] = blue;
    }

    // get an encoder for the png
    let mut png_encoder = png::Encoder::new(file_writer, IMAGE_WIDTH, IMAGE_HEIGHT);
    png_encoder.set_color(png::ColorType::Rgb);
    png_encoder.set_depth(png::BitDepth::Eight);

    // write the data with the encoder
    png_encoder
        .write_header()
        .with_context(|| "Failed to get a png write header")?
        .write_image_data(image_data.deref())
        .with_context(|| "Failed to write to file")?;

    finalize_progress_bar();

    Ok(())
}
