use crate::ascii::get_char_from_luma;
use crate::image::GenericImageView;
use image::DynamicImage;

pub fn ascii_luma(img: &DynamicImage) {
    let lum = img.to_luma8();

    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let character = get_char_from_luma(*luma_pixel);
            print!("{}{}", character, character);
        }
        print!("\n");
    }
}

pub fn ascii_colored(img: &DynamicImage, color_fun: &dyn Fn(u8, u8, u8) -> String) {
    let lum = img.to_luma8();

    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let pixel = img.get_pixel(x, y);
            let character = get_char_from_luma(*luma_pixel);
            let color = color_fun(pixel[0], pixel[1], pixel[2]);
            print!("{}{}{}", color, character, character);
        }
        print!("\n");
    }
}

pub fn filled_rgb(img: &DynamicImage, color_fun: &dyn Fn(u8, u8, u8) -> String) {
    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            let pixel = img.get_pixel(x, y);
            if pixel[3] == 0 {
                print!("  ");
            } else {
                let color = color_fun(pixel[0], pixel[1], pixel[2]);
                print!("{}██", color,);
            }

        }
        print!("\n");
    }
}