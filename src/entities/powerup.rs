use std::time::Duration;

use crossterm::style::Color;
use rusty_time::timer::Timer;

use crate::frame::{Cell, Drawable};
use crate::NUM_ROWS;

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    RapidFire,
    TripleShot,
    Shield,
    ExtraLife,
    Bomb,
}

impl PowerUpType {
    pub fn ch(&self) -> char {
        match self {
            PowerUpType::RapidFire => 'R',
            PowerUpType::TripleShot => 'T',
            PowerUpType::Shield => 'S',
            PowerUpType::ExtraLife => 'L',
            PowerUpType::Bomb => 'B',
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PowerUpType::RapidFire => Color::Yellow,
            PowerUpType::TripleShot => Color::Magenta,
            PowerUpType::Shield => Color::Cyan,
            PowerUpType::ExtraLife => Color::Green,
            PowerUpType::Bomb => Color::Red,
        }
    }

    pub fn duration_ms(&self) -> u64 {
        match self {
            PowerUpType::RapidFire => 5000,
            PowerUpType::TripleShot => 6000,
            PowerUpType::Shield => 4000,
            PowerUpType::ExtraLife => 0,
            PowerUpType::Bomb => 0,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i % 5 {
            0 => PowerUpType::RapidFire,
            1 => PowerUpType::TripleShot,
            2 => PowerUpType::Shield,
            3 => PowerUpType::ExtraLife,
            _ => PowerUpType::Bomb,
        }
    }
}

pub struct PowerUp {
    pub x: usize,
    pub y: usize,
    pub power_type: PowerUpType,
    fall_timer: Timer,
    blink_timer: Timer,
    visible: bool,
}

impl PowerUp {
    pub fn new(x: usize, y: usize, power_type: PowerUpType) -> Self {
        Self {
            x,
            y,
            power_type,
            fall_timer: Timer::from_millis(200),
            blink_timer: Timer::from_millis(150),
            visible: true,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.fall_timer.update(delta);
        self.blink_timer.update(delta);

        if self.fall_timer.ready {
            self.y += 1;
            self.fall_timer.reset();
        }
        if self.blink_timer.ready {
            self.visible = !self.visible;
            self.blink_timer.reset();
        }
    }

    pub fn dead(&self) -> bool {
        self.y >= NUM_ROWS - 1
    }
}

impl Drawable for PowerUp {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        if self.visible {
            frame[self.x][self.y] = Cell::new(self.power_type.ch(), self.power_type.color());
        }
    }
}

pub struct ActivePowerUp {
    pub power_type: PowerUpType,
    pub timer: Timer,
}

impl ActivePowerUp {
    pub fn new(power_type: PowerUpType) -> Self {
        Self {
            power_type,
            timer: Timer::from_millis(power_type.duration_ms()),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
    }

    pub fn expired(&self) -> bool {
        self.timer.ready
    }
}
