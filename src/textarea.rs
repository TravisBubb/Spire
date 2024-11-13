use ratatui::{
    layout::Margin,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Widget},
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

    /// Retrieve the current X-position of the cursor in the TextArea
    pub fn get_cursor_x(&self) -> u16 {
        match TryInto::<u16>::try_into(self.cursor_x) {
            Ok(val) => val,
            Err(_) => panic!("Cursor X value too large: {}", self.cursor_x),
        }
    }

    /// Retrieve the current Y-position of the cursor in the TextArea
    pub fn get_cursor_y(&self) -> u16 {
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
        self.cursor_x += 1;
    }

    /// Move the cursor one unit to the left
    pub fn move_cursor_left(&mut self) {
        self.cursor_x -= 1;
    }

    /// Move the cursor one unit up
    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    /// Move the cursor one unit down
    pub fn move_cursor_down(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }
}
