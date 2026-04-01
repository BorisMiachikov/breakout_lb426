# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Build & Run Commands

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

Bevy 0.18.1 ECS-based Breakout clone. Window: 960×540. Config persisted to `config.json`.

### Module Structure

```
src/
├── main.rs              # App::new().add_plugins(AppPlugins).run()
├── app/
│   ├── plugins.rs       # AppPlugins — registers all top-level plugins
│   └── states.rs        # GameState enum (MainMenu, Playing, Paused, GameOver, Settings)
├── core/
│   ├── config.rs        # GameConfig resource + ConfigPlugin (load/save config.json via serde_json)
│   └── camera.rs        # Camera2d setup
├── gameplay/            # All game logic
│   ├── plugin.rs        # GameplayPlugin — registers systems and resources
│   ├── components/      # Ball, Paddle, Brick, Collider, Velocity
│   ├── resources/       # Lives(u32), Score(u32)
│   ├── systems/
│   │   ├── inputs.rs    # paddle_input, game_pause
│   │   ├── movement.rs  # ball_movement, paddle_movement
│   │   └── collision/   # wall, paddle, bricks, death
│   └── spawn/
│       └── level.rs     # setup_level() — 5 rows × 10 columns of bricks
└── ui/
    ├── plugin.rs         # UiPlugin
    └── screens/          # main_menu, pause, settings, game_over
```

### State Machine

```
MainMenu ──ENTER──► Playing ──ESC──► Paused ──ESC──► Playing
   ▲                  │                │
   └──────ENTER───────┘                └──ENTER──► MainMenu
              │
           0 lives
              ▼
           GameOver ──SPACE──► Playing (reset)
```

### Key Concepts

**Collision detection** — custom AABB (no Rapier). Each collision module has a local `collide()` helper checking x/y overlap. Bounce direction is determined by overlap axis; paddle bounce angle depends on contact point offset from paddle center.

**Brick types** — `BrickType::Normal` (1 hit, 100 pts) and `BrickType::Strong` (2 hits, 200 pts). Health reaches 0 → entity despawned.

**Ball death** — ball falls below screen → decrement `Lives`; if `Lives == 0` → transition to `GameOver`; otherwise reset ball position.

**Assets** — fonts: `assets/fonts/FiraSans-Bold.ttf`; sounds: `assets/sounds/bounce.*` and `assets/sounds/break.*` (mp3/ogg/wav).

**Settings persistence** — `GameConfig` (window size, music/sfx volume) is loaded from and saved to `config.json` via `ConfigPlugin`.

### Controls

| Action | Keys |
|--------|------|
| Move paddle | A/D or ←/→ |
| Pause | ESC |
| Resume | ESC |
| Back to menu (paused) | ENTER |
| Restart (game over) | SPACE |
| Navigate settings | ←/→ |

## Notes

- Empty stubs exist: `core/assets.rs`, `core/time.rs`, `utils/math.rs`, `src/spawn/`, `src/systems/` — these are placeholders for future features listed in `project.md`.
- `src/components/mod.rs` re-exports from `gameplay/components` for backwards compatibility.
- VS Code debug config in `.vscode/launch.json` uses LLDB.
