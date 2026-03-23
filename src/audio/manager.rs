use rusty_audio::Audio;

pub struct AudioManager {
    audio: Audio,
}

impl AudioManager {
    pub fn new() -> Self {
        let mut audio = Audio::new();
        audio.add("explode", "explode.wav");
        audio.add("lose", "lose.wav");
        audio.add("move", "move.wav");
        audio.add("pew", "pew.wav");
        audio.add("startup", "startup.wav");
        audio.add("win", "win.wav");
        Self { audio }
    }

    pub fn play_startup(&mut self) {
        self.audio.play("startup");
    }

    pub fn play_pew(&mut self) {
        self.audio.play("pew");
    }

    pub fn play_explode(&mut self) {
        self.audio.play("explode");
    }

    pub fn play_move(&mut self) {
        self.audio.play("move");
    }

    pub fn play_win(&mut self) {
        self.audio.play("win");
    }

    pub fn play_lose(&mut self) {
        self.audio.play("lose");
    }

    pub fn wait(&self) {
        self.audio.wait();
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
