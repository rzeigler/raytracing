use crate::canvas::Canvas;

use anyhow::{Context, Result};
use std::io::Write;

const WRITE_FAIL: &str = "failed to write image data";

pub fn write<T: Write>(canvas: &Canvas, write: &mut T) -> Result<()> {
    writeln!(write, "P3\n{} {}\n255", canvas.width, canvas.height).context(WRITE_FAIL)?;
    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let p = canvas.at(j, i);
            writeln!(write, "{} {} {}", p.r, p.g, p.b).context(WRITE_FAIL)?;
        }
    }
    Ok(())
}
