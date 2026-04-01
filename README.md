# Breakout LB426

A simple Breakout clone built with Rust and Bevy.

## Features

- Bevy 0.18.1 ECS architecture
- Main menu, pause, settings, and game over screens
- Paddle, ball, wall, brick, and death collision systems
- Two brick types: normal and strong
- Persistent configuration stored in `config.json`
- Window scaling based on a 960x540 virtual playfield

## Requirements

- Rust
- Cargo

## Run

```bash
cargo run
```

## Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

## Checks

```bash
cargo check
cargo test
cargo clippy
```

## Controls

| Action | Keys |
| --- | --- |
| Move paddle | `A` / `D` or `Left` / `Right` |
| Pause game | `Esc` |
| Resume game | `Esc` |
| Return to main menu from pause | `Enter` |
| Restart after game over | `Space` |
| Navigate menu | `Up` / `Down` |
| Change settings values | `Left` / `Right` |

## Project Structure

```text
src/
|-- main.rs
|-- app/
|   |-- plugins.rs
|   `-- states.rs
|-- core/
|   |-- camera.rs
|   `-- config.rs
|-- gameplay/
|   |-- plugin.rs
|   |-- components/
|   |-- resources/
|   |-- spawn/
|   `-- systems/
`-- ui/
    |-- plugin.rs
    `-- screens/
```

## Configuration

The game loads settings from `config.json` on startup and saves updated values from the settings screen.

Current config fields:

- `window_width`
- `window_height`
- `music_volume`
- `sfx_volume`

## Assets

- Font: `assets/fonts/FiraSans-Bold.ttf`
- Sounds: `assets/sounds/bounce.*`, `assets/sounds/break.*`
- Texture: `assets/textures/ball.png`

## Notes

- Some placeholder modules are still present for future expansion.
- There are currently no gameplay tests in the project yet.
