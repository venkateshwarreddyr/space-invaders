use std::cmp::max;
use std::time::Duration;

use rand::Rng;
use rusty_time::timer::Timer;

use crate::entities::enemy_shot::EnemyShot;
use crate::entities::invader::{Invader, InvaderType};
use crate::frame::{Cell, Drawable};
use crate::game::level::LevelConfig;
use crate::{NUM_COLS, NUM_ROWS};

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
    enemy_shots: Vec<EnemyShot>,
    shot_chance: f64,
    max_shots: usize,
}

impl Invaders {
    pub fn new() -> Self {
        Self::from_level(&LevelConfig::new(1))
    }

    pub fn from_level(config: &LevelConfig) -> Self {
        let mut army = Vec::new();
        let rows = config.invader_rows.min(10);

        for x in 0..NUM_COLS {
            for row in 0..rows {
                let y = row + 2; // Start from row 2
                if x > 2 && x < NUM_COLS - 3 && x % 2 == 0 {
                    let invader_type = InvaderType::from_row(row);
                    army.push(Invader::new(x, y, invader_type));
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(config.invader_speed_ms),
            direction: 1,
            enemy_shots: Vec::new(),
            shot_chance: config.enemy_shot_chance,
            max_shots: config.max_enemy_shots,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        // Update enemy shots
        for shot in self.enemy_shots.iter_mut() {
            shot.update(delta);
        }
        self.enemy_shots.retain(|shot| !shot.dead());

        // Enemy shooting
        self.try_shoot();

        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;

            if self.direction == -1 {
                let min_x = self.army.iter().map(|inv| inv.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|inv| inv.x).max().unwrap_or(0);
                if max_x >= NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 200, 200);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

    fn try_shoot(&mut self) {
        if self.army.is_empty() || self.enemy_shots.len() >= self.max_shots {
            return;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.shot_chance {
            // Pick a random invader from the bottom row of each column
            let mut bottom_invaders: Vec<(usize, usize)> = Vec::new();
            for invader in &self.army {
                let dominated = self
                    .army
                    .iter()
                    .any(|other| other.x == invader.x && other.y > invader.y);
                if !dominated {
                    bottom_invaders.push((invader.x, invader.y));
                }
            }

            if !bottom_invaders.is_empty() {
                let idx = rng.gen_range(0..bottom_invaders.len());
                let (x, y) = bottom_invaders[idx];
                self.enemy_shots.push(EnemyShot::new(x, y + 1));
            }
        }
    }

    pub fn enemy_shots_mut(&mut self) -> &mut Vec<EnemyShot> {
        &mut self.enemy_shots
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn bottom_reached(&self) -> bool {
        self.army
            .iter()
            .map(|invader| invader.y)
            .max()
            .unwrap_or(0)
            >= NUM_ROWS - 3
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> Option<u64> {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| invader.x == x && invader.y == y)
        {
            let points = self.army[idx].invader_type.points();
            self.army.remove(idx);
            Some(points)
        } else {
            None
        }
    }

    pub fn kill_all(&mut self) -> u64 {
        let points: u64 = self.army.iter().map(|inv| inv.invader_type.points()).sum();
        self.army.clear();
        points
    }

    pub fn count(&self) -> usize {
        self.army.len()
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        let anim_phase = self.move_timer.time_left.as_secs_f32()
            / self.move_timer.duration.as_secs_f32();

        for invader in self.army.iter() {
            let (ch1, ch2) = invader.invader_type.chars();
            let ch = if anim_phase > 0.5 { ch1 } else { ch2 };
            let color = invader.invader_type.color();
            frame[invader.x][invader.y] = Cell::new(ch, color);
        }

        // Draw enemy shots
        for shot in &self.enemy_shots {
            shot.draw(frame);
        }
    }
}
