extern crate image;
use clap;

mod print;
mod ascii;
mod color;
mod utils;

use quicli::prelude::*;
use clap::{Parser, ArgEnum};
use std::io::Write;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Input file to read
    file: String,
    // Print mode to use
    #[clap(short, long, arg_enum)]
    mode: Option<Mode>,
    // Color format to use
    #[clap(short, long, arg_enum)]
    color_format: Option<ColorFormat>,

    // Directory to export to
    #[clap(short, long)]
    export_dir: Option<String>,

    // Max size for width/height
    #[clap(short, long)]
    size: Option<u32>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Ascii,
    AsciiColored,
    Pixels
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum ColorFormat {
    Truecolor,
    Ansi256
}

fn main() -> CliResult {
    let args: Arguments = Arguments::parse();
    let size: u32 = if let Some(s) = args.size {s} else {
        if let Some((w, h)) = term_size::dimensions() {
            std::cmp::min(w / 2, h) as u32
        } else {
            64
        }
    };

    let img = image::open(&args.file)?;
    let resized = img.resize(size, size, image::imageops::FilterType::Nearest);

    let mode = if let Some(s) = args.mode {s} else {Mode::Pixels};
    let color_format = if let Some(f) = args.color_format {f} else {ColorFormat::Truecolor};

    let color_fun: &dyn Fn(u8, u8, u8) -> String = match color_format{
        ColorFormat::Ansi256 => &color::ansi256,
        ColorFormat::Truecolor => &color::truecolor
    };

    let output: Vec<String> = match mode {
        Mode::Ascii => print::ascii_luma(&resized),
        Mode::AsciiColored => print::ascii_colored(&resized, color_fun),
        Mode::Pixels => print::filled_rgb(&resized, color_fun)
    };

    for line in &output {
        println!("{}", line);
    }

    if let Some(out_dir) = args.export_dir {
        if std::path::Path::new(&out_dir).is_dir() {
            let name: &str = std::path::Path::new(&args.file).file_stem().unwrap().to_str().unwrap();
            let mut file = std::fs::File::create(format!("{}/{}", out_dir, format!("{}.sh", name))).expect("Failed to create file");
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
