use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct Input {
    /// The typed key
    pub key: Key,
    /// Ctrl modifier; `true` means the Ctrl key was also pressed
    pub ctrl: bool,
    /// Alt modifier; `true` means the Alt key was also pressed
    pub alt: bool,
    /// Shift modifier; `true` means the Shift key was also pressed
    pub shift: bool
}

impl Input {
    pub fn from_key_event(key: KeyEvent) -> Self {
        Self {
            key: get_key_from_code(key.code),
            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
            alt: key.modifiers.contains(KeyModifiers::ALT),
            shift: key.modifiers.contains(KeyModifiers::SHIFT),
        }
    }
}

/// Key input type
pub enum Key {
    /// Normal letter input
    Character(char),
    /// Function ("F") keys
    Function(u8),
    /// Backspace key
    Backspace,
    /// Enter key
    Enter,
    /// Left arrow key
    LeftArrow,
    /// Right arrow key
    RightArrow,
    /// Up arrow key
    UpArrow,
    /// Down arrow key
    DownArrow,
    /// Tab key
    Tab,
    /// Delete key
    Delete,
    /// Home key
    Home,
    /// End key
    End,
    /// Page up key
    PageUp,
    /// Page down key
    PageDown,
    /// Escape key
    Escape,
    /// Invalid input - always ignored
    Null
}

fn get_key_from_code(code: KeyCode) -> Key {
    let key = match code {
        KeyCode::Char(c) => Key::Character(c),
        KeyCode::F(n) => Key::Function(n),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Left => Key::LeftArrow,
        KeyCode::Right => Key::RightArrow,
        KeyCode::Up => Key::UpArrow,
        KeyCode::Down => Key::DownArrow,
        KeyCode::Tab => Key::Tab,
        KeyCode::Delete => Key::Delete,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Esc => Key::Escape,
        _ => Key::Null
    }; 

    key
}