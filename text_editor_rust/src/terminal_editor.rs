use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct TerminalEditor {
    is_quit: bool,
}

impl TerminalEditor {
    // the Default trait to provide default values.
    pub fn default() -> Self {
        TerminalEditor { is_quit: false }
    }

    pub fn run(&mut self) {
        Self::init().unwrap();
        self.repl().unwrap();
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

    fn refresh(&self) {
        if self.is_quit {
            Self::clear_screen();
        }
    }

    // repl => Read Eval Print Loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate(&event);
            self.refresh();
            if self.is_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, event: &Event) {
        if let Key(KeyEvent { code, .. }) = event {
            match code {
                Char('q') => {
                    self.is_quit = true;
                }
                _ => (),
            }
        }
    }
}
