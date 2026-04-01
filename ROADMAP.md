# ROADMAP

This roadmap turns the current Breakout prototype into a polished learning project for Rust and Bevy, with strong architecture, stronger UI, a 5-level campaign, and an in-game level editor.

## Project Goal

Primary goal:
- Learn Rust and Bevy through a real game project.

Product goal:
- Build a nearly finished mini-game with 5 levels, campaign completion, high scores, music, effects, animated sprites, and an in-game level editor.

Engineering goal:
- Keep the codebase expandable from the start, so new gameplay, UI screens, and editor tools can be added without large rewrites.

## Current State

Already implemented:
- Main menu, pause, settings, and game over screens
- Paddle and ball gameplay loop
- Basic collision logic
- Score and lives resources
- Config persistence in `config.json`
- Ball texture support
- File-based level loading from `assets/levels/level1.json`

Missing for target version:
- 5-level campaign flow
- Win conditions for full campaign
- High score table
- Music and richer sound flow
- Animated sprites and visual effects
- In-game level editor
- Broader UI polish and reusable UI patterns

## Development Principles

- Prefer data-driven systems over hardcoded content.
- Keep gameplay, UI, editor, and shared data separated by module boundaries.
- Use resources for app state and configuration, not hidden globals.
- Build reusable UI pieces instead of one-off screens.
- Make file formats stable before building tools on top of them.
- Avoid temporary shortcuts that will block the editor later.

## Phase 1 - Campaign Foundation

Goal:
- Turn single-level gameplay into a real campaign structure.

Deliverables:
- Campaign level list
- 5 playable level files
- Level completion detection
- Transition to next level
- Final campaign victory state
- Reset flow for new runs

Tasks:
- Add a campaign manifest resource that stores level order.
- Create `assets/levels/level2.json` through `level5.json`.
- Detect "all bricks cleared" and trigger level completion.
- Add a state or transition flow for level completion.
- Advance to the next level without corrupting score/lives flow.
- Add a final victory screen after the last level.
- Show current level index in the HUD.

Definition of done:
- Player can start the game, finish 5 levels in sequence, and reach a victory screen.

## Phase 2 - Data And Architecture Cleanup

Goal:
- Prepare the project structure for long-term growth.

Deliverables:
- Cleaner module boundaries
- Shared data models for levels and scores
- Reduced legacy confusion

Tasks:
- Review and simplify overlap between `core`, `gameplay`, `ui`, and legacy placeholder modules.
- Introduce a dedicated data-oriented area if needed for:
  - level files
  - campaign manifest
  - high scores
  - future editor persistence
- Standardize how files are loaded, validated, and saved.
- Add clear comments and docs only where structure is not obvious.
- Decide which placeholder modules remain and which should be removed later.

Definition of done:
- A new feature can be placed into the project without guesswork about ownership.

## Phase 3 - High Scores And Persistence

Goal:
- Add longer-term player progression and replay value.

Deliverables:
- High score file
- High score resource and serialization
- High score UI screen
- Score submission flow

Tasks:
- Define a `high_scores.json` format.
- Store at least:
  - player name or label
  - score
  - date/time or run identifier
  - optional completion time
- Add load/save logic with safe fallback if the file is missing or broken.
- Add a menu entry for high scores.
- Show high scores after campaign completion or game over.
- Decide whether scores are tracked per run, per campaign, or both.

Definition of done:
- Finishing runs updates a persistent score table visible from the UI.

## Phase 4 - UI System Upgrade

Goal:
- Make UI one of the strengths of the project.

Deliverables:
- Consistent visual direction across all screens
- More reusable UI building blocks
- Better navigation and feedback

Tasks:
- Redesign the main menu layout and navigation flow.
- Add a dedicated victory screen.
- Improve game over, pause, and settings visuals.
- Build a consistent HUD with:
  - score
  - lives
  - current level
  - best score or run info
- Create reusable UI helpers/components for text blocks, selection states, and layout sections.
- Add a visible menu entry for the future level editor.
- Improve keyboard navigation consistency on every screen.

Definition of done:
- All major screens feel like part of one game, not separate prototypes.

## Phase 5 - Audio And Juice

Goal:
- Add life and feedback to the game.

Deliverables:
- Background music
- Stronger event-based sound flow
- Destruction and collision effects
- Basic animated sprites

Tasks:
- Add menu music and gameplay music.
- Respect `music_volume` and `sfx_volume` through a clearer audio flow.
- Add sounds for:
  - menu actions
  - level complete
  - campaign victory
  - game over
- Add visual effects for:
  - brick destruction
  - ball impact
  - transitions between screens
- Add animated or state-based sprites where valuable:
  - ball
  - bricks
  - decorative UI or background

Definition of done:
- The game feels responsive and lively even without adding new mechanics.

## Phase 6 - In-Game Level Editor

Goal:
- Build an editor inside the game, accessible from the main menu.

Deliverables:
- Editor menu entry
- Separate editor app state
- Grid editing tools
- Load/save level files
- Playtest flow from the editor

Tasks:
- Add an `Editor` state to the app flow.
- Build an editor screen with a visible grid.
- Support painting cells with:
  - normal brick
  - strong brick
  - empty cell
- Support save and load for level JSON files.
- Show the current tool and file name in the UI.
- Add quick validation feedback for invalid level data.
- Add "play this level" from the editor.
- Decide whether editor changes target campaign levels, custom levels, or both.

Definition of done:
- A level can be created, edited, saved, loaded, and tested without leaving the game.

## Phase 7 - Polish And Release Prep

Goal:
- Make the project presentation-ready.

Deliverables:
- Cleaner docs
- Reduced warnings and technical debt
- Better balancing
- Stronger presentation for portfolio or publishing

Tasks:
- Review gameplay balance across 5 levels.
- Clean warnings that no longer make sense.
- Add a few targeted tests around data loading and validation.
- Improve README with screenshots and project overview.
- Prepare a short changelog or milestone notes.
- Package a release build and verify startup assets/config behavior.

Definition of done:
- The project is comfortable to show publicly as a serious learning game project.

## Recommended Order For The Next Few Weeks

1. Finish campaign flow.
2. Add 5 level files and campaign completion.
3. Add victory screen and HUD level display.
4. Add high scores and persistence.
5. Upgrade UI system and main menu structure.
6. Add music and visual effects.
7. Build the in-game level editor on top of the stabilized data model.

## Immediate Backlog

Top priority:
- Add campaign manifest and level progression resource
- Create level completion detection
- Add `Victory` state and screen
- Create levels 2-5
- Show current level in HUD

Next priority:
- Define `high_scores.json`
- Add high score loading and saving
- Add high score screen to main menu
- Refactor UI for reusable screen patterns
- Add menu entry for `Level Editor`

Later priority:
- Audio controller and playback structure
- Visual effects for brick destruction
- Animated sprites
- Editor state and editor UI
- Playtest-from-editor flow

## Suggested First Implementation Slice

If we start coding immediately, the best first slice is:

1. Add a campaign manifest resource.
2. Support multiple level files.
3. Detect level completion when no bricks remain.
4. Advance to the next level.
5. Add final campaign victory screen.

Why this slice:
- It moves the project from prototype to real game structure.
- It supports your learning goals in Rust and Bevy.
- It creates a stable base for high scores, UI work, and the future editor.

## Success Criteria

The roadmap is successful when:
- The project teaches Rust and Bevy through real architectural decisions.
- The game is enjoyable from start to finish across 5 levels.
- UI feels intentional and coherent.
- New levels can be authored without code changes.
- The editor fits naturally into the same data model as gameplay.
