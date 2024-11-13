//! # Rows

#[derive(Clone)]
pub struct Row {
    characters: Vec<u8>,
}

impl Row {
    pub fn new(chars: Vec<u8>) -> Self {
        Row { characters: chars }
    }

    pub fn print(&self) {
        for (_, c) in self.characters.iter().enumerate() {
            print!("{}", (*c) as char);
        }
        println!();
    }

    pub fn get_string(&mut self) -> String {
        String::from_utf8(self.characters.clone()).unwrap()
    }
}
