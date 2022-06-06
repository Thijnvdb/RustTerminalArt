extern crate image;
use clap;

mod print;
mod ascii;
mod ansi;
mod utils;

use quicli::prelude::*;

use clap::Parser;
#[derive(Parser,Default,Debug)]
pub struct Arguments {
    /// Input file to read
    file: String,
    #[clap(short, long)]
    ascii: bool,
    #[clap(short, long)]
    pixels: bool,
    #[clap(short, long)]
    colored_ascii: bool,
}

fn main() -> CliResult {
    let mut size: u32 = 64;
    if let Some((w, h)) = term_size::dimensions() {
        size = std::cmp::min(w / 2, h) as u32;
    }

    let args = Arguments::parse();
    let img = image::open(&args.file)?;

    let resized = img.resize(size, size, image::imageops::FilterType::Nearest);
    // println!("dimensions {:?}", resized.dimensions());

    
    // let mut fout = File::create(&Path::new("./images/madeline.png")).unwrap();
    // let _ = resized.save(&mut fout, image::PNG);

    if (args.pixels && args.ascii) || (args.pixels && args.colored_ascii) || (args.ascii && args.colored_ascii) {
        print!("\x1b[31Invalid Arguments");
        std::process::exit(exitcode::USAGE);
    }

    if args.pixels {
        print::filled_rgb(&resized);
    } else if args.ascii {
        print::ascii_luma(&resized);
    } else if args.colored_ascii {
        print::ascii_colored(&resized);
    } else {
        print::filled_rgb(&resized);
    }

    Ok(())
}
