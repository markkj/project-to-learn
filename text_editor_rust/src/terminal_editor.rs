use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{self};

pub struct TerminalEditor {
    is_quit: bool,
    stdout: io::Stdout,
    cursor_x_position: u16,
    cursor_y_position: u16,
}

impl TerminalEditor {
    // the Default trait to provide default values.
    pub fn default() -> Self {
        TerminalEditor {
            is_quit: false,
            stdout: io::stdout(),
            cursor_x_position: 0,
            cursor_y_position: 0,
        }
    }

    pub fn run(&mut self) {
        self.init().unwrap();
        self.repl().unwrap();
        Self::cleanup().unwrap();
    }

    fn init(&mut self) -> Result<(), std::io::Error> {
        self.clear_screen()?;
        print!("Type q to quit.\r\n");
        return enable_raw_mode();
    }

    fn cleanup() -> Result<(), std::io::Error> {
        println!("Exiting...");
        return disable_raw_mode();
    }

    fn clear_screen(&mut self) -> Result<(), io::Error> {
        return execute!(self.stdout, Clear(ClearType::All));
    }

    fn refresh(&mut self) -> Result<(), io::Error> {
        return execute!(
            self.stdout,
            MoveTo(self.cursor_x_position, self.cursor_y_position)
        );
    }

    // repl => Read Eval Print Loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate(&event);
            self.refresh()?;
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
                KeyCode::Left => {
                    self.cursor_x_position = self.cursor_x_position.saturating_sub(1);
                }
                KeyCode::Right => {
                    self.cursor_x_position = self.cursor_x_position.saturating_add(1);
                }
                KeyCode::Up => {
                    self.cursor_y_position = self.cursor_y_position.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.cursor_y_position = self.cursor_y_position.saturating_add(1);
                }
                _ => (),
            }
        }
    }
}
