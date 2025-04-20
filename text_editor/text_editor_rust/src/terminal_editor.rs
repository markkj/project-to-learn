use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{self, Write};

pub struct TerminalEditor {
    is_quit: bool,
    stdout: io::Stdout,
    cursor_x_position: u16,
    cursor_y_position: u16,
    text_buffer: [[char; 500]; 500],
    max_x_position: [u16; 500],
    max_line: u16,
}

impl TerminalEditor {
    // the Default trait to provide default values.
    pub fn default() -> Self {
        TerminalEditor {
            is_quit: false,
            stdout: io::stdout(),
            cursor_x_position: 1,
            cursor_y_position: 1,
            text_buffer: [[0 as char; 500]; 500],
            max_x_position: [1 as u16; 500],
            max_line: 1,
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

    fn print_text_buffer(&mut self) -> Result<(), io::Error> {
        for (i, row) in self.text_buffer.iter().enumerate() {
            let mut s = String::new();
            for col in row {
                s.push(*col);
            }
            execute!(self.stdout, MoveTo(1, (i + 1) as u16))?;
            print!("{}", s)
        }
        Ok(())
    }

    fn refresh(&mut self) -> Result<(), io::Error> {
        self.clear_screen()?;
        self.print_text_buffer()?;
        execute!(
            self.stdout,
            MoveTo(self.cursor_x_position, self.cursor_y_position)
        )?;
        self.stdout.flush()?;
        Ok(())
    }

    // repl => Read Eval Print Loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh()?;
            let event = read()?;
            self.evaluate(&event);
            if self.is_quit {
                break;
            }
        }
        Ok(())
    }

    fn move_cursor_to(&mut self, x: u16, y: u16) {
        let y = y.max(1);
        let x = x.max(1);
        let row: usize = (y - 1) as usize;
        self.cursor_x_position = self.max_x_position[row].min(x);
        self.cursor_y_position = y
    }

    fn evaluate(&mut self, event: &Event) {
        if let Key(KeyEvent { code, .. }) = event {
            match code {
                Char(c) => {
                    self.text_buffer[(self.cursor_y_position - 1) as usize]
                        [(self.cursor_x_position - 1) as usize] = *c;
                    self.max_x_position[(self.cursor_y_position - 1) as usize] += 1;
                    self.move_cursor_to(
                        self.cursor_x_position.saturating_add(1),
                        self.cursor_y_position,
                    );
                }
                KeyCode::Esc => {
                    self.is_quit = true;
                }
                KeyCode::Left => {
                    self.move_cursor_to(
                        self.cursor_x_position.saturating_sub(1),
                        self.cursor_y_position,
                    );
                }
                KeyCode::Right => {
                    self.move_cursor_to(
                        self.cursor_x_position.saturating_add(1),
                        self.cursor_y_position,
                    );
                }
                KeyCode::Up => {
                    self.move_cursor_to(
                        self.cursor_x_position,
                        self.cursor_y_position.saturating_sub(1),
                    );
                }
                KeyCode::Down => {
                    if self.cursor_y_position < self.max_line {
                        self.move_cursor_to(
                            self.cursor_x_position,
                            self.cursor_y_position.saturating_add(1),
                        );
                    }
                }
                KeyCode::Enter => {
                    self.max_line += 1;
                    self.move_cursor_to(
                        self.cursor_x_position,
                        self.cursor_y_position.saturating_add(1),
                    );
                }
                _ => (),
            }
        }
    }
}
