//! # Rows

#[derive(Clone)]
pub struct Row {
    characters: Vec<u8>,
}

impl Row {
    pub fn from_vec(mut chars: Vec<u8>) -> Self {
        if !chars.ends_with(&[b'\n']) {
            chars.push(b'\n');
        }

        Row { characters: chars }
    }

    pub fn from_slice(chars: &[u8]) -> Self {
        let mut char_vec = chars.to_vec();

        if !char_vec.ends_with(&[b'\n']) {
            char_vec.push(b'\n');
        }

        Row { characters: chars.to_vec() }
    }

    pub fn print(&self) {
        for (_, c) in self.characters.iter().enumerate() {
            print!("{}", (*c) as char);
        }
        println!();
    }

    pub fn get_bytes(&mut self) -> &[u8] {
        self.characters.as_slice()
    }

    pub fn get_string(&mut self) -> String {
        String::from_utf8(self.characters.clone()).unwrap()
    }
}
