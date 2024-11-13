//! # Editor

use crate::input::{Input, Key};
use crate::{row::Row, textarea::TextArea, Error};
use core::fmt;
use crossterm::event::{read, Event};
use ratatui::layout::Layout;
use ratatui::layout::{Constraint, Position};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::path::Path;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
}

impl Default for EditorMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl fmt::Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EditorMode::Normal => write!(f, "NORMAL"),
            EditorMode::Insert => write!(f, "INSERT"),
        }
    }
}

/// The `Editor` struct contains the state and configuration of the editor
#[derive(Default)]
pub struct Editor {
    /// The current editing mode
    mode: EditorMode,
    /// The individual lines for the open file
    rows: Vec<Row>,
    /// The text area component
    textarea: TextArea,
    /// The file currently open in the editor
    file_name: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::Normal,
            rows: Vec::new(),
            textarea: TextArea::new(Vec::new()),
            file_name: None,
        }
    }

    pub fn run(&mut self, file_name: &Option<String>) -> Result<(), Error> {
        if let Some(path) = file_name
            .as_ref()
            .map(|p| std::path::PathBuf::from(p.as_str()))
        {
            self.load_file(path.as_path())?;
            self.file_name = Some(path.to_string_lossy().to_string());
        } else {
            self.file_name = None;
        }

        let mut terminal = ratatui::init();

        self.textarea = TextArea::new(self.rows.iter_mut().map(|r| r.get_string()).collect())
            .set_style(Style::new().fg(Color::White));

        loop {
            terminal.draw(|frame| {
                use Constraint::{Length, Min};

                let vertical = Layout::vertical([Min(0), Length(1)]);
                let [main_area, status_area] = vertical.areas(frame.area());

                let mut status_text = String::new();
                fmt::write(&mut status_text, format_args!("Mode: {}", self.mode)).unwrap();
                frame.render_widget(Text::from(status_text), status_area);
                frame.render_widget(self.textarea.clone(), main_area);
                frame.set_cursor_position(Position::new(
                    self.textarea.get_cursor_x(),
                    self.textarea.get_cursor_y(),
                ));
            })?;

            if let Event::Key(key) = read()? {
                let input = Input::from_key_event(key);

                let should_quit = match input {
                    Input {
                        key: Key::Character('Q'),
                        shift: true,
                        ..
                    } => self.mode == EditorMode::Normal,
                    _ => false,
                };

                if should_quit {
                    break;
                }

                self.handle_input(input);
            }
        }
        ratatui::restore();
        Ok(())
    }

    /// Handle a key input with default key mappings.
    ///
    /// Return if the input modified the text in the textarea.
    fn handle_input(&mut self, input: Input) -> bool {
        if self.mode == EditorMode::Normal {
            return self.handle_normal_input(input);
        } else if self.mode == EditorMode::Insert {
            return self.handle_insert_input(input);
        }

        false
    }

    fn handle_normal_input(&mut self, input: Input) -> bool {
        let modified = match input {
            Input {
                key: Key::Character('i'),
                ..
            } => {
                self.set_insert_mode();
                false
            }
            Input {
                key: Key::Character('l'),
                ..
            }
            | Input {
                key: Key::RightArrow,
                ..
            } => {
                self.textarea.move_cursor_right();
                false
            }
            Input {
                key: Key::Character('h'),
                ..
            }
            | Input {
                key: Key::LeftArrow,
                ..
            } => {
                self.textarea.move_cursor_left();
                false
            }
            Input {
                key: Key::Character('j'),
                ..
            }
            | Input {
                key: Key::DownArrow,
                ..
            } => {
                self.textarea.move_cursor_down();
                false
            }
            Input {
                key: Key::Character('k'),
                ..
            }
            | Input {
                key: Key::UpArrow, ..
            } => {
                self.textarea.move_cursor_up();
                false
            }
            _ => false,
        };

        modified
    }

    fn handle_insert_input(&mut self, input: Input) -> bool {
        let modified = match input {
            Input {
                key: Key::Escape, ..
            } => {
                self.set_normal_mode();
                false
            }
            _ => false,
        };

        modified
    }

    fn set_insert_mode(&mut self) {
        self.mode = EditorMode::Insert;
    }

    fn set_normal_mode(&mut self) {
        self.mode = EditorMode::Normal;
    }

    fn load_file(&mut self, path: &Path) -> Result<(), Error> {
        let file_type = std::fs::metadata(path)?.file_type();
        if !file_type.is_file() || file_type.is_symlink() {
            return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid input file type").into());
        }

        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).split(b'\n') {
                    self.rows.push(Row::new(line?));
                }
            }
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }
}
