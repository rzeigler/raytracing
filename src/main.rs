use anyhow::{Context, Result};
use clap::{App, Arg};
use std::fs::File;
use std::path::Path;

mod data;
mod ppm;
mod tracer;

use data::*;

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

const IMAGE_WIDTH: usize = 1600;
const IMAGE_HEIGHT: usize = 800;

fn run(out_path: &Path) -> Result<()> {
    let canvas = tracer::draw(IMAGE_HEIGHT, IMAGE_WIDTH);
    let mut out_file = File::create(out_path)
        .with_context(|| format!("unable to open {} for writting", out_path.to_string_lossy()))?;

    ppm::write(&canvas, &mut out_file)?;

    Ok(())
}
