use crate::data::*;

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * unit_direction.1 + 1f64;
    unimplemented!()
}

fn draw(width: usize, height: usize) -> Canvas {
    unimplemented!();
}
