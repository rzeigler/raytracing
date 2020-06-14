#![warn(clippy::all)]
use anyhow::{Context, Result};
use clap::{value_t, App, Arg};
use std::fs::File;
use std::path::Path;

const IMAGE_WIDTH: usize = 1600;
const IMAGE_HEIGHT: usize = 800;

fn main() -> Result<()> {
    let matches = App::new("raytracing")
        .version("chapter-1")
        .author("Ryan Zeigler <zeiglerr@gmail.com>")
        .about("Implementation of the raytracing from https://raytracing.github.io/ to learn Rust")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("The width in pixels of the generated image"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .help("The height in pixels of the generated image"),
        )
        .arg(
            Arg::with_name("out")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("The path to write output too"),
        )
        .get_matches();
    let width = if matches.is_present("width") {
        value_t!(matches, "width", usize).with_context(|| "invalid width")?
    } else {
        IMAGE_WIDTH
    };
    let height = if matches.is_present("height") {
        value_t!(matches, "height", usize).with_context(|| "invalid height")?
    } else {
        IMAGE_WIDTH
    };
    let out_path = Path::new(matches.value_of("out").unwrap());
    run(width, height, &out_path)
}

fn run(width: usize, height: usize, out_path: &Path) -> Result<()> {
    Ok(())
}
