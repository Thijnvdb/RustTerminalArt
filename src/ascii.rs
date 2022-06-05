use image::Luma;
use image::Rgba;
use crate::utils::map_range;

pub static GRADIENT: &str = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

pub fn get_char_from_pixels(pixel: Rgba<u8>, luma_pixel: Luma<u8>) -> String {
    if pixel[3] == 0 {return "  ".to_string();}
    let index = map_range((0, 256), (0, GRADIENT.chars().count() - 1), luma_pixel[0] as usize);
    let character: char = GRADIENT.chars().nth(index).unwrap();
    return String::from(format!("{}{}", character, character));
}