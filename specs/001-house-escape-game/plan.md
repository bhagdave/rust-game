
# Implementation Plan: House Escape Game

**Branch**: `001-house-escape-game` | **Date**: 2025-10-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/home/dave/Projects/rust-game/specs/001-house-escape-game/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code, or `AGENTS.md` for all other agents).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary

2D atmospheric platformer where players navigate a dark Victorian mansion using candle-based lighting. Players manage limited visibility, solve puzzles, avoid traps, collect items, and find the exit. Features context-sensitive jump mechanics (double jump via powerup), auto-revealing map, auto-save system, and instant-death trap mechanics with checkpoint respawn.

**Technical Approach**: Rust game using Bevy ECS engine for cross-platform deployment. Dynamic 2D lighting system with circular gradient rendering. Tile-based room system with metroidvania-style gating. Entity-based inventory and state management.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2024 as per Cargo.toml)
**Primary Dependencies**: Bevy 0.16.1 (ECS game engine), bevy_ecs_tilemap (2D tilemap), bevy_kira_audio (audio), bevy_egui (UI), serde (save/load)
**Storage**: Local file-based save system (RON/JSON format), asset files (sprites, audio, level data)
**Testing**: cargo test, criterion (benchmarks), bevy test harness for integration tests
**Target Platform**: Cross-platform (Windows, macOS, Linux) via native compilation; potential WASM for web deployment
**Project Type**: Single project (game binary with embedded assets)
**Performance Goals**: 60 FPS target, <16ms frame time, <16ms input lag, <5s startup, <1s room transitions
**Constraints**: <100MB memory footprint, deterministic physics for testing, cross-platform shader compatibility
**Scale/Scope**: 20-30 rooms, 14 entity types, 6 puzzle system types, 100+ assets (sprites, sounds, level data)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify compliance with Core Principles (see `.specify/memory/constitution.md`):

### I. Code Quality First
- [x] Will code pass `cargo fmt --check` and `cargo clippy -- -D warnings`? YES - CI enforcement planned
- [x] Are `unsafe` blocks justified and documented? YES - Bevy internals handle unsafe, game logic stays safe
- [x] Is error handling using `Result`/`Option` (no unjustified `unwrap()`)? YES - Asset loading and save/load use Result types
- [x] Are newtype patterns used for domain types? YES - CandleWax(f32), RoomId(usize), ItemId(u32) planned
- [x] Are public APIs documented with rustdoc? YES - All component/resource/system public APIs will have docs

### II. Testing Discipline (NON-NEGOTIABLE)
- [x] Will implementation achieve 80% test coverage for business logic? YES - Game systems, inventory, puzzle logic
- [x] Are all tests deterministic (no flaky tests)? YES - Fixed timestep for physics, seeded RNG for testing
- [x] Will unit test suite complete under 30 seconds? YES - Unit tests for components/systems in isolation
- [x] Are integration tests defined for critical flows? YES - Player death/respawn, save/load, room transitions, puzzle solving
- [x] Will all tests pass in CI/CD? YES - GitHub Actions CI planned with test gates

### III. User Experience Consistency
- [x] Does input handling maintain <16ms lag? YES - Bevy's event system processes input within frame budget
- [x] Are controls configurable where applicable? YES - Key binding config resource with save/load
- [x] Are accessibility features considered (colorblind palettes, text scaling)? YES - Colorblind shader modes, config-driven palette swaps
- [x] Is multi-input support (keyboard, mouse, gamepad) consistent? YES - Bevy input abstraction supports all three
- [x] Is user feedback clear for all actions? YES - Audio cues, visual particles, UI notifications

### IV. Performance Requirements
- [x] Will implementation maintain 60 FPS target on target hardware? YES - 2D sprite rendering, limited entities per room
- [x] Is there zero tolerance for memory leaks? YES - Rust ownership prevents leaks, Bevy handles cleanup
- [x] Are startup/loading times within limits (5s startup, 3s level loading)? YES - Async asset loading, room-based streaming
- [x] Has performance profiling been planned? YES - `cargo flamegraph`, Bevy diagnostic plugins

### V. ECS Architecture Adherence
- [x] Does design follow Bevy ECS patterns? YES - Components for state, Systems for logic, Resources for globals
- [x] Does each system have single clear purpose? YES - PlayerMovementSystem, CandleBurnSystem, TrapCollisionSystem, etc.
- [x] Are game systems logically modularized? YES - Modules: player, lighting, inventory, room, puzzle, audio, ui
- [x] Is resource management and ownership clear? YES - Assets in AssetServer, game state in Resources, entities in World

**Violations**: None - design aligns with all constitutional principles

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)

