# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Build And Run Commands

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Run game
cargo run

# Run release
cargo run --release

# Check for errors without building
cargo check

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Lint
cargo clippy
```

## Architecture Overview

Bevy 0.18.1 ECS-based Breakout clone. The game uses a 960x540 virtual playfield, scales the camera to the real window size, and persists settings in `config.json`.

### Module Structure

```text
src/
|-- main.rs              # App::new().add_plugins(AppPlugins).run()
|-- app/
|   |-- plugins.rs       # AppPlugins registers top-level plugins and window setup
|   `-- states.rs        # GameState enum (MainMenu, Playing, Paused, GameOver, Settings)
|-- core/
|   |-- camera.rs        # Camera2d setup and virtual resolution scaling
|   `-- config.rs        # GameConfig load/save via serde_json
|-- gameplay/
|   |-- plugin.rs        # GameplayPlugin registers systems and resources
|   |-- components/      # Ball, Paddle, Brick, Collider, Velocity
|   |-- resources/       # Lives(u32), Score(u32)
|   |-- systems/
|   |   |-- inputs.rs    # paddle_input, game_pause
|   |   |-- movement.rs  # ball_movement, paddle_movement
|   |   `-- collision/   # wall, paddle, bricks, death
|   `-- spawn/
|       |-- level.rs     # setup_level() for brick layout
|       `-- mod.rs       # spawn_game(), cleanup_game(), reset_game_resources()
|-- ui/
|   |-- plugin.rs        # UiPlugin
|   `-- screens/         # main_menu, pause, settings, game_over
|-- utils/
|   `-- math.rs          # AABB collision helper
|-- spawn/               # legacy placeholder modules
`-- systems/             # legacy placeholder modules
```

### State Machine

```text
MainMenu -- Enter --> Playing -- Esc --> Paused -- Esc --> Playing
    ^                       |                     |
    |                       |                     `-- Enter --> MainMenu
    `------- Enter ---------+
            |
         0 lives
            v
         GameOver -- Space --> Playing (full reset)
```

## Key Concepts

**Collision detection** - Uses custom AABB overlap checks from `utils/math.rs`. Bounce direction is determined from overlap depth; paddle bounce also adjusts horizontal velocity from the hit offset.

**Brick types** - `BrickType::Normal` takes 1 hit and awards 100 points. `BrickType::Strong` takes 2 hits and awards 200 points.

**Ball death** - When the ball falls below the virtual playfield, `Lives` decreases. At 0 lives, the game enters `GameOver`. Restarting from `GameOver` clears old gameplay entities and resets lives and score.

**Window and camera** - The startup window size is loaded from `config.json`. Gameplay logic uses the virtual resolution constants in `core/camera.rs` rather than raw window dimensions.

**Settings persistence** - `GameConfig` stores `window_width`, `window_height`, `music_volume`, and `sfx_volume`. The settings screen updates the resource and saves it back to `config.json`.

**Assets** - Fonts live in `assets/fonts/`. Sounds live in `assets/sounds/`. The ball texture is in `assets/textures/ball.png`.

## Controls

| Action | Keys |
| --- | --- |
| Move paddle | `A` / `D` or `Left` / `Right` |
| Pause | `Esc` |
| Resume | `Esc` |
| Back to menu from pause | `Enter` |
| Restart from game over | `Space` |
| Navigate menus | `Up` / `Down` |
| Change setting values | `Left` / `Right` |

## Notes

- `src/components/mod.rs` re-exports items from `gameplay/components` for backwards compatibility.
- `core/assets.rs`, `core/time.rs`, `src/spawn/`, and `src/systems/` are still placeholders or legacy scaffolding.
- VS Code debug config in `.vscode/launch.json` uses LLDB.
