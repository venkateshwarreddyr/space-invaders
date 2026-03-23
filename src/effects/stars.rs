use crossterm::style::Color;
use rand::Rng;
use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::{Cell, Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};

struct Star {
    x: usize,
    y: usize,
    ch: char,
    color: Color,
    twinkle_timer: Timer,
    visible: bool,
}

pub struct Starfield {
    stars: Vec<Star>,
}

impl Starfield {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();

        let star_chars = ['.', '+', '*', '`', '\''];
        let star_colors = [
            Color::DarkGrey,
            Color::Grey,
            Color::White,
            Color::DarkBlue,
            Color::Blue,
        ];

        for _ in 0..40 {
            let x = rng.gen_range(0..NUM_COLS);
            let y = rng.gen_range(0..NUM_ROWS);
            let ch = star_chars[rng.gen_range(0..star_chars.len())];
            let color = star_colors[rng.gen_range(0..star_colors.len())];
            let twinkle_ms = rng.gen_range(500..3000);

            stars.push(Star {
                x,
                y,
                ch,
                color,
                twinkle_timer: Timer::from_millis(twinkle_ms),
                visible: rng.gen_bool(0.7),
            });
        }

        Self { stars }
    }

    pub fn update(&mut self, delta: Duration) {
        for star in self.stars.iter_mut() {
            star.twinkle_timer.update(delta);
            if star.twinkle_timer.ready {
                star.visible = !star.visible;
                star.twinkle_timer.reset();
            }
        }
    }
}

impl Default for Starfield {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Starfield {
    fn draw(&self, frame: &mut Frame) {
        for star in &self.stars {
            if star.visible && frame[star.x][star.y].is_empty() {
                frame[star.x][star.y] = Cell::new(star.ch, star.color);
            }
        }
    }
}
