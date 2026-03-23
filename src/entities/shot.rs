use std::time::Duration;

use crossterm::style::Color;
use rusty_time::timer::Timer;

use crate::frame::{Cell, Drawable};

#[derive(Clone, Copy, PartialEq)]
pub enum ShotType {
    Normal,
    Rapid,
    Triple,
}

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub shot_type: ShotType,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize, shot_type: ShotType) -> Self {
        let speed = match shot_type {
            ShotType::Normal => 40,
            ShotType::Rapid => 20,
            ShotType::Triple => 35,
        };
        Self {
            x,
            y,
            exploding: false,
            shot_type,
            timer: Timer::from_millis(speed),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(200);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        if self.exploding {
            frame[self.x][self.y] = Cell::new('*', Color::Yellow);
            // Explosion splash effect
            if self.x > 0 {
                frame[self.x - 1][self.y] = Cell::new('.', Color::DarkYellow);
            }
            if self.x + 1 < frame.len() {
                frame[self.x + 1][self.y] = Cell::new('.', Color::DarkYellow);
            }
        } else {
            let (ch, color) = match self.shot_type {
                ShotType::Normal => ('|', Color::Cyan),
                ShotType::Rapid => ('!', Color::Yellow),
                ShotType::Triple => ('|', Color::Magenta),
            };
            frame[self.x][self.y] = Cell::new(ch, color);
        }
    }
}
