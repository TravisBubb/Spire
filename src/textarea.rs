use ratatui::{
    layout::{Margin, Position, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{
        Block, Borders, ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};

/// A type to manage the state of a textarea
#[derive(Clone, Default)]
pub struct TextArea {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    style: Style,
}

impl Widget for TextArea {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default().borders(Borders::BOTTOM);

        block.render(area, buf);

        let text = self
            .lines
            .iter()
            .map(|line| Span::from(Span::styled(line.clone(), self.style)))
            .collect::<Vec<Span>>();

        for (i, span) in text.iter().enumerate() {
            if i < area.height as usize {
                span.render(area.inner(Margin::new(0, i.try_into().unwrap())), buf);
            }
        }

        let mut state = ScrollbarState::default().content_length(self.cursor_x);

        Scrollbar::new(ScrollbarOrientation::HorizontalBottom).render(
            area.inner(Margin::new(0, area.height - 3)),
            buf,
            &mut state,
        );
    }
}

impl TextArea {
    /// Create a `TextArea` with the provided lines.
    pub fn new(mut lines: Vec<String>) -> Self {
        if lines.is_empty() {
            lines.push(String::new());
        }

        Self {
            lines,
            cursor_x: 0,
            cursor_y: 0,
            style: Style::default().fg(Color::White),
        }
    }

    pub fn get_lines(&mut self) -> Vec<String> {
        self.lines.clone()
    }

    /// Retrieve the current X-position of the cursor in the TextArea with respect to the given area
    pub fn get_cursor_x(&self, area: Rect) -> u16 {
        match TryInto::<u16>::try_into(self.cursor_x) {
            Ok(val) => val,
            Err(_) => panic!("Cursor X value too large: {}", self.cursor_x),
        }
    }

    /// Retrieve the current Y-position of the cursor in the TextArea with respect to the given area
    pub fn get_cursor_y(&self, area: Rect) -> u16 {
        match TryInto::<u16>::try_into(self.cursor_y) {
            Ok(val) => val,
            Err(_) => panic!("Cursor Y value too large: {}", self.cursor_y),
        }
    }

    /// Set the `style` property of the `TextArea`..
    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    /// Move the cursor one unit to the right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_x == self.lines[self.cursor_y].len() - 1 {
            if self.cursor_y == self.lines.len() - 1 {
                return;
            }

            self.cursor_y += 1;
            self.cursor_x = 0;
        } else {
            self.cursor_x += 1;
        }
    }

    /// Move the cursor one unit to the left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_x == 0 {
            if self.cursor_y == 0 {
                return;
            }

            self.cursor_y -= 1;
            self.move_cursor_to_end();
        } else {
            self.cursor_x -= 1;
        }
    }

    /// Move the cursor one unit up
    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len() - 1);
        }
    }

    /// Move the cursor one unit down
    pub fn move_cursor_down(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len() - 1);
        }
    }

    /// Move cursor to the end of the current line
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_x = self.lines[self.cursor_y].len() - 1;
    }

    /// Insert a single character at the current cursor position
    pub fn insert_character(&mut self, c: char) {
        self.lines[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
    }

    /// Delete 1 character to the left of the cursor
    pub fn delete_left(&mut self) {
        if self.cursor_x == 0 && self.cursor_y == 0 {
            return;
        }

        if self.cursor_x == 0
            && self.lines[self.cursor_y]
                .chars()
                .filter(|c| !c.is_whitespace())
                .count()
                == 0
        {
            self.lines.remove(self.cursor_y);
            self.move_cursor_left();
            return;
        }

        self.move_cursor_left();
        self.lines[self.cursor_y].remove(self.cursor_x);
    }
}
