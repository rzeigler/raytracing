#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::*;

mod geom;
use geom::*;

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
    let hittables: Vec<Box<dyn Hittable + Send + Sync + 'static>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];
    let world = World::new(hittables);
    let content = trace::draw(width, height, &world);
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
    use rand::distributions::Uniform;
    use rand::*;
    use rand_distr::{Distribution, UnitBall};
    use rayon::prelude::*;
    use std::ops::AddAssign;
    struct Pixel(Vec3);

    impl Pixel {
        fn as_rgb(&self, samples: u32) -> [u8; 4] {
            let scale = 1.0 / f64::from(samples);

            let r = (self.0.x() * scale).sqrt();
            let g = (self.0.y() * scale).sqrt();
            let b = (self.0.z() * scale).sqrt();

            let ir = (clamp(r, 0.0, 1.0) * 255.0) as u8;
            let ig = (clamp(g, 0.0, 1.0) * 255.0) as u8;
            let ib = (clamp(b, 0.0, 1.0) * 255.0) as u8;
            [ir, ig, ib, 255]
        }
    }

    impl AddAssign for Pixel {
        fn add_assign(&mut self, rhs: Pixel) {
            self.0 += rhs.0;
        }
    }

    fn clamp(v: f64, lower: f64, upper: f64) -> f64 {
        v.min(upper).max(lower)
    }

    #[derive(Clone)]
    struct Camera {
        origin: Vec3,
        lower_left_corner: Vec3,
        horizontal: Vec3,
        vertical: Vec3,
    }

    impl Camera {
        pub fn new(width: u32, height: u32) -> Camera {
            let aspect_ratio = f64::from(width) / f64::from(height);
            let viewport_height = 2.0;
            let viewport_width = viewport_height * aspect_ratio;
            let focal_length = 1.0;

            let origin = Vec3::zero();
            let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
            let vertical = Vec3::new(0.0, viewport_height, 0.0);
            let lower_left_corner =
                origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

            Camera {
                origin,
                lower_left_corner,
                horizontal,
                vertical,
            }
        }

        pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
            Ray::new(
                self.origin,
                self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            )
        }
    }

    const MAX_DEPTH: u32 = 50;

    pub fn draw<H: Hittable + Sync>(width: u32, height: u32, world: &H) -> Vec<u8> {
        let image_width = f64::from(width);
        let image_height = f64::from(height);
        let samples_per_pixel = 100;
        let camera = Camera::new(width, height);

        // Final output of the entire representation
        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(height as usize);
        let dist = Uniform::new(0.0f64, 1.0f64);

        (0..height)
            .into_par_iter()
            // Invert because png writes from top to bottom
            .rev()
            .map(|j| {
                let mut pixels: Vec<u8> = Vec::with_capacity(width as usize * 4);
                let mut rng = thread_rng();
                for i in 0..width {
                    let mut color = Pixel(Vec3::zero());
                    for _ in 0..samples_per_pixel {
                        let u = (f64::from(i) + rng.sample(dist)) / (image_width - 1.0);
                        let v = (f64::from(j) + rng.sample(dist)) / (image_height - 1.0);
                        let ray = camera.cast_ray(u, v);
                        color += ray_color(&mut rng, &ray, world, MAX_DEPTH);
                    }
                    pixels.extend_from_slice(&color.as_rgb(samples_per_pixel));
                }
                pixels
            })
            .collect_into_vec(&mut rows);

        let mut output_buffer: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 4);
        rows.into_iter()
            .for_each(|row| output_buffer.extend_from_slice(row.as_slice()));
        output_buffer
    }

    fn ray_color<H: Hittable, R: Rng>(rng: &mut R, ray: &Ray, world: &H, depth: u32) -> Pixel {
        if depth == 0 {
            return Pixel(Vec3::zero());
        }
        if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
            let target = hit.point + hit.normal + random_in_hemisphere(rng, &hit.normal);
            return Pixel(
                0.5 * ray_color(
                    rng,
                    &Ray::new(hit.point, target - hit.point),
                    world,
                    depth - 1,
                )
                .0,
            );
        }
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y() + 1.0);
        Pixel((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }

    pub fn random_in_hemisphere<R: Rng>(rng: &mut R, normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::new_raw(UnitBall.sample(rng));
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere.flip()
        }
    }
}
