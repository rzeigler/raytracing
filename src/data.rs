use std::borrow::Borrow;

#[derive(Clone)]
pub struct Pixel {
    pub data: [u8; 4],
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { data: [0, 0, 0, 0] }
    }

    pub fn new_from_u8(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { data: [r, g, b, a] }
    }

    /**
     * Assumes all values in [0, 1)
     */
    pub fn new_from_f64(r: f64, g: f64, b: f64, a: f64) -> Pixel {
        Pixel::new_from_u8(
            (r * 255.99) as u8,
            (g * 255.99) as u8,
            (b * 255.99) as u8,
            (a * 255.99) as u8,
        )
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::new_from(0.0, 0.0, 0.0)
    }

    pub fn new_from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn flip(&self) -> Vec3 {
        Vec3::new_from(-self.x, -self.y, -self.z)
    }
}

impl<T: Borrow<Vec3>> std::ops::AddAssign<T> for Vec3 {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.borrow().x;
        self.y += rhs.borrow().y;
        self.z += rhs.borrow().z;
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3::new_from(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3::new_from(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
