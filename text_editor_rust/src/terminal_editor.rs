use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct TerminalEditor {}

impl TerminalEditor {
    // the Default trait to provide default values.
    pub fn default() -> Self {
        TerminalEditor {}
    }

    pub fn run(&self) {
        Self::init().unwrap();
        self.repl();
        Self::cleanup().unwrap();
    }

    fn init() -> Result<(), std::io::Error> {
        Self::clear_screen();
        print!("Type q to quit.\r\n");
        return enable_raw_mode();
    }

    fn cleanup() -> Result<(), std::io::Error> {
        Self::clear_screen();
        return disable_raw_mode();
    }

    fn clear_screen() {
        println!("\x1b[2J");
    }

    // repl => Read Eval Print Loop
    fn repl(&self) {
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {err}");
                } // handle other key event that not exist in KeyEvent
                _ => {}
            }
        }
    }
}
