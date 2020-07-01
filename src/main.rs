#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use rand::distributions::*;
use rand::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

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

    let image_width = f64::from(width);
    let image_height = f64::from(height);
    let aspect_ratio = image_width / image_height;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = draw::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = create_large();

    let content = draw::draw(width, height, &camera, &world);
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

fn create_large() -> Box<dyn Hittable + Send + Sync> {
    let mut rng = thread_rng();
    let random_double = Uniform::new(0.0, 1.0);
    let fuzz_dist = Uniform::new(0.0, 0.5);

    let mut objects: Vec<Box<dyn Hittable + Sync + Send>> = Vec::new();
    /*
     * Build the world... this is kind of a bad interface because I tried to be clever with refs
     */
    let ground: Box<dyn Hittable + Sync + Send> = Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ));

    objects.push(ground);

    let spheres = (-11..11)
        .flat_map(|a| (-11..11).map(move |b| (a, b)))
        .map(|(a, b)| {
            let choose_mat = random_double.sample(&mut rng);
            let center = Vec3::new(
                f64::from(a) + 0.9 * random_double.sample(&mut rng),
                0.2,
                f64::from(b) + 0.9 * random_double.sample(&mut rng),
            );

            let dist_05_1 = Uniform::new(0.5, 1.0);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere: Box<dyn Hittable + Send + Sync> = if choose_mat < 0.8 {
                    /*
                                        auto center2 = center + vec3(0, random_double(0,.5), 0);
                    world.add(make_shared<moving_sphere>(
                        center, center2, 0.0, 1.0, 0.2, sphere_material));
                    */
                    let center2 = center + Vec3::new(0.0, rng.sample(&fuzz_dist), 0.0);
                    let albedo = Vec3::random_dist(&mut rng, &random_double)
                        * Vec3::random_dist(&mut rng, &random_double);
                    let mat = Arc::new(Lambertian::new(albedo));
                    Box::new(Sphere::new_moving(
                        Timed::new(center, 0.0),
                        Timed::new(center2, 1.0),
                        0.2f64,
                        mat,
                    ))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_dist(&mut rng, &dist_05_1);
                    let fuzz = fuzz_dist.sample(&mut rng);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    Box::new(Sphere::new(center, 0.2f64, mat))
                } else {
                    let mat = Arc::new(Dielectric::new(1.5));
                    Box::new(Sphere::new(center, 0.2f64, mat))
                };
                Some(sphere)
            } else {
                None
            }
        })
        .flatten();

    objects.extend(spheres);

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    bvh_split_hittables(&mut rng, objects, 0.0, 1.0)
}
