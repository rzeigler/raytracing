use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::UnitBall;
use std::cmp::Ordering;
use std::ops::*;
use std::sync::Arc;

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

    pub fn random_ball(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new_raw(UnitBall.sample(rng))
    }

    pub fn random_dist<D: Distribution<f64>>(rng: &mut ThreadRng, dist: &D) -> Vec3 {
        Vec3::new(dist.sample(rng), dist.sample(rng), dist.sample(rng))
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
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_at(origin: Vec3, direction: Vec3, at: f64) -> Ray {
        Ray {
            origin,
            direction,
            time: at,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Vec3, // a color
}

pub trait Material: Sync {
    // We hardcode the dep on ThreadRng...
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::new_raw(UnitBall.sample(rng));
        Some(Scatter {
            scattered: Ray::new_at(hit.point, scatter_direction, ray.time),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = reflect(&ray.direction.unit(), &hit.normal);
        let scattered = Ray::new_at(
            hit.point,
            reflected + self.fuzz * Vec3::new_raw(UnitBall.sample(rng)),
            ray.time,
        );
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

fn refact(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = uv.flip().dot(n);
    let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * *n;
    r_out_parallel + r_out_perp
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<Scatter> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = (unit_direction.flip().dot(&hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 || rng.sample(Uniform::new(0.0, 1.0)) < reflect_prob {
            let reflected = reflect(&unit_direction, &hit.normal);
            let scattered = Ray::new_at(hit.point, reflected, ray.time);
            Some(Scatter {
                scattered,
                attenuation,
            })
        } else {
            let refacted = refact(&unit_direction, &hit.normal, etai_over_etat);
            let scattered = Ray::new_at(hit.point, refacted, ray.time);
            Some(Scatter {
                scattered,
                attenuation,
            })
        }
    }
}

pub struct Hit<'ma> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'ma dyn Material,
}

impl<'ma> Hit<'ma> {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray: &Ray,
        material: &'ma dyn Material,
    ) -> Hit<'ma> {
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
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

// Is it possible to implement a Moving<A: Hittable>?
// The problem I forsee is that the material itself doesn't move so how would we blend?
// Probably surmountable
pub struct Timed<A> {
    value: A,
    time: f64,
}

impl<A> Timed<A> {
    pub fn new(value: A, time: f64) -> Timed<A> {
        Timed { value, time }
    }
}

pub struct Sphere {
    center0: Timed<Vec3>,
    center1: Timed<Vec3>,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center0: Timed {
                value: center,
                time: 0.0,
            },
            center1: Timed {
                value: center,
                time: f64::INFINITY,
            },
            radius,
            material,
        }
    }

    pub fn new_moving(
        center0: Timed<Vec3>,
        center1: Timed<Vec3>,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Sphere {
        Sphere {
            center0,
            center1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center0.value
            + ((time - self.center0.time) / (self.center1.time - self.center0.time))
                * (self.center1.value - self.center0.value)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let oc = ray.origin - self.center(ray.time);
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
                let outward_normal = (point - self.center(ray.time)) / self.radius;
                return Some(Hit::new(
                    point,
                    outward_normal,
                    t,
                    ray,
                    self.material.as_ref(),
                ));
            }
            let t = (-half_b + root) / a;
            if t < max_t && t > min_t {
                let point = ray.at(t);
                let outward_normal = (point - self.center(ray.time)) / self.radius;
                return Some(Hit::new(
                    point,
                    outward_normal,
                    t,
                    ray,
                    self.material.as_ref(),
                ));
            }
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(&box0, &box1))
    }
}

fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = Vec3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(small, big)
}

pub struct Collection(Vec<Box<dyn Hittable + Send + Sync>>);

impl Collection {
    pub fn new(v: Vec<Box<dyn Hittable + Send + Sync>>) -> Collection {
        Collection(v)
    }
}

impl Hittable for Collection {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let mut current_hit: Option<Hit> = None;
        for hittable in self.0.iter() {
            let _max_t = current_hit.as_ref().map(|h| h.t).unwrap_or(max_t);
            current_hit = hittable.hit(ray, min_t, _max_t).or(current_hit);
        }
        return current_hit;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut bound: Option<AABB> = None;
        for hit_bound in self.0.iter().map(|h| h.bounding_box(t0, t1)) {
            bound = match (bound, hit_bound) {
                (Some(b), Some(h)) => Some(surrounding_box(&b, &h)),
                (Some(b), None) => Some(b),
                (None, Some(h)) => Some(h),
                _ => None,
            }
        }
        bound
    }
}

/**
 * The bounding box thingamajig
 */
#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction.data[a];
            let mut t0 = (self.min.data[a] - ray.origin.data[a]) * inv_d;
            let mut t1 = (self.max.data[a] - ray.origin.data[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub struct BVHNode {
    left: Box<dyn Hittable + Send + Sync>,
    right: Box<dyn Hittable + Send + Sync>,
    time0: f64,
    time1: f64,
    aabb: Option<AABB>,
}

impl BVHNode {
    pub fn new(
        left: Box<dyn Hittable + Send + Sync>,
        right: Box<dyn Hittable + Send + Sync>,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let left_box = left.bounding_box(time0, time1);
        let right_box = right.bounding_box(time0, time1);
        let bound = match (left_box, right_box) {
            (Some(l), Some(r)) => Some(surrounding_box(&l, &r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            _ => None,
        };
        BVHNode {
            left,
            right,
            time0,
            time1,
            aabb: bound,
        }
    }
}

struct Ephemeral;

impl Hittable for Ephemeral {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        None
    }
}

pub fn bvh_split_hittables(
    rng: &mut ThreadRng,
    mut hittables: Vec<Box<dyn Hittable + Send + Sync>>,
    t0: f64,
    t1: f64,
) -> Box<dyn Hittable + Send + Sync> {
    if hittables.is_empty() {
        Box::new(Ephemeral)
    } else if hittables.len() == 1 {
        hittables.into_iter().next().unwrap()
    } else {
        let axis = Axis::random(rng);
        hittables.sort_by(|left, right| {
            min_on_axis(axis, left, t0, t1)
                .partial_cmp(&min_on_axis(axis, right, t0, t1))
                .unwrap_or(Ordering::Equal)
        });
        let point = hittables.len() / 2;
        let right = hittables.split_off(point);
        Box::new(BVHNode::new(
            bvh_split_hittables(rng, hittables, t0, t1),
            bvh_split_hittables(rng, right, t0, t1),
            t0,
            t1,
        ))
    }
}

fn min_on_axis<T: Deref<Target = dyn Hittable + Send + Sync>>(
    axis: Axis,
    h: &T,
    t0: f64,
    t1: f64,
) -> f64 {
    match axis {
        Axis::X => h.bounding_box(t0, t1).map(|b| b.min.x()).unwrap_or(0.0),
        Axis::Y => h.bounding_box(t0, t1).map(|b| b.min.y()).unwrap_or(0.0),
        Axis::Z => h.bounding_box(t0, t1).map(|b| b.min.z()).unwrap_or(0.0),
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn random(rng: &mut ThreadRng) -> Axis {
        match rng.gen_range(0, 3) {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("impossible!"),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        if let Some(aabb) = self.aabb.as_ref() {
            if aabb.hit(ray, min_t, max_t) {
                let hit_left = self.left.hit(ray, min_t, max_t);
                let max_t = hit_left.as_ref().map(|h| h.t).unwrap_or(max_t);
                let hit_right = self.right.hit(ray, min_t, max_t);
                return hit_right.or(hit_left);
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.aabb.clone()
    }
}

pub trait Texture {
    fn color(u: f64, v: f64, point: &Vec3) -> Vec3;
}

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: Vec3::new(r, g, b),
        }
    }
}
