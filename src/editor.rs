//! # Editor

use crate::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::path::Path;

/// The `Editor` struct contains the state and configuration of the editor
#[derive(Default)]
pub struct Editor {
    /// The file currently open in the editor
    file_name: Option<String>,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let editor = Self::default();

        Ok(editor)
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

        Ok(())
    }

    fn load_file(&mut self, path: &Path) -> Result<(), Error> {
        let file_type = std::fs::metadata(path)?.file_type();
        if !file_type.is_file() || file_type.is_symlink() {
            return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid input file type").into());
        }

        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).split(b'\n') {
                    for (_, c) in line?.iter().enumerate() {
                        print!("{}", (*c) as char);
                    }
                    println!();
                }
            }
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }
}
