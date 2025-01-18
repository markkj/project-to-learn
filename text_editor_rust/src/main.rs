use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        // is_control is function that if input is the general category for control codes.
        // example ctrl, tab, alt, etc.
        if c.is_control() {
            // 08b => # is add preix (such as 0bxxx), 0 is padding, 8 is maximum output, b is binary
            println!("Binary: {0:#08b} ASCII: {0:03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:03} Charactre: {1:?} \r", b, c);
        }
        if c == 'q' {
            disable_raw_mode().unwrap();
            println!("exit program");
            break;
        }
    }
}
