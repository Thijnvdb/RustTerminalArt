extern crate image;
use clap::{self, ValueEnum};

mod ascii;
mod color;
mod print;
mod utils;

use clap::Parser;
use image::DynamicImage;
use quicli::prelude::*;
use std::io::Write;

const DEFAULT_SIZE: u32 = 64;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Input file to read
    file: String,

    /// Print mode to use
    #[clap(short, long, value_enum)]
    mode: Option<Mode>,

    /// Color format to use
    #[clap(short, long, value_enum)]
    color_format: Option<ColorFormat>,

    /// Directory to export to
    #[clap(short, long)]
    export_dir: Option<String>,

    /// Max size for width/height
    #[clap(short, long)]
    size: Option<u32>,

    /// Value to change contrast by (negative values are allowed)
    #[clap(long)]
    contrast: Option<f32>,

    /// Value to change the hue by (negative values are allowed)
    #[clap(long)]
    hue: Option<i32>,

    /// Value to change the brightnes by (negative values are allowed)
    #[clap(long)]
    brightness: Option<i32>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Ascii,
    AsciiColored,
    Pixels,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorFormat {
    Truecolor,
    Ansi256,
}

fn main() -> CliResult {
    let args: Arguments = Arguments::parse();

    let mut img = image::open(&args.file)?;

    img = apply_edits(&img, &args);

    let mode = if let Some(s) = args.mode {
        s
    } else {
        Mode::Pixels
    };
    let color_format = if let Some(f) = args.color_format {
        f
    } else {
        ColorFormat::Truecolor
    };

    let color_fun: &dyn Fn(u8, u8, u8) -> String = match color_format {
        ColorFormat::Ansi256 => &color::ansi256,
        ColorFormat::Truecolor => &color::truecolor,
    };

    let output: Vec<String> = match mode {
        Mode::Ascii => print::ascii_luma(&img),
        Mode::AsciiColored => print::ascii_colored(&img, color_fun),
        Mode::Pixels => print::filled_rgb(&img, color_fun),
    };

    for line in &output {
        println!("{}", line);
    }

    if let Some(out_dir) = args.export_dir {
        if std::path::Path::new(&out_dir).is_dir() {
            let name: &str = std::path::Path::new(&args.file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let mut file = std::fs::File::create(format!("{}/{}", out_dir, format!("{}.sh", name)))
                .expect("Failed to create file");
            let _res0 = file.write_all("#!/bin/sh\n".as_bytes());
            for line in &output {
                let _res = file.write_all(format!("echo \"{}\"\n", line).as_bytes());
            }
        } else {
            print!("\x1b[0mExport failed: specified export directory is not a valid directory\n");
        }
    }

    Ok(())
}

fn apply_edits(image: &DynamicImage, args: &Arguments) -> DynamicImage {
    let mut copy = image.clone();
    let size: u32 = if let Some(s) = args.size {
        s
    } else {
        if let Some((w, h)) = term_size::dimensions() {
            std::cmp::min(w / 2, h) as u32
        } else {
            DEFAULT_SIZE
        }
    };

    if let Some(contrast) = args.contrast {
        copy = copy.adjust_contrast(contrast);
    }

    if let Some(hue) = args.hue {
        copy = copy.huerotate(hue);
    }

    if let Some(brightness) = args.brightness {
        copy = copy.brighten(brightness)
    }

    copy = copy.resize(size, size, image::imageops::FilterType::Nearest);

    copy
}
