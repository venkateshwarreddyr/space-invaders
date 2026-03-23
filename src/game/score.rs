use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Score {
    pub value: u64,
    pub combo: u32,
    pub combo_multiplier: u32,
    combo_timer: Timer,
    pub high_score: u64,
}

impl Score {
    pub fn new() -> Self {
        Self {
            value: 0,
            combo: 0,
            combo_multiplier: 1,
            combo_timer: Timer::from_millis(2000),
            high_score: 0,
        }
    }

    pub fn add_kill(&mut self, base_points: u64) {
        self.combo += 1;
        self.combo_multiplier = match self.combo {
            0..=2 => 1,
            3..=5 => 2,
            6..=9 => 3,
            10..=14 => 5,
            _ => 10,
        };
        let points = base_points * self.combo_multiplier as u64;
        self.value += points;
        if self.value > self.high_score {
            self.high_score = self.value;
        }
        self.combo_timer = Timer::from_millis(2000);
    }

    pub fn update(&mut self, delta: Duration) {
        self.combo_timer.update(delta);
        if self.combo_timer.ready {
            self.combo = 0;
            self.combo_multiplier = 1;
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.combo = 0;
        self.combo_multiplier = 1;
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}
