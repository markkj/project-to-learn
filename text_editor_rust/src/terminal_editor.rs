use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

pub struct TerminalEditor {}

impl TerminalEditor {
    // the Default trait to provide default values.
    pub fn default() -> Self {
        TerminalEditor {}
    }

    pub fn run(&self) {
        // unwrap is is method is used to extract the value inside and `Option` or `Result` type
        // and we confident that the operation will succeed
        // and do not want to explicitly handle errors or None
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            // match is handle Result
            match b {
                Ok(b) => {
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
                        println!("exit program");
                        break;
                    }
                }
                Err(err) => {
                    println!("Error: {}", err)
                }
            }
        }
        disable_raw_mode().unwrap();
    }
}
