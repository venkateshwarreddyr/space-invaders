use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum InvaderType {
    Grunt,
    Soldier,
    Elite,
    Commander,
}

impl InvaderType {
    pub fn points(&self) -> u64 {
        match self {
            InvaderType::Grunt => 10,
            InvaderType::Soldier => 20,
            InvaderType::Elite => 40,
            InvaderType::Commander => 80,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            InvaderType::Grunt => Color::Green,
            InvaderType::Soldier => Color::Yellow,
            InvaderType::Elite => Color::Magenta,
            InvaderType::Commander => Color::Red,
        }
    }

    pub fn chars(&self) -> (char, char) {
        match self {
            InvaderType::Grunt => ('W', 'M'),
            InvaderType::Soldier => ('X', '+'),
            InvaderType::Elite => ('#', '%'),
            InvaderType::Commander => ('@', '&'),
        }
    }

    pub fn from_row(row: usize) -> Self {
        match row {
            0 => InvaderType::Commander,
            1 => InvaderType::Elite,
            2..=3 => InvaderType::Soldier,
            _ => InvaderType::Grunt,
        }
    }
}

pub struct Invader {
    pub x: usize,
    pub y: usize,
    pub invader_type: InvaderType,
}

impl Invader {
    pub fn new(x: usize, y: usize, invader_type: InvaderType) -> Self {
        Self { x, y, invader_type }
    }
}
