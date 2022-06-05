extern crate image;
use std::path::Path;
use std::fs::File;
use image::GenericImage;
mod ascii;
mod ansi;
mod utils;


fn main() {
    let img = image::open("./images/umbreon.png").unwrap();
    let resized = img.resize(64, 64, image::imageops::FilterType::Nearest);
    // println!("dimensions {:?}", resized.dimensions());

    let lum = resized.to_luma();
    
    // let mut fout = File::create(&Path::new("./images/madeline.png")).unwrap();
    // let _ = resized.save(&mut fout, image::PNG);

    for y in 0..resized.dimensions().1 {
        for x in 0..resized.dimensions().0 {
            let pixel = resized.get_pixel(x, y);
            let luma_pixel = lum.get_pixel(x,y);

            if pixel[3] == 0 {
                print!{"  "};
            } else {
                print!("{}{}", ansi::from_rbg(pixel[0], pixel[1], pixel[2]), ascii::get_char_from_pixels(pixel, *luma_pixel));
                //print!("{}", ascii::get_char_from_pixels(pixel, *luma_pixel));
            }
        }
        print!("\n");
    }
}
