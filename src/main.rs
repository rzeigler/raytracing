#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
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
    let mut canvas = Canvas::new(width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            let r = f64::from(i as u32) / f64::from(width - 1);
            let g = f64::from(j) / f64::from(height - 1);
            let b = 0.25f64;
            let pixel = canvas.at_mut(i, j);
            pixel.set_rgb((255.99 * r) as u8, (255.99 * g) as u8, (255.99 * b) as u8);
            pixel.set_alpha(255);
        }
    }
    let file = File::create(out_path)
        .with_context(|| format!("failed to open output path: {:?}", out_path))?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().context("failed to write header")?;
    writer
        .write_image_data(&canvas.rgba_bytes()[..])
        .context("failed to write data")?;
    Ok(())
}
