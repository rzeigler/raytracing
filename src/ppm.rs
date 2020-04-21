use crate::data::{Canvas, Vec3};

use anyhow::{Context, Result};
use std::io::Write;

const WRITE_FAIL: &str = "failed to write image data";

pub fn write<T: Write>(canvas: &Canvas, write: &mut T) -> Result<()> {
    writeln!(write, "P3\n{} {}\n255", canvas.width, canvas.height).context(WRITE_FAIL)?;
    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            write_pixel(canvas.at(j, i), write)?;
        }
    }
    Ok(())
}

fn write_pixel<T: Write>(p: &Vec3, write: &mut T) -> Result<()> {
    writeln!(
        write,
        "{} {} {}",
        (p.0 * 255.99f64) as u8,
        (p.1 * 255.99f64) as u8,
        (p.2 * 255.99f64) as u8
    )
    .context(WRITE_FAIL)
}
