use crate::data::*;

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * unit_direction.y() + 1f64;
    UNIT * (1f64 - t) + Vec3(0.5, 0.7, 1.0) * t
}

pub fn draw(height: usize, width: usize) -> Canvas {
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = ZERO;
    let mut canvas = Canvas::new(height, width);
    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let u = (i as f64) / width as f64;
            let v = (j as f64) / height as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = ray_color(&r);
            canvas.set(i, j, color)
        }
    }
    canvas
}
