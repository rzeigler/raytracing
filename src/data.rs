use std::ops::{Add, Div, Mul, Sub};

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
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            bytes: vec![vec![Pixel::new(); width as usize]; height as usize],
        }
    }

    pub fn at(&self, x: u32, y: u32) -> &Pixel {
        &self.bytes[y as usize][x as usize]
    }

    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.bytes[y as usize][x as usize]
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

    fn mahattan_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.mahattan_squared().sqrt()
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
}
