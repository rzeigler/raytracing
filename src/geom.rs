use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn length_squared(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn invert(&self) -> Vec3 {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(l, r)| l * r)
            .sum()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        for (l, r) in self.data.iter_mut().zip(rhs.data.iter()) {
            *l += r;
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        for (l, r) in self.data.iter_mut().zip(rhs.data.iter()) {
            *l -= r;
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        for (l, r) in self.data.iter_mut().zip(rhs.data.iter()) {
            *l *= r;
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for l in self.data.iter_mut() {
            *l *= rhs;
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        for l in self.data.iter_mut() {
            *l /= rhs;
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        let mut dup = self;
        dup += rhs;
        dup
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        let mut dup = self;
        dup -= rhs;
        dup
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut dup = self;
        dup *= rhs;
        dup
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut dup = self;
        dup *= rhs;
        dup
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        let mut dup = self;
        dup /= rhs;
        dup
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
