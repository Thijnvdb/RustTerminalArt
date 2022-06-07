// pub static BLACK: &str = "30";
// pub static RED : &str = "31";
// pub static GREEN : &str = "32";
// pub static YELLOW : &str = "33";
// pub static BLUE : &str = "34";
// pub static MAGENTA : &str = "35";
// pub static CYAN : &str =  "36";
// pub static WHITE : &str = "37";
// pub static DEFAULT : &str = "39";
// pub static RESET : &str = "0";
// pub static ESC: &str = "\x1b";

pub fn truecolor(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;2;{};{};{}m", r, g, b);
}

pub fn ansi256(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;5;{}m",  rgb2ansi256::rgb_to_ansi256(r,g,b));
}