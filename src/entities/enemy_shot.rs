use std::time::Duration;

use crossterm::style::Color;
use rusty_time::timer::Timer;

use crate::frame::{Cell, Drawable};
use crate::NUM_ROWS;

pub struct EnemyShot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl EnemyShot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(80),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y < NUM_ROWS - 1 {
                self.y += 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(150);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.y >= NUM_ROWS - 1
    }
}

impl Drawable for EnemyShot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        if self.exploding {
            frame[self.x][self.y] = Cell::new('*', Color::Red);
        } else {
            frame[self.x][self.y] = Cell::new('V', Color::Red);
        }
    }
}
