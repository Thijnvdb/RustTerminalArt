extern crate image;
use clap;

mod print;
mod ascii;
mod color;
mod utils;

use quicli::prelude::*;

use clap::{Parser, ArgEnum};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Input file to read
    file: String,
    // Style to use
    #[clap(short, long, arg_enum)]
    style: Option<Style>,
    // Color format to use
    #[clap(short, long, arg_enum)]
    color_format: Option<ColorFormat>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Style {
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
    let mut size: u32 = 64;
    if let Some((w, h)) = term_size::dimensions() {
        size = std::cmp::min(w / 2, h) as u32;
    }

    let args = Arguments::parse();
    let img = image::open(&args.file)?;

    let resized = img.resize(size, size, image::imageops::FilterType::Nearest);

    let style = if let Some(s) = args.style {s} else {Style::Pixels};
    let color_format = if let Some(f) = args.color_format {f} else {ColorFormat::Truecolor};

    let color_fun: &dyn Fn(u8, u8, u8) -> String = match color_format{
        ColorFormat::Ansi256 => &color::ansi256,
        ColorFormat::Truecolor => &color::truecolor
    };

    match style {
        Style::Ascii => {
            print::ascii_luma(&resized);
        }
        Style::AsciiColored => {
            print::ascii_colored(&resized, color_fun);
        }
        Style::Pixels => {
            print::filled_rgb(&resized, color_fun);
        }
    }

    Ok(())
}
