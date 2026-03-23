use crate::{NUM_COLS, NUM_ROWS};
use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            ch: ' ',
            fg: Color::White,
            bg: Color::Black,
        }
    }

    pub fn new(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            bg: Color::Black,
        }
    }

    pub fn with_bg(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }

    pub fn is_empty(&self) -> bool {
        self.ch == ' '
    }
}

pub type Frame = Vec<Vec<Cell>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(Cell::empty());
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
