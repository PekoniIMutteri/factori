use super::{TERMINAL_SIZE, BOTTOM_SPACE};

pub fn display_all() {
    delimitations();
    go_to(1, 1);
}

fn go_to(x: u32, y: u32) {
    print!("\x1B[{};{}H", y, x);
}

fn delimitations() {
    horizontal_delimitation('#', TERMINAL_SIZE.1 - BOTTOM_SPACE, 1, TERMINAL_SIZE.0);
    vertical_delimitation('#', TERMINAL_SIZE.0 - 20, 1, TERMINAL_SIZE.1 - BOTTOM_SPACE);
}

fn horizontal_delimitation(character: char, y: u32, min: u32, max: u32) {
    go_to(min, y);
    for _ in min..(max / 2) {
        print!("{} ", character);
    }
}

fn vertical_delimitation(character: char, x: u32, min: u32, max: u32) {
    for y in min..max {
        go_to(x, y);
        print!("{}", character);
    }
}
