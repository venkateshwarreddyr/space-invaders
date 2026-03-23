use crossterm::style::Color;

use crate::frame::{Cell, Drawable, Frame};
use crate::NUM_COLS;

const BARRIER_WIDTH: usize = 5;
const NUM_BARRIERS: usize = 4;

#[derive(Clone)]
pub struct BarrierBlock {
    pub x: usize,
    pub y: usize,
    pub health: u8,
}

impl BarrierBlock {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, health: 3 }
    }

    pub fn hit(&mut self) -> bool {
        if self.health > 0 {
            self.health -= 1;
            true
        } else {
            false
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn color(&self) -> Color {
        match self.health {
            3 => Color::Green,
            2 => Color::Yellow,
            1 => Color::Red,
            _ => Color::DarkGrey,
        }
    }

    fn ch(&self) -> char {
        match self.health {
            3 => '#',
            2 => '=',
            1 => '-',
            _ => ' ',
        }
    }
}

pub struct Barriers {
    pub blocks: Vec<BarrierBlock>,
}

impl Barriers {
    pub fn new(y_pos: usize) -> Self {
        let mut blocks = Vec::new();
        let spacing = NUM_COLS / (NUM_BARRIERS + 1);

        for i in 0..NUM_BARRIERS {
            let center_x = spacing * (i + 1);
            // Build a shield shape
            for dx in 0..BARRIER_WIDTH {
                let x = center_x - BARRIER_WIDTH / 2 + dx;
                if x < NUM_COLS {
                    blocks.push(BarrierBlock::new(x, y_pos));
                    if dx > 0 && dx < BARRIER_WIDTH - 1 {
                        blocks.push(BarrierBlock::new(x, y_pos + 1));
                    }
                }
            }
        }

        Self { blocks }
    }

    pub fn hit_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(block) = self
            .blocks
            .iter_mut()
            .find(|b| b.x == x && b.y == y && b.is_alive())
        {
            block.hit();
            true
        } else {
            false
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.blocks
            .iter()
            .any(|b| b.x == x && b.y == y && b.is_alive())
    }
}

impl Drawable for Barriers {
    fn draw(&self, frame: &mut Frame) {
        for block in &self.blocks {
            if block.is_alive() {
                frame[block.x][block.y] = Cell::new(block.ch(), block.color());
            }
        }
    }
}
