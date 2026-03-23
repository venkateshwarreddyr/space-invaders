pub struct LevelConfig {
    pub level: u32,
    pub invader_rows: usize,
    pub invader_speed_ms: u64,
    pub enemy_shot_chance: f64,
    pub max_enemy_shots: usize,
    pub powerup_chance: f64,
}

impl LevelConfig {
    pub fn new(level: u32) -> Self {
        let invader_rows = (4 + level as usize).min(10);
        let invader_speed_ms = (2000u64).saturating_sub(level as u64 * 150).max(400);
        let enemy_shot_chance = (0.002 * level as f64).min(0.03);
        let max_enemy_shots = (2 + level as usize).min(8);
        let powerup_chance = 0.005;

        Self {
            level,
            invader_rows,
            invader_speed_ms,
            enemy_shot_chance,
            max_enemy_shots,
            powerup_chance,
        }
    }

    pub fn next_level(&self) -> Self {
        Self::new(self.level + 1)
    }
}

impl Default for LevelConfig {
    fn default() -> Self {
        Self::new(1)
    }
}
