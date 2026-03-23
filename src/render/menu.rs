use crossterm::style::Color;

use crate::frame::Frame;
use crate::render::color::Theme;
use crate::render::hud::draw_text;
use crate::NUM_COLS;

pub fn draw_menu(frame: &mut Frame, theme: &Theme) {
    let cx = NUM_COLS / 2;
    let art = [
        r"  _____ ___   _   ___ ___  ",
        r" / __| _ \ / \ / __| __| ",
        r" \__ \  _/ /_\ \ (__| _|  ",
        r" |___/_| /_/ \_\___|___| ",
        r"                          ",
        r"  ___ _  ___   _____ ___  ___  ___  ___  ",
        r" |_ _| \| \ \ / / _ | __|  _ \/ __| ",
        r"  | || .` |\ V /| _ | |) | __| _ \__ \ ",
        r" |___|_|\_| \_/ |_| |___/|___|_| \___/ ",
    ];

    let start_y = 5;
    for (i, line) in art.iter().enumerate() {
        let x = cx.saturating_sub(line.len() / 2);
        draw_text(frame, x, start_y + i, line, theme.title);
    }

    let instructions = [
        ("ARROW KEYS", "Move"),
        ("SPACE/ENTER", "Shoot"),
        ("P", "Pause"),
        ("ESC/Q", "Quit"),
    ];

    let inst_y = start_y + art.len() + 3;
    for (i, (key, action)) in instructions.iter().enumerate() {
        let text = format!("[{}] {}", key, action);
        let x = cx.saturating_sub(text.len() / 2);
        draw_text(frame, x, inst_y + i, &text, Color::Grey);
    }

    let prompt = ">>> PRESS SPACE TO START <<<";
    let x = cx.saturating_sub(prompt.len() / 2);
    draw_text(frame, x, inst_y + instructions.len() + 2, prompt, theme.menu_highlight);

    let powerup_info = [
        ("[R] Rapid Fire  [T] Triple Shot  [S] Shield",),
        ("[L] Extra Life  [B] Screen Bomb",),
    ];
    let pu_y = inst_y + instructions.len() + 5;
    for (i, line) in powerup_info.iter().enumerate() {
        let x = cx.saturating_sub(line.0.len() / 2);
        draw_text(frame, x, pu_y + i, line.0, Color::DarkYellow);
    }
}

pub fn draw_game_over(frame: &mut Frame, score: u64, level: u32, theme: &Theme) {
    let cx = NUM_COLS / 2;
    let art = [
        r"  ___   _   __  __ ___  ",
        r" / __| /_\ |  \/  | __| ",
        r"| (_ |/ _ \| |\/| | _|  ",
        r" \___/_/ \_|_|  |_|___| ",
        r"                        ",
        r"  _____   _____ ___  ",
        r" / _ \ \ / / __| _ \ ",
        r"| (_) \ V /| _||   / ",
        r" \___/ \_/ |___|_|_\ ",
    ];

    let start_y = 6;
    for (i, line) in art.iter().enumerate() {
        let x = cx.saturating_sub(line.len() / 2);
        draw_text(frame, x, start_y + i, line, theme.game_over);
    }

    let score_text = format!("FINAL SCORE: {}", score);
    let x = cx.saturating_sub(score_text.len() / 2);
    draw_text(frame, x, start_y + art.len() + 2, &score_text, Color::Yellow);

    let level_text = format!("LEVEL REACHED: {}", level);
    let x = cx.saturating_sub(level_text.len() / 2);
    draw_text(frame, x, start_y + art.len() + 4, &level_text, Color::Green);

    let prompt = "PRESS SPACE TO RESTART | ESC TO QUIT";
    let x = cx.saturating_sub(prompt.len() / 2);
    draw_text(frame, x, start_y + art.len() + 7, prompt, Color::Grey);
}

pub fn draw_victory(frame: &mut Frame, score: u64, level: u32, theme: &Theme) {
    let cx = NUM_COLS / 2;
    let art = [
        r" __   _____ ___ _____ ___  _____   ___",
        r" \ \ / /_ _/ __|_   _/ _ \| _ \ \ / /",
        r"  \ V / | | (__  | || (_) |   /\ V / ",
        r"   \_/ |___\___| |_| \___/|_|_\ |_|  ",
    ];

    let start_y = 8;
    for (i, line) in art.iter().enumerate() {
        let x = cx.saturating_sub(line.len() / 2);
        draw_text(frame, x, start_y + i, line, theme.victory);
    }

    let score_text = format!("SCORE: {}", score);
    let x = cx.saturating_sub(score_text.len() / 2);
    draw_text(frame, x, start_y + art.len() + 2, &score_text, Color::Yellow);

    let level_text = format!("LEVEL: {}", level);
    let x = cx.saturating_sub(level_text.len() / 2);
    draw_text(frame, x, start_y + art.len() + 4, &level_text, Color::Cyan);

    let prompt = "PRESS SPACE TO CONTINUE | ESC TO QUIT";
    let x = cx.saturating_sub(prompt.len() / 2);
    draw_text(frame, x, start_y + art.len() + 7, prompt, Color::Grey);
}

pub fn draw_level_up(frame: &mut Frame, level: u32, theme: &Theme) {
    let cx = NUM_COLS / 2;

    let text = format!("=== LEVEL {} ===", level);
    let x = cx.saturating_sub(text.len() / 2);
    draw_text(frame, x, 12, &text, theme.level);

    let sub = "GET READY!";
    let x = cx.saturating_sub(sub.len() / 2);
    draw_text(frame, x, 14, sub, Color::Yellow);
}

pub fn draw_paused(frame: &mut Frame, theme: &Theme) {
    let cx = NUM_COLS / 2;

    let text = "=== PAUSED ===";
    let x = cx.saturating_sub(text.len() / 2);
    draw_text(frame, x, 13, text, theme.title);

    let sub = "Press P to resume";
    let x = cx.saturating_sub(sub.len() / 2);
    draw_text(frame, x, 15, sub, Color::Grey);
}
