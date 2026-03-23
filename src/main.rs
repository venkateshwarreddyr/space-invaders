use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use rand::Rng;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

use invaders::audio::manager::AudioManager;
use invaders::effects::particles::ParticleSystem;
use invaders::effects::stars::Starfield;
use invaders::entities::barrier::Barriers;
use invaders::entities::invaders::Invaders;
use invaders::entities::player::Player;
use invaders::entities::powerup::{PowerUp, PowerUpType};
use invaders::frame::{new_frame, Drawable};
use invaders::game::level::LevelConfig;
use invaders::game::score::Score;
use invaders::game::state::{GameState, GameStateManager};
use invaders::input::handler::{drain_actions, GameAction};
use invaders::render::color::Theme;
use invaders::render::hud::draw_hud;
use invaders::render::menu::{draw_game_over, draw_level_up, draw_menu, draw_paused, draw_victory};
use invaders::render::renderer;
use invaders::NUM_ROWS;

fn main() -> Result<(), Box<dyn Error>> {
    // Audio
    let mut audio = AudioManager::new();
    audio.play_startup();
    audio.wait();

    // Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        renderer::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            renderer::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game state
    let theme = Theme::neon();
    let mut state_mgr = GameStateManager::new();
    let mut score = Score::new();
    let mut level_config = LevelConfig::new(1);
    let mut player = Player::new();
    let mut invaders = Invaders::from_level(&level_config);
    let mut barriers = Barriers::new(NUM_ROWS - 5);
    let mut particles = ParticleSystem::new();
    let mut starfield = Starfield::new();
    let mut powerups: Vec<PowerUp> = Vec::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        let actions = drain_actions()?;

        match state_mgr.state {
            GameState::Menu => {
                starfield.update(delta);
                starfield.draw(&mut curr_frame);
                draw_menu(&mut curr_frame, &theme);

                for action in &actions {
                    match action {
                        GameAction::Shoot | GameAction::Confirm => {
                            state_mgr.transition_to(GameState::Playing, 0);
                            player = Player::new();
                            score.reset();
                            level_config = LevelConfig::new(1);
                            invaders = Invaders::from_level(&level_config);
                            barriers = Barriers::new(NUM_ROWS - 5);
                            powerups.clear();
                        }
                        GameAction::Quit => break 'gameloop,
                        _ => {}
                    }
                }
            }

            GameState::Playing => {
                // Process input
                for action in &actions {
                    match action {
                        GameAction::MoveLeft => player.move_left(),
                        GameAction::MoveRight => player.move_right(),
                        GameAction::Shoot => {
                            if player.shoot() {
                                audio.play_pew();
                            }
                        }
                        GameAction::Pause => {
                            state_mgr.transition_to(GameState::Paused, 0);
                        }
                        GameAction::Quit => {
                            audio.play_lose();
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }

                // Update game entities
                player.update(delta);
                score.update(delta);
                starfield.update(delta);
                particles.update(delta);

                if invaders.update(delta) {
                    audio.play_move();
                }

                // Player shots hit invaders/barriers
                let killed = player.detect_hits(&mut invaders, &mut barriers);
                for points in &killed {
                    score.add_kill(*points);
                    audio.play_explode();
                }

                // Spawn particles for kills
                for shot in &player.shots {
                    if shot.exploding {
                        particles.spawn_explosion(shot.x, shot.y, crossterm::style::Color::Yellow);
                    }
                }

                // Enemy shots hit player/barriers
                let enemy_shots = invaders.enemy_shots_mut();
                if player.check_enemy_hits(enemy_shots, &mut barriers) {
                    audio.play_explode();
                    particles.spawn_big_explosion(player.x, player.y);
                    if player.is_dead() {
                        state_mgr.transition_to(GameState::GameOver, 2000);
                        audio.play_lose();
                    }
                }
                enemy_shots.retain(|s| !s.dead());

                // Update & check powerups
                update_powerups(
                    &mut powerups,
                    &mut player,
                    &mut invaders,
                    &mut particles,
                    &mut score,
                    &mut audio,
                    delta,
                );

                // Spawn powerups on kills
                if !killed.is_empty() {
                    maybe_spawn_powerup(&mut powerups, &level_config, &player);
                }

                // Draw everything (order matters: background first)
                starfield.draw(&mut curr_frame);
                barriers.draw(&mut curr_frame);
                invaders.draw(&mut curr_frame);
                for powerup in &powerups {
                    powerup.draw(&mut curr_frame);
                }
                player.draw(&mut curr_frame);
                particles.draw(&mut curr_frame);
                draw_hud(
                    &mut curr_frame,
                    &score,
                    player.lives,
                    level_config.level,
                    &theme,
                );

                // Win/lose checks
                if invaders.all_killed() {
                    audio.play_win();
                    state_mgr.transition_to(GameState::Victory, 2000);
                }
                if invaders.bottom_reached() {
                    audio.play_lose();
                    state_mgr.transition_to(GameState::GameOver, 2000);
                }
            }

            GameState::Paused => {
                starfield.draw(&mut curr_frame);
                draw_paused(&mut curr_frame, &theme);

                for action in &actions {
                    match action {
                        GameAction::Pause => {
                            state_mgr.transition_to(GameState::Playing, 0);
                        }
                        GameAction::Quit => break 'gameloop,
                        _ => {}
                    }
                }
            }

            GameState::Victory => {
                state_mgr.update(delta);
                starfield.update(delta);
                particles.update(delta);
                starfield.draw(&mut curr_frame);
                particles.draw(&mut curr_frame);
                draw_victory(
                    &mut curr_frame,
                    score.value,
                    level_config.level,
                    &theme,
                );

                if state_mgr.transition_ready() {
                    for action in &actions {
                        match action {
                            GameAction::Shoot | GameAction::Confirm => {
                                // Next level
                                level_config = level_config.next_level();
                                invaders = Invaders::from_level(&level_config);
                                barriers = Barriers::new(NUM_ROWS - 5);
                                powerups.clear();
                                player.reset_position();
                                state_mgr.transition_to(GameState::LevelUp, 1500);
                            }
                            GameAction::Quit => break 'gameloop,
                            _ => {}
                        }
                    }
                }
            }

            GameState::LevelUp => {
                state_mgr.update(delta);
                starfield.update(delta);
                starfield.draw(&mut curr_frame);
                draw_level_up(&mut curr_frame, level_config.level, &theme);

                if state_mgr.transition_ready() {
                    state_mgr.transition_to(GameState::Playing, 0);
                }
            }

            GameState::GameOver => {
                state_mgr.update(delta);
                starfield.update(delta);
                particles.update(delta);
                starfield.draw(&mut curr_frame);
                particles.draw(&mut curr_frame);
                draw_game_over(&mut curr_frame, score.value, level_config.level, &theme);

                if state_mgr.transition_ready() {
                    for action in &actions {
                        match action {
                            GameAction::Shoot | GameAction::Confirm => {
                                state_mgr.transition_to(GameState::Menu, 0);
                            }
                            GameAction::Quit => break 'gameloop,
                            _ => {}
                        }
                    }
                }
            }
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn update_powerups(
    powerups: &mut Vec<PowerUp>,
    player: &mut Player,
    invaders: &mut Invaders,
    particles: &mut ParticleSystem,
    score: &mut Score,
    audio: &mut AudioManager,
    delta: Duration,
) {
    for powerup in powerups.iter_mut() {
        powerup.update(delta);
    }

    // Check collection
    let player_x = player.x;
    let player_y = player.y;
    let mut collected = Vec::new();

    for (i, powerup) in powerups.iter().enumerate() {
        if powerup.x >= player_x.saturating_sub(1)
            && powerup.x <= player_x + 1
            && powerup.y == player_y
        {
            collected.push(i);
        }
    }

    for &i in collected.iter().rev() {
        let powerup = &powerups[i];
        let ptype = powerup.power_type;
        let px = powerup.x;
        let py = powerup.y;
        particles.spawn_powerup_collect(px, py, ptype.color());

        match ptype {
            PowerUpType::Bomb => {
                let bonus = invaders.kill_all();
                score.value += bonus;
                audio.play_explode();
                // Big explosion at center
                particles.spawn_big_explosion(30, 10);
            }
            _ => {
                player.add_powerup(ptype);
            }
        }

        powerups.remove(i);
    }

    powerups.retain(|p| !p.dead());
}

fn maybe_spawn_powerup(powerups: &mut Vec<PowerUp>, config: &LevelConfig, player: &Player) {
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < config.powerup_chance && powerups.len() < 3 {
        let x = rng.gen_range(3..invaders::NUM_COLS - 3);
        let y = 3;
        let power_type = PowerUpType::from_index(rng.gen_range(0..5));

        // Don't spawn extra life if already at max
        if power_type == PowerUpType::ExtraLife && player.lives >= 5 {
            return;
        }

        powerups.push(PowerUp::new(x, y, power_type));
    }
}
