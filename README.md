# Space Invaders

Terminal-based Space Invaders built in Rust — runs in your terminal with real audio, threaded rendering, and smooth collision detection.

## Features

- **Threaded rendering** — render loop runs on a separate thread via `mpsc` channel
- **Real audio** — startup, pew, explode, move, win, lose sounds (WAV)
- **Delta-time updates** — frame-rate independent movement for player and invaders
- **Win/lose conditions** — kill all invaders to win; they reach the bottom and you lose
- **Controls**: `←` `→` to move, `Space`/`Enter` to shoot, `Esc`/`q` to quit

## Build & Run

```bash
# Requires Rust toolchain (https://rustup.rs)
cargo run --release
```

## Tech Stack

- **Rust** 2021 edition
- **crossterm** — cross-platform terminal control (raw mode, alternate screen, cursor hiding)
- **rusty_audio** — WAV audio playback
- **rusty_time** — timer utilities for delta-time

## Project Structure

```
src/
├── main.rs       # Game loop, input handling, audio, thread setup
├── lib.rs        # Module declarations
├── player.rs     # Player movement and shooting
├── invaders.rs   # Invader grid, movement patterns, update logic
├── shot.rs       # Projectile logic and collision
├── frame.rs      # Frame buffer, Drawable trait
└── render.rs     # Diff-based terminal renderer
*.wav             # Sound assets (startup, pew, explode, move, win, lose)
```
