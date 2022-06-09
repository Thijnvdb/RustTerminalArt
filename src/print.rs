use crate::ascii::get_char_from_luma;
use crate::image::GenericImageView;
use image::DynamicImage;

pub fn ascii_luma(img: &DynamicImage) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let lum = img.to_luma8();

    for y in 0..img.dimensions().1 {
        let mut line: String = "".to_string();
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let character = get_char_from_luma(*luma_pixel);
            line.push_str(&format!("{}{}", character, character));
        }
        lines.push(line);
    }

    return lines;
}

pub fn ascii_colored(img: &DynamicImage, color_fun: &dyn Fn(u8, u8, u8) -> String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let lum = img.to_luma8();

    for y in 0..img.dimensions().1 {
        let mut line: String = "".to_string();
        for x in 0..img.dimensions().0 {
            let luma_pixel = lum.get_pixel(x,y);
            let pixel = img.get_pixel(x, y);
            let character = get_char_from_luma(*luma_pixel);
            let color = color_fun(pixel[0], pixel[1], pixel[2]);
            line.push_str(&format!("{}{}{}", color, character, character));
        }
        lines.push(line);
    }

    return lines;
}

pub fn filled_rgb(img: &DynamicImage, color_fun: &dyn Fn(u8, u8, u8) -> String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    
    for y in 0..img.dimensions().1 {
        let mut line: String = "".to_string();
        for x in 0..img.dimensions().0 {
            let pixel = img.get_pixel(x, y);
            if pixel[3] == 0 {
                line.push_str("  ");
            } else {
                let color = color_fun(pixel[0], pixel[1], pixel[2]);
                line.push_str(&format!("{}██", color));
            }
        }
        lines.push(line);
    }

    return lines;
}