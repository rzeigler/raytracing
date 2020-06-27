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

    pub fn new_raw(data: [f64; 3]) -> Vec3 {
        Vec3 { data }
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

    pub fn flip(&self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        for (l, r) in self.data.iter_mut().zip(rhs.data.iter()) {
            *l += r;
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        for l in self.data.iter_mut() {
            *l += rhs;
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

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        let mut dup = self;
        dup += rhs;
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

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(point: Vec3, outward_normal: Vec3, t: f64, ray: &Ray) -> Hit {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal.flip()
        };
        Hit {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t < max_t && t > min_t {
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;
                return Some(Hit::new(point, outward_normal, t, ray));
            }
            let t = (-half_b + root) / a;
            if t < max_t && t > min_t {
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;
                return Some(Hit::new(point, outward_normal, t, ray));
            }
            None
        }
    }
}

pub struct World(Vec<Box<dyn Hittable + Send + Sync>>);

impl World {
    pub fn new(v: Vec<Box<dyn Hittable + Send + Sync>>) -> World {
        World(v)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let mut current_hit: Option<Hit> = None;
        for hittable in self.0.iter() {
            let _max_t = current_hit.as_ref().map(|h| h.t).unwrap_or(max_t);
            current_hit = hittable.hit(ray, min_t, _max_t).or(current_hit);
        }
        return current_hit;
    }
}
