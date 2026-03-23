use std::time::Duration;

use crossterm::style::Color;
use rusty_time::timer::Timer;

use crate::frame::{Cell, Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ch: char,
    color: Color,
    life: Timer,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn spawn_explosion(&mut self, x: usize, y: usize, color: Color) {
        let chars = ['*', '.', '+', '~', 'o', '#'];
        let colors = [color, Color::Yellow, Color::DarkYellow, Color::White];

        for i in 0..12 {
            let angle = (i as f32) * std::f32::consts::PI * 2.0 / 12.0;
            let speed = 1.5 + (i % 3) as f32 * 0.8;
            self.particles.push(Particle {
                x: x as f32,
                y: y as f32,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed * 0.5,
                ch: chars[i % chars.len()],
                color: colors[i % colors.len()],
                life: Timer::from_millis(300 + (i as u64 % 3) * 100),
            });
        }
    }

    pub fn spawn_big_explosion(&mut self, x: usize, y: usize) {
        let chars = ['#', '@', '*', '%', '&', '!', '$', '~'];
        let colors = [
            Color::Red,
            Color::Yellow,
            Color::DarkYellow,
            Color::White,
            Color::Magenta,
        ];

        for i in 0..24 {
            let angle = (i as f32) * std::f32::consts::PI * 2.0 / 24.0;
            let speed = 2.0 + (i % 4) as f32 * 0.6;
            self.particles.push(Particle {
                x: x as f32,
                y: y as f32,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed * 0.5,
                ch: chars[i % chars.len()],
                color: colors[i % colors.len()],
                life: Timer::from_millis(400 + (i as u64 % 5) * 80),
            });
        }
    }

    pub fn spawn_powerup_collect(&mut self, x: usize, y: usize, color: Color) {
        let chars = ['+', '*', '.'];
        for i in 0..8 {
            let angle = (i as f32) * std::f32::consts::PI * 2.0 / 8.0;
            self.particles.push(Particle {
                x: x as f32,
                y: y as f32,
                vx: angle.cos() * 1.2,
                vy: angle.sin() * 0.6,
                ch: chars[i % chars.len()],
                color,
                life: Timer::from_millis(250),
            });
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for particle in self.particles.iter_mut() {
            particle.life.update(delta);
            particle.x += particle.vx * delta.as_secs_f32() * 15.0;
            particle.y += particle.vy * delta.as_secs_f32() * 15.0;
        }
        self.particles.retain(|p| !p.life.ready);
    }

    pub fn has_particles(&self) -> bool {
        !self.particles.is_empty()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for ParticleSystem {
    fn draw(&self, frame: &mut Frame) {
        for particle in &self.particles {
            let px = particle.x.round() as isize;
            let py = particle.y.round() as isize;
            if px >= 0 && px < NUM_COLS as isize && py >= 0 && py < NUM_ROWS as isize {
                let ux = px as usize;
                let uy = py as usize;
                if frame[ux][uy].is_empty() {
                    frame[ux][uy] = Cell::new(particle.ch, particle.color);
                }
            }
        }
    }
}
