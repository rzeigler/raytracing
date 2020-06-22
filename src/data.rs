use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * std::f64::consts::PI / 180.0
}

#[derive(Clone)]
pub struct Pixel {
    data: [u8; 4],
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { data: [0, 0, 0, 0] }
    }

    pub fn red(&self) -> u8 {
        self.data[0]
    }

    pub fn set_red(&mut self, r: u8) {
        self.data[0] = r;
    }

    pub fn blue(&self) -> u8 {
        self.data[1]
    }

    pub fn set_blue(&mut self, b: u8) {
        self.data[1] = b;
    }

    pub fn green(&self) -> u8 {
        self.data[2]
    }

    pub fn set_green(&mut self, g: u8) {
        self.data[2] = g;
    }

    pub fn alpha(&self) -> u8 {
        self.data[3]
    }

    #[allow(dead_code)]
    pub fn set_alpha(&mut self, a: u8) {
        self.data[3] = a;
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.set_red(r);
        self.set_green(g);
        self.set_blue(b);
    }

    pub fn set_color(&mut self, color: &Vec3) {
        self.set_red((color.x * 255.99) as u8);
        self.set_green((color.y * 255.99) as u8);
        self.set_blue((color.z * 255.99) as u8);
    }

    pub fn set_color_sampled(&mut self, color: &Vec3, samples: usize) {
        let scale = 1.0f64 / (samples as f64);

        let r = color.x * scale;
        let g = color.y * scale;
        let b = color.z * scale;

        self.set_color(&Vec3::new(r, g, b));
    }
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    // Pixels in form [y][x]
    pub pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![Pixel::new(); width as usize]; height as usize],
        }
    }

    pub fn at(&self, x: u32, y: u32) -> &Pixel {
        &self.pixels[y as usize][x as usize]
    }

    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.pixels[y as usize][x as usize]
    }

    pub fn rgba_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![0 as u8; (self.width * self.height) as usize * 4];
        let mut offset = 0;
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let p = self.at(j, i);
                buffer[offset] = p.red();
                buffer[offset + 1] = p.green();
                buffer[offset + 2] = p.blue();
                buffer[offset + 3] = p.alpha();
                offset += 4
            }
        }
        buffer
    }
}

static FOCAL_LENGTH: f64 = 1.0;

pub fn origin() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

pub struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(width: f64, height: f64) -> Camera {
        let aspect_ratio = width / height;
        let viewport_height = 2f64;
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin() - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera {
            viewport_height,
            viewport_width,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn cast(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            origin(),
            self.lower_left_corner + u * self.horizontal + v * self.vertical - origin(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            x: e0,
            y: e1,
            z: e2,
        }
    }

    pub fn add_vec(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn sub_vec(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    pub fn mult_vec(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    pub fn mult_scalar(&self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }

    pub fn div_scalar(&self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * self.z - rhs.z * rhs.y,
            self.z * self.y - rhs.y * rhs.z,
            self.x * self.y - rhs.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Vec3 {
        self.div_scalar(self.length())
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random<R: Rng>(rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_clamped<R: Rng>(rng: &mut R, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_sphere<R: Rng>(rng: &mut R) -> Vec3 {
        loop {
            let v = Vec3::random_clamped(rng, -1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }
}

// Implementation of fun things like add traits so that we can more easily
// mirror the raytracing code
impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(point: Vec3, t: f64, ray: &Ray, outward_normal: &Vec3) -> Hit {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let real_normal = if front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
        Hit {
            point,
            normal: real_normal,
            t,
            front_face,
        }
    }
}

pub trait CanHit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl CanHit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t1 = (-half_b - root) / a;
            let found1 = if t1 < t_max && t1 > t_min {
                let point = ray.at(t1);
                let outward_normal = (point - self.center) / self.radius;
                Some(Hit::new(point, t1, ray, &outward_normal))
            } else {
                None
            };
            let t2 = (-half_b + root) / a;
            let found2 = if t2 < t_max && t2 > t_min {
                let point = ray.at(t2);
                let outward_normal = (point - self.center) / self.radius;
                Some(Hit::new(point, t2, ray, &outward_normal))
            } else {
                None
            };
            // We are wastefull because we compute both... oh well
            return found2.or(found1);
        }
        None
    }
}

impl CanHit for &[Box<dyn CanHit + Sync>] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut maybe_hit: Option<Hit> = None;
        for can_hit in self.iter() {
            let _t_max = maybe_hit.as_ref().map(|h| h.t).unwrap_or(t_max);
            maybe_hit = can_hit.hit(ray, t_min, _t_max).or(maybe_hit);
        }
        maybe_hit
    }
}
