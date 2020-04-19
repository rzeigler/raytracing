use anyhow::{Context, Result};
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod canvas;
mod ppm;

use canvas::*;

const OUT_PARAM: &str = "out";

fn main() {
    let matches = App::new("raytracing")
        .version("chapter-1")
        .author("Ryan Zeigler <zeiglerr@gmail.com>")
        .about("Implementation of the raytracing from https://raytracing.github.io/ to learn Rust")
        .arg(
            Arg::with_name(OUT_PARAM)
                .short("o")
                .long("out")
                .value_name("FILE")
                .takes_value(true)
                .help("The path to write output too"),
        )
        .get_matches();
    let out_path = Path::new(matches.value_of(OUT_PARAM).unwrap());

    if let Err(e) = run(&out_path) {
        eprintln!("raytracing failed:\n{}", e);
        std::process::exit(1);
    }
}

const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;

fn run(out_path: &Path) -> Result<()> {
    let mut canvas = Canvas::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let r = (i as f64) / (canvas.width as f64);
            let g = (j as f64) / (canvas.height as f64);
            let b = 0.2 as f64;

            let ir = (r * 255.99) as u8;
            let ig = (g * 255.99) as u8;
            let ib = (b * 255.99) as u8;
            let p = canvas.at_mut(j, i);
            p.r = ir;
            p.g = ig;
            p.b = ib;
        }
    }

    let mut out_file = File::create(out_path)
        .with_context(|| format!("unable to open {} for writting", out_path.to_string_lossy()))?;

    ppm::write(&canvas, &mut out_file)?;

    Ok(())
}