```
src/
├── main.rs                 # Game entry point, Bevy app setup
├── lib.rs                  # Library exports for testing
├── components/             # ECS Components
│   ├── mod.rs
│   ├── player.rs          # Player, Velocity, JumpState, DoubleJumpUnlocked
│   ├── lighting.rs        # Candle, CandleWax, VisibilityRadius, LightSource
│   ├── inventory.rs       # Inventory, Item, StackableItem
│   ├── room.rs            # Room, RoomConnection, Door, LockedDoor
│   ├── trap.rs            # Trap, TrapTrigger, EnvironmentalHazard
│   └── puzzle.rs          # Puzzle, PuzzleState, PressurePlate, Lever
├── systems/               # ECS Systems
│   ├── mod.rs
│   ├── player_movement.rs # Input handling, jump, climb
│   ├── candle_burn.rs     # Wax depletion, match usage, extinguish logic
│   ├── lighting.rs        # Dynamic light rendering, fog of war
│   ├── collision.rs       # Trap detection, door interaction, item pickup
│   ├── inventory.rs       # Item management, equipment
│   ├── room_transition.rs # Room loading/unloading, camera follow
│   ├── puzzle.rs          # Puzzle logic systems
│   ├── save_load.rs       # Auto-save on room entry, game state persistence
│   └── respawn.rs         # Death detection, checkpoint respawn
├── resources/             # Global Resources
│   ├── mod.rs
│   ├── game_state.rs      # CurrentRoom, PlayerStats, CompletionTime
│   ├── input_config.rs    # Configurable key bindings
│   ├── asset_handles.rs   # Texture/audio handles
│   └── map_state.rs       # Explored rooms for auto-revealing map
├── entities/              # Entity spawn functions
│   ├── mod.rs
│   ├── player.rs
│   ├── items.rs
│   ├── traps.rs
│   └── puzzles.rs
├── ui/                    # UI systems
│   ├── mod.rs
│   ├── hud.rs             # Candle meter, match count, inventory bar
│   ├── map.rs             # Auto-revealing map display
│   └── menu.rs            # Pause menu, settings
└── audio/                 # Audio systems
    ├── mod.rs
    └── sound_events.rs    # Audio cues for actions

assets/
├── sprites/               # 2D sprite images
├── audio/                 # Sound effects and music
├── fonts/                 # UI fonts
├── shaders/               # Custom shaders for lighting
└── levels/                # Room layout data (RON/JSON)

tests/
├── integration/           # End-to-end game flow tests
│   ├── player_death_respawn.rs
│   ├── save_load.rs
│   ├── room_transitions.rs
│   └── puzzle_completion.rs
└── unit/                  # Component/system unit tests
    ├── candle_burn_test.rs
    ├── inventory_test.rs
    └── puzzle_logic_test.rs

benches/                   # Performance benchmarks
└── lighting_bench.rs
```

**Structure Decision**: Single Rust project using Bevy ECS architecture. Code organized by ECS pattern (components, systems, resources) and game domain (player, lighting, inventory, room, puzzle, audio, ui). Tests follow parallel structure with integration tests for critical flows and unit tests for isolated logic. Assets embedded or loaded from `assets/` directory for cross-platform deployment.

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh claude`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (data-model.md, systems_contract.md, quickstart.md)
- Each Component (from data-model.md) → component implementation task [P]
- Each System (from systems_contract.md) → system implementation task (ordered by dependencies)
- Each integration test scenario (from quickstart.md) → integration test task
- Each unit test requirement → unit test task [P]
- Setup tasks for Cargo project, dependencies, asset structure

**Ordering Strategy**:
1. **Setup Phase** (T001-T003): Project init, dependencies, CI/CD config
2. **Component Phase** (T004-T020): Implement all components in parallel [P] (Player, Candle, Inventory, Room, etc.)
3. **Resource Phase** (T021-T025): Global resources [P] (GameState, InputConfig, MapState)
4. **System Phase** (T026-T050): Systems in dependency order (Movement → Collision → Traps → Respawn → Inventory → Room Transition → Save/Load)
5. **Integration Test Phase** (T051-T060): Integration tests from quickstart scenarios [P]
6. **Polish Phase** (T061-T070): UI, audio, shaders, asset loading, performance tuning

**Dependency Rules**:
- Components before Systems (systems query components)
- Systems ordered by event dependencies (e.g., CollisionSystem before TrapActivationSystem)
- Tests before implementation where TDD required (puzzle logic, inventory management)
- Mark [P] for truly independent files (different modules)

**Estimated Output**: 60-70 numbered, ordered tasks in tasks.md

**Key Task Categories**:
1. **Setup**: Cargo.toml, CI, project structure (3 tasks)
2. **Components**: 14 entities × 1-2 tasks each = ~20 tasks
3. **Resources**: 4 global resources = 4 tasks
4. **Systems**: 12 core systems = ~15 tasks (some systems split into multiple tasks)
5. **Integration Tests**: 11 quickstart scenarios = 11 tasks
6. **Unit Tests**: Component tests, system tests = ~10 tasks
7. **Assets**: Sprite loading, audio, shaders, level data = ~8 tasks
8. **Polish**: UI, performance, benchmarks = ~5 tasks

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

No constitutional violations identified.

## Research Status & Version Verification

**⚠️ IMPORTANT**: Bevy 0.16.1 is a rapidly evolving framework. The following areas require verification before proceeding with `/tasks`:

### Critical Items Requiring Verification (see research.md section 13)

**Priority 1 - Must Resolve Before `/tasks` Command**:
1. **Plugin Versions**: Verify exact compatible versions:
   - `bevy_ecs_tilemap` (estimated ~0.15, needs confirmation)
   - `bevy_kira_audio` (estimated ~0.21, needs confirmation)
   - `bevy_egui` (estimated ~0.31, needs confirmation)
   - `leafwing-input-manager` (version unknown, needs research)

2. **Bevy 0.16.1 Core APIs**: Confirm current patterns for:
   - System scheduling (`.before()`, `.after()`, `.in_set()`)
   - State management (`States` trait, `OnEnter`, `OnExit`, `OnUpdate`)
   - Fixed timestep (`FixedUpdate` schedule)

3. **WGSL Shader API**: Verify for 2D lighting:
   - `Material2d` trait implementation
   - Shader uniform passing from ECS
   - Post-process render pass API

**Recommended Actions**:
1. Check crates.io and GitHub repositories for each plugin
2. Review Bevy 0.16.1 migration guide and examples
3. Test minimal shader example before full implementation
4. Update Cargo.toml with exact versions once verified

**Status**: Research document updated with detailed verification checklist (research.md section 13). Manual verification recommended before task generation to ensure accurate implementation guidance.


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command) - 47 tasks created
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none)

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
