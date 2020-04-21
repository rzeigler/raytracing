use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        let u = self;
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        let u = self;
        Vec3(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Vec3 {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Vec3 {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

const ZERO: Vec3 = Vec3(0f64, 0f64, 0f64);

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Vec3>>,
}

impl Canvas {
    pub fn new(height: usize, width: usize) -> Canvas {
        let row = {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(ZERO);
            }
            row
        };
        let mut cols = Vec::with_capacity(height);
        for _ in 0..height {
            cols.push(row.clone());
        }
        Canvas {
            width,
            height,
            pixels: cols,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &Vec3 {
        &self.pixels[x][y]
    }

    // pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Vec3 {
    //     &mut self.pixels[x][y]
    // }

    pub fn set(&mut self, x: usize, y: usize, vec: Vec3) {
        self.pixels[x][y] = vec;
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
