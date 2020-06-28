#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use rand::distributions::*;
use rand::*;
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

    let c1 = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let o1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &c1);

    let c2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let o2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &c2);

    let c3 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3);
    let o3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &c3);

    // Hollow glass sphere
    let glass = Dielectric::new(1.5);
    let outer_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &glass);
    let inner_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, &glass);
    let glass_sphere: Vec<&(dyn Hittable + Send + Sync)> = vec![&outer_sphere, &inner_sphere];
    let glass_world = World::new(glass_sphere);

    let hittables: Vec<&(dyn Hittable + Send + Sync)> = vec![&o1, &o2, &o3, &glass_world];

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
    );

    /*
     * Build the world... this is kind of a bad interface because I tried to be clever with refs
     */
    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let ground = Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    let mut rng = thread_rng();
    let random_double = Uniform::new(0.0, 1.0);
    let fuzz_dist = Uniform::new(0.0, 0.5);

    let mut material_vec: Vec<Box<dyn Material>> = Vec::new();
    // Temporary storage so that we can wire up materials afterwards
    let mut sphere_descr: Vec<(Vec3, f64)> = Vec::new();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double.sample(&mut rng);
            let center = Vec3::new(
                f64::from(a) + 0.9 * random_double.sample(&mut rng),
                0.2,
                f64::from(b) + 0.9 * random_double.sample(&mut rng),
            );

            let dist_05_1 = Uniform::new(0.5, 1.0);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Box<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Vec3::random(&mut rng) * Vec3::random(&mut rng);
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_dist(&mut rng, &dist_05_1);
                    let fuzz = fuzz_dist.sample(&mut rng);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    Box::new(Dielectric::new(1.5))
                };
                material_vec.push(mat);
                sphere_descr.push((center, 0.2f64));
            }
        }
    }

    material_vec.push(Box::new(Dielectric::new(1.5)));
    sphere_descr.push((Vec3::new(0.0, 1.0, 0.0), 1.0));

    material_vec.push(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
    sphere_descr.push((Vec3::new(-4.0, 1.0, 0.0), 1.0));

    material_vec.push(Box::new(Metal::new(Vec3::new(0.7, 0., 0.5), 0.0)));
    sphere_descr.push((Vec3::new(4.0, 1.0, 0.0), 1.0));

    let material_vec = material_vec;
    let sphere_vec: Vec<Box<Sphere>> = sphere_descr
        .iter()
        .zip(material_vec.iter())
        .map(|(desc, mat)| Box::new(Sphere::new(desc.0, desc.1, mat.as_ref())))
        .collect();

    let mut hittables: Vec<&(dyn Hittable + Send + Sync)> = Vec::new();
    for sphere in sphere_vec.iter() {
        hittables.push(sphere.as_ref());
    }
    hittables.push(ground.as_ref());

    let world = World::new(hittables);

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
