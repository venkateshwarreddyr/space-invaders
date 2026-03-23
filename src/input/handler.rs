use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    Shoot,
    Pause,
    Quit,
    Confirm,
    None,
}

pub fn poll_action() -> Result<GameAction, Box<dyn std::error::Error>> {
    if event::poll(Duration::default())? {
        if let Event::Key(key_event) = event::read()? {
            return Ok(match key_event.code {
                KeyCode::Left | KeyCode::Char('a') => GameAction::MoveLeft,
                KeyCode::Right | KeyCode::Char('d') => GameAction::MoveRight,
                KeyCode::Char(' ') | KeyCode::Enter => GameAction::Shoot,
                KeyCode::Char('p') => GameAction::Pause,
                KeyCode::Esc | KeyCode::Char('q') => GameAction::Quit,
                _ => GameAction::None,
            });
        }
    }
    Ok(GameAction::None)
}

pub fn drain_actions() -> Result<Vec<GameAction>, Box<dyn std::error::Error>> {
    let mut actions = Vec::new();
    while event::poll(Duration::default())? {
        if let Event::Key(key_event) = event::read()? {
            let action = match key_event.code {
                KeyCode::Left | KeyCode::Char('a') => GameAction::MoveLeft,
                KeyCode::Right | KeyCode::Char('d') => GameAction::MoveRight,
                KeyCode::Char(' ') | KeyCode::Enter => GameAction::Shoot,
                KeyCode::Char('p') => GameAction::Pause,
                KeyCode::Esc | KeyCode::Char('q') => GameAction::Quit,
                _ => GameAction::None,
            };
            if action != GameAction::None {
                actions.push(action);
            }
        }
    }
    actions.dedup();
    Ok(actions)
}
