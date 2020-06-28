use super::geom::*;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
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

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

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

// TODO: Pass in the camera along with the world
pub fn draw<H: Hittable + Sync>(width: u32, height: u32, world: &H) -> Vec<u8> {
    let image_width = f64::from(width);
    let image_height = f64::from(height);
    let samples_per_pixel = 100;
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        image_width / image_height,
    );

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

fn ray_color<H: Hittable>(rng: &mut ThreadRng, ray: &Ray, world: &H, depth: u32) -> Pixel {
    if depth == 0 {
        return Pixel(Vec3::zero());
    }
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = hit.material.scatter(ray, &hit, rng) {
            return Pixel(
                scatter.attenuation * ray_color(rng, &scatter.scattered, world, depth - 1).0,
            );
        }
        return Pixel(Vec3::zero());
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
