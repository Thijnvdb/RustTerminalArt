use crate::ascii::get_char_from_luma;
use crate::ansi::from_rbg;
use crate::image::GenericImageView;
use image::DynamicImage;

pub fn ascii_luma(img: &DynamicImage) {
    let lum = img.as_luma8().unwrap();

    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let character = get_char_from_luma(*luma_pixel);
            print!("{}{}", character, character);
        }
        print!("\n");
    }
}

pub fn ascii_colored(img: &DynamicImage) {
    let lum = img.as_luma8().unwrap();

    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let pixel = img.get_pixel(x, y);
            let character = get_char_from_luma(*luma_pixel);
            let color = from_rbg(pixel[0], pixel[1], pixel[2]);
            print!("{}{}{}", color, character, character);
        }
        print!("\n");
    }
}

pub fn filled_rgb(img: &DynamicImage) {
    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let pixel = img.get_pixel(x, y);
            if pixel[3] == 0 {
                print!("  ");
            } else {
                let color = from_rbg(pixel[0], pixel[1], pixel[2]);
                print!("{}██", color,);
            }

        }
        print!("\n");
    }
}