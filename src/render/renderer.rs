use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
    }

    let mut last_fg = Color::White;
    let mut last_bg = Color::Black;

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if *cell != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();

                if cell.fg != last_fg {
                    stdout.queue(SetForegroundColor(cell.fg)).unwrap();
                    last_fg = cell.fg;
                }
                if cell.bg != last_bg {
                    stdout.queue(SetBackgroundColor(cell.bg)).unwrap();
                    last_bg = cell.bg;
                }

                stdout.queue(Print(cell.ch)).unwrap();
            }
        }
    }

    stdout.queue(ResetColor).unwrap();
    stdout.flush().unwrap();
}
