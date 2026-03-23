use crossterm::style::Color;

pub struct Theme {
    pub border: Color,
    pub title: Color,
    pub score: Color,
    pub lives: Color,
    pub combo: Color,
    pub level: Color,
    pub menu_highlight: Color,
    pub game_over: Color,
    pub victory: Color,
}

impl Theme {
    pub fn neon() -> Self {
        Self {
            border: Color::DarkCyan,
            title: Color::Cyan,
            score: Color::Yellow,
            lives: Color::Red,
            combo: Color::Magenta,
            level: Color::Green,
            menu_highlight: Color::Cyan,
            game_over: Color::Red,
            victory: Color::Green,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::neon()
    }
}
