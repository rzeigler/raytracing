#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use rand::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod data;

use data::*;

const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = 800;

fn main() -> Result<()> {
    let matches = App::new("raytracing")
        .version("chapter-1")
        .author("Ryan Zeigler <zeiglerr@gmail.com>")
        .about("Implementation of the raytracing from https://raytracing.github.io/ to learn Rust")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("The width in pixels of the generated image"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .help("The height in pixels of the generated image"),
        )
        .arg(
            Arg::with_name("out")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("The path to write output too"),
        )
        .get_matches();
    let width = if matches.is_present("width") {
        value_t!(matches, "width", u32).with_context(|| "invalid width")?
    } else {
        IMAGE_WIDTH
    };
    let height = if matches.is_present("height") {
        value_t!(matches, "height", u32).with_context(|| "invalid height")?
    } else {
        IMAGE_HEIGHT
    };
    let out_path = Path::new(matches.value_of("out").unwrap());
    run(width, height, &out_path)
}

fn run(width: u32, height: u32, out_path: &Path) -> Result<()> {
    // Draw the image
    let mut canvas = Canvas::new(width, height);
    draw(&mut canvas);
    write_png(&canvas, &out_path)
}

fn draw(canvas: &mut Canvas) {
    let max_depth = 50u32;
    let samples_per_pixel = 100usize;
    let image_width = f64::from(canvas.width);
    let image_height = f64::from(canvas.height);
    let camera = Camera::new(image_width, image_height);

    let world: Vec<Box<dyn CanHit + Sync>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // let mut rng = rand::thread_rng();

    // for j in (0..canvas.height).rev() {
    //     for i in 0..canvas.width {
    //         let mut color = Vec3::new(0.0, 0.0, 0.0);
    //         for _ in 0..samples_per_pixel {
    //             let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1.0);
    //             let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1.0);
    //             let ray = camera.cast(u, v);
    //             color += ray_color(&ray, &world, &mut rng, max_depth);
    //         }
    //         let pixel = canvas.at_mut(i, j);
    //         pixel.set_color_sampled(&color, samples_per_pixel);
    //         pixel.set_alpha(255);
    //     }
    // }

    canvas
        .pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(j, row)| {
            for (i, pixel) in row.iter_mut().enumerate() {
                let mut rng = rand::thread_rng();
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1.0);
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1.0);
                    let ray = camera.cast(u, v);
                    color += ray_color(&ray, &world, &mut rng, max_depth);
                }
                pixel.set_color_sampled(&color, samples_per_pixel);
                pixel.set_alpha(255);
            }
        });
}

fn ray_color<R: Rng>(r: &Ray, world: &[Box<dyn CanHit + Sync>], rng: &mut R, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, std::f64::INFINITY) {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        } else {
            let target = hit.point + hit.normal + Vec3::random_in_sphere(rng);
            return 0.5
                * ray_color(
                    &Ray::new(hit.point, target - hit.point),
                    world,
                    rng,
                    depth - 1,
                );
        }
    }
    let unit_direction = r.direction.unit();
    let t = 0.5f64 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_png(canvas: &Canvas, out_path: &Path) -> Result<()> {
    // Do PNG things
    let file = File::create(out_path)
        .with_context(|| format!("failed to open output path: {:?}", out_path))?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, canvas.width, canvas.height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().context("failed to write header")?;
    writer
        .write_image_data(&canvas.rgba_bytes()[..])
        .context("failed to write data")
}
