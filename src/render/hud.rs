use crossterm::style::Color;

use crate::frame::{Cell, Frame};
use crate::game::score::Score;
use crate::render::color::Theme;
use crate::NUM_COLS;

pub fn draw_hud(
    frame: &mut Frame,
    score: &Score,
    lives: u8,
    level: u32,
    theme: &Theme,
) {
    let y = 0;

    // Score
    let score_text = format!("SCORE:{}", score.value);
    draw_text(frame, 1, y, &score_text, theme.score);

    // Combo
    if score.combo_multiplier > 1 {
        let combo_text = format!("x{} COMBO!", score.combo_multiplier);
        draw_text(frame, 15, y, &combo_text, theme.combo);
    }

    // Level
    let level_text = format!("LVL:{}", level);
    draw_text(frame, NUM_COLS / 2 - 3, y, &level_text, theme.level);

    // Lives
    let lives_text = format!("LIVES:{}", "♥".repeat(lives as usize));
    let lives_x = NUM_COLS.saturating_sub(lives_text.len() + 1);
    draw_text(frame, lives_x, y, &lives_text, theme.lives);

    // High score
    if score.high_score > 0 && score.high_score > score.value {
        let hi_text = format!("HI:{}", score.high_score);
        draw_text(frame, NUM_COLS / 2 - 5, y, &hi_text, Color::DarkGrey);
    }

    // Bottom border
    let bottom_y = frame[0].len() - 1;
    for x in 0..NUM_COLS {
        frame[x][bottom_y] = Cell::new('=', theme.border);
    }
}

pub fn draw_text(frame: &mut Frame, x: usize, y: usize, text: &str, color: Color) {
    for (i, ch) in text.chars().enumerate() {
        let px = x + i;
        if px < frame.len() && y < frame[0].len() {
            frame[px][y] = Cell::new(ch, color);
        }
    }
}
