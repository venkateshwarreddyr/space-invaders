use std::time::Duration;

use rusty_time::timer::Timer;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    LevelUp,
    Paused,
    GameOver,
    Victory,
}

pub struct GameStateManager {
    pub state: GameState,
    pub transition_timer: Timer,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            transition_timer: Timer::from_millis(2000),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.transition_timer.update(delta);
    }

    pub fn transition_to(&mut self, new_state: GameState, delay_ms: u64) {
        self.state = new_state;
        self.transition_timer = Timer::from_millis(delay_ms);
    }

    pub fn transition_ready(&self) -> bool {
        self.transition_timer.ready
    }

    pub fn is_playing(&self) -> bool {
        self.state == GameState::Playing
    }
}

impl Default for GameStateManager {
    fn default() -> Self {
        Self::new()
    }
}
