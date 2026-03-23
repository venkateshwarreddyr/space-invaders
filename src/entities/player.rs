use std::time::Duration;

use crossterm::style::Color;
use rusty_time::timer::Timer;

use crate::entities::barrier::Barriers;
use crate::entities::enemy_shot::EnemyShot;
use crate::entities::invaders::Invaders;
use crate::entities::powerup::{ActivePowerUp, PowerUpType};
use crate::entities::shot::{Shot, ShotType};
use crate::frame::{Cell, Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub lives: u8,
    pub shots: Vec<Shot>,
    pub active_powerups: Vec<ActivePowerUp>,
    pub shield_active: bool,
    invincible_timer: Timer,
    pub is_invincible: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 2,
            lives: 3,
            shots: Vec::new(),
            active_powerups: Vec::new(),
            shield_active: false,
            invincible_timer: Timer::from_millis(0),
            is_invincible: false,
        }
    }

    pub fn reset_position(&mut self) {
        self.x = NUM_COLS / 2;
    }

    pub fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 2 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        let max_shots = if self.has_powerup(PowerUpType::RapidFire) {
            5
        } else {
            3
        };

        if self.shots.len() < max_shots {
            let shot_type = if self.has_powerup(PowerUpType::RapidFire) {
                ShotType::Rapid
            } else if self.has_powerup(PowerUpType::TripleShot) {
                ShotType::Triple
            } else {
                ShotType::Normal
            };

            self.shots.push(Shot::new(self.x, self.y - 1, shot_type));

            if self.has_powerup(PowerUpType::TripleShot) {
                if self.x > 1 {
                    self.shots
                        .push(Shot::new(self.x - 1, self.y - 1, ShotType::Triple));
                }
                if self.x < NUM_COLS - 2 {
                    self.shots
                        .push(Shot::new(self.x + 1, self.y - 1, ShotType::Triple));
                }
            }
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());

        for powerup in self.active_powerups.iter_mut() {
            powerup.update(delta);
        }
        self.active_powerups.retain(|p| !p.expired());

        self.shield_active = self.has_powerup(PowerUpType::Shield);

        self.invincible_timer.update(delta);
        if self.invincible_timer.ready {
            self.is_invincible = false;
        }
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders, barriers: &mut Barriers) -> Vec<u64> {
        let mut killed_points = Vec::new();
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if let Some(points) = invaders.kill_invader_at(shot.x, shot.y) {
                    killed_points.push(points);
                    shot.explode();
                } else if barriers.hit_at(shot.x, shot.y) {
                    shot.explode();
                }
            }
        }
        killed_points
    }

    pub fn check_enemy_hits(
        &mut self,
        enemy_shots: &mut Vec<EnemyShot>,
        barriers: &mut Barriers,
    ) -> bool {
        for shot in enemy_shots.iter_mut() {
            if !shot.exploding {
                // Check barrier hits
                if barriers.hit_at(shot.x, shot.y) {
                    shot.explode();
                    continue;
                }
                // Check player hit
                if shot.x >= self.x.saturating_sub(1)
                    && shot.x <= self.x + 1
                    && shot.y == self.y
                {
                    shot.explode();
                    if self.shield_active || self.is_invincible {
                        continue;
                    }
                    self.lives = self.lives.saturating_sub(1);
                    self.is_invincible = true;
                    self.invincible_timer = Timer::from_millis(2000);
                    return true;
                }
            }
        }
        false
    }

    pub fn add_powerup(&mut self, power_type: PowerUpType) {
        match power_type {
            PowerUpType::ExtraLife => {
                self.lives = (self.lives + 1).min(5);
            }
            PowerUpType::Bomb => {
                // Handled externally
            }
            _ => {
                self.active_powerups
                    .retain(|p| p.power_type != power_type);
                self.active_powerups.push(ActivePowerUp::new(power_type));
            }
        }
    }

    pub fn has_powerup(&self, power_type: PowerUpType) -> bool {
        self.active_powerups
            .iter()
            .any(|p| p.power_type == power_type)
    }

    pub fn is_dead(&self) -> bool {
        self.lives == 0
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        // Don't draw if invincible and blinking
        if self.is_invincible {
            // Blink effect using a simple time-based toggle
            // We approximate by checking timer progress
            let progress = self.invincible_timer.time_left.as_millis() % 200;
            if progress < 100 {
                return;
            }
        }

        let player_color = if self.shield_active {
            Color::Cyan
        } else if self.has_powerup(PowerUpType::RapidFire) {
            Color::Yellow
        } else if self.has_powerup(PowerUpType::TripleShot) {
            Color::Magenta
        } else {
            Color::Green
        };

        // Draw player ship (wider, cooler shape)
        frame[self.x][self.y] = Cell::new('A', player_color);
        if self.x > 0 {
            frame[self.x - 1][self.y] = Cell::new('<', player_color);
        }
        if self.x + 1 < NUM_COLS {
            frame[self.x + 1][self.y] = Cell::new('>', player_color);
        }
        // Engine glow
        frame[self.x][self.y.min(NUM_ROWS - 1)] = Cell::new('A', player_color);

        // Shield visual
        if self.shield_active && self.y > 0 {
            if self.x > 1 {
                frame[self.x - 2][self.y] = Cell::new('(', Color::Cyan);
            }
            if self.x + 2 < NUM_COLS {
                frame[self.x + 2][self.y] = Cell::new(')', Color::Cyan);
            }
            frame[self.x][self.y - 1] = Cell::new('^', Color::Cyan);
        }

        // Draw shots
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
