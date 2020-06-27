#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod geom;

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
    let content = trace::draw(width, height);
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

mod trace {
    use super::geom::*;

    struct Pixel(Vec3);

    impl Pixel {
        fn as_rgb(&self) -> [u8; 4] {
            let ir = (self.0.x() * 255.0) as u8;
            let ig = (self.0.y() * 255.0) as u8;
            let ib = (self.0.z() * 255.0) as u8;
            [ir, ig, ib, 255]
        }
    }

    pub fn draw(width: u32, height: u32) -> Vec<u8> {
        let size = width as usize * height as usize * 4;
        let image_width = f64::from(width);
        let image_height = f64::from(height);
        let aspect_ratio = image_width / image_height;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Vec3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        let mut result = Vec::with_capacity(size);
        (0..height)
            .rev() // most positive y written first in png format
            .flat_map(|j| {
                (0..width).map(move |i| {
                    let u = f64::from(i) / (image_width - 1.0);
                    let v = f64::from(j) / (image_height - 1.0);
                    let ray = Ray::new(
                        origin,
                        lower_left_corner + u * horizontal + v * vertical - origin,
                    );
                    ray_color(&ray).as_rgb()
                })
            })
            .for_each(|pixel| result.extend_from_slice(&pixel));
        result
    }

    fn ray_color(ray: &Ray) -> Pixel {
        if Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5).hit(ray) {
            return Pixel(Vec3::new(1.0, 0.0, 0.0));
        }
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y() + 1.0);
        Pixel((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }
}
