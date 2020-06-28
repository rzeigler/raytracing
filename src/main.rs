#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod geom;
use geom::*;
mod draw;

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

    let c1 = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let o1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &c1);

    let c2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let o2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &c2);

    let c3 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3);
    let o3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &c3);

    let c4 = Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0);
    let o4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &c4);

    let hittables: Vec<&(dyn Hittable + Send + Sync)> = vec![&o1, &o2, &o3, &o4];

    // let hittables: Vec<Box<dyn Hittable + Send + Sync + 'static>> = vec![
    //     Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5), &color1),
    //     Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    // ];
    let world = World::new(hittables);
    let content = draw::draw(width, height, &world);
    write_png(width, height, &content, out_path)
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
