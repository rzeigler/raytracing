use std::borrow::Borrow;
use std::ops::*;

#[derive(Clone)]
pub struct Pixel {
    pub data: [u8; 4],
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { data: [0, 0, 0, 0] }
    }

    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { data: [r, g, b, a] }
    }

    /**
     * Assumes all values in [0, 1)
     */
    pub fn from_f64(r: f64, g: f64, b: f64, a: f64) -> Pixel {
        Pixel::from_u8(
            (r * 255.99) as u8,
            (g * 255.99) as u8,
            (b * 255.99) as u8,
            (a * 255.99) as u8,
        )
    }

    pub fn from_color(color: &Color) -> Pixel {
        Pixel::from_f64(color.0.x, color.0.y, color.0.z, 1.0)
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn flip(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.borrow().x;
        self.y += rhs.borrow().y;
        self.z += rhs.borrow().z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Additional helpers for *
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

#[derive(Clone, Copy)]
pub struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3(Vec3::new(x, y, z))
    }
}

impl From<Vec3> for Point3 {
    fn from(vec: Vec3) -> Self {
        Point3(vec)
    }
}
#[derive(Clone, Copy)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }
}

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Self {
        Color(vec)
    }
}

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
        }
    }
}

pub trait Hittable {
    fn hit_by(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray) -> bool {
        let oc = ray.origin.0 - self.center.0;
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
