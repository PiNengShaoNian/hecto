use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }
        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
