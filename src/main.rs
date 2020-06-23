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
    let content = draw(width, height);
    write_png(width, height, &content, out_path)
}

fn draw(width: u32, height: u32) -> Vec<u8> {
    let image_width = width as f64;
    let image_height = height as f64;
    let size = width as usize * height as usize * 4;
    let mut result = Vec::with_capacity(size);
    (0..height)
        .rev() // most positive x written first in png format
        .flat_map(|j| {
            (0..width).map(move |i| {
                let r = (i as f64) / (image_width - 1.0);
                let g = (j as f64) / (image_height - 1.0);
                let b = 0.25f64;
                Pixel::new_from_f64(r, g, b, 1.0)
            })
        })
        .for_each(|pixel| result.extend_from_slice(&pixel.data));
    result
}

fn write_png(width: u32, height: u32, data: &Vec<u8>, out_path: &Path) -> Result<()> {
    // Do PNG things
    let file = File::create(out_path)
        .with_context(|| format!("failed to open output path: {:?}", out_path))?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().context("failed to write header")?;
    writer
        .write_image_data(&data)
        .context("failed to write data")
}
