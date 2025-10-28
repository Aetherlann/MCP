use crate::color::Color;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellAttributes {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub inverse: bool,
    pub hidden: bool,
}

impl Default for CellAttributes {
    fn default() -> Self {
        Self {
            foreground: Color::Default,
            background: Color::Default,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            inverse: false,
            hidden: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub ch: char,
    pub attrs: CellAttributes,
    pub wide: bool, // Double-width character
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            attrs: CellAttributes::default(),
            wide: false,
        }
    }
}

/// Terminal grid (scrollback + visible screen)
pub struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Cell>,
    scrollback: Vec<Vec<Cell>>,
    scrollback_limit: usize,
    cursor_x: usize,
    cursor_y: usize,
    current_attrs: CellAttributes,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, scrollback_limit: usize) -> Self {
        let cells = vec![Cell::default(); rows * cols];
        Self {
            rows,
            cols,
            cells,
            scrollback: Vec::new(),
            scrollback_limit,
            cursor_x: 0,
            cursor_y: 0,
            current_attrs: CellAttributes::default(),
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
        self.cells.resize(rows * cols, Cell::default());
    }

    pub fn put_char(&mut self, ch: char) {
        let width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);

        if self.cursor_x >= self.cols {
            self.cursor_x = 0;
            self.cursor_y += 1;
            if self.cursor_y >= self.rows {
                self.scroll_up(1);
                self.cursor_y = self.rows - 1;
            }
        }

        let idx = self.cursor_y * self.cols + self.cursor_x;
        if idx < self.cells.len() {
            self.cells[idx] = Cell {
                ch,
                attrs: self.current_attrs,
                wide: width > 1,
            };
        }

        self.cursor_x += width;
    }

    pub fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    pub fn line_feed(&mut self) {
        self.cursor_y += 1;
        if self.cursor_y >= self.rows {
            self.scroll_up(1);
            self.cursor_y = self.rows - 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn tab(&mut self) {
        self.cursor_x = ((self.cursor_x / 8) + 1) * 8;
        if self.cursor_x >= self.cols {
            self.cursor_x = self.cols - 1;
        }
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x.min(self.cols - 1);
        self.cursor_y = y.min(self.rows - 1);
    }

    pub fn move_cursor_relative(&mut self, dx: isize, dy: isize) {
        let new_x = (self.cursor_x as isize + dx).max(0) as usize;
        let new_y = (self.cursor_y as isize + dy).max(0) as usize;
        self.move_cursor(new_x, new_y);
    }

    pub fn clear_screen(&mut self) {
        self.cells.fill(Cell::default());
    }

    pub fn clear_line(&mut self) {
        let start = self.cursor_y * self.cols;
        let end = start + self.cols;
        if end <= self.cells.len() {
            self.cells[start..end].fill(Cell::default());
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        for _ in 0..lines {
            // Save first line to scrollback
            let first_line: Vec<Cell> = (0..self.cols)
                .map(|x| self.cells[x].clone())
                .collect();

            self.scrollback.push(first_line);

            // Trim scrollback
            if self.scrollback.len() > self.scrollback_limit {
                self.scrollback.remove(0);
            }

            // Shift all rows up
            for y in 1..self.rows {
                for x in 0..self.cols {
                    let src = y * self.cols + x;
                    let dst = (y - 1) * self.cols + x;
                    self.cells[dst] = self.cells[src].clone();
                }
            }

            // Clear last line
            let last_line_start = (self.rows - 1) * self.cols;
            for i in 0..self.cols {
                self.cells[last_line_start + i] = Cell::default();
            }
        }
    }

    pub fn set_attributes(&mut self, attrs: CellAttributes) {
        self.current_attrs = attrs;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.cols && y < self.rows {
            Some(&self.cells[y * self.cols + x])
        } else {
            None
        }
    }

    pub fn cursor_pos(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn scrollback(&self) -> &[Vec<Cell>] {
        &self.scrollback
    }
}
