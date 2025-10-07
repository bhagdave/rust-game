
# Implementation Plan: Demo Level on First Run

**Branch**: `002-when-a-developer` | **Date**: 2025-10-06 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/home/dave/Projects/rust-game/specs/002-when-a-developer/spec.md`

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
Create a demo level that automatically loads when the game starts, showcasing the existing game engine's capabilities (player movement, basic interactions with objects, and 2D tilemap rendering). The demo must load within 10 seconds, maintain 30+ FPS, and have <50ms input lag. When assets are missing or corrupted, placeholder graphics must be displayed to allow core functionality testing.

## Technical Context
**Language/Version**: Rust 1.75+ (edition 2024)
**Primary Dependencies**: Bevy 0.16.1 (ECS game engine), bevy_ecs_tilemap 0.16.0 (2D tilemap), bevy_kira_audio 0.23.0 (audio), bevy_egui 0.36.0 (UI), leafwing-input-manager 0.17.0 (input), serde 1.0 + ron 0.8 (serialization)
**Storage**: RON files for level data and game state serialization
**Testing**: cargo test with integration tests, cargo clippy for linting, cargo fmt for formatting
**Target Platform**: Desktop (Linux primary, cross-platform via Bevy)
**Project Type**: single (game binary with library crate structure)
**Performance Goals**: 30 FPS minimum (target 60 FPS per constitution), <10s level loading, <50ms input lag
**Constraints**: Existing game engine with player movement, tilemap rendering, collision, inventory, puzzles, traps, lighting, save/load systems already implemented
**Scale/Scope**: Single demo level showcasing core mechanics - leveraging existing systems from the "House Escape" game

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify compliance with Core Principles (see `.specify/memory/constitution.md`):

### I. Code Quality First
- [x] Will code pass `cargo fmt --check` and `cargo clippy -- -D warnings`? - Yes, all new code follows existing patterns
- [x] Are `unsafe` blocks justified and documented? - N/A, no unsafe code needed
- [x] Is error handling using `Result`/`Option` (no unjustified `unwrap()`)? - Yes, using .unwrap_or_default() for fallback assets
- [x] Are newtype patterns used for domain types? - Yes, existing components follow this pattern
- [x] Are public APIs documented with rustdoc? - Yes, demo level data structures will have rustdoc

### II. Testing Discipline (NON-NEGOTIABLE)
- [x] Will implementation achieve 80% test coverage for business logic? - Yes, tests for level loading, asset fallback, and demo behavior
- [x] Are all tests deterministic (no flaky tests)? - Yes, using Bevy's deterministic test framework
- [x] Will unit test suite complete under 30 seconds? - Yes, simple level data validation tests
- [x] Are integration tests defined for critical flows? - Yes, demo level loading and interaction tests
- [x] Will all tests pass in CI/CD? - Yes, follows existing CI patterns

### III. User Experience Consistency
- [⚠️] Does input handling maintain <16ms lag? - **DEVIATION**: Spec allows <50ms for demo purposes (relaxed from 16ms)
- [x] Are controls configurable where applicable? - Yes, using existing leafwing-input-manager configuration
- [N/A] Are accessibility features considered (colorblind palettes, text scaling)? - Deferred to main game (demo uses existing systems)
- [x] Is multi-input support (keyboard, mouse, gamepad) consistent? - Yes, existing input system already supports this
- [x] Is user feedback clear for all actions? - Yes, demo level shows interactions visually

### IV. Performance Requirements
- [⚠️] Will implementation maintain 60 FPS target on target hardware? - **DEVIATION**: Spec allows 30 FPS minimum for demo (relaxed from 60 FPS)
- [x] Is there zero tolerance for memory leaks? - Yes, Rust's memory safety + Bevy resource management
- [⚠️] Are startup/loading times within limits (5s startup, 3s level loading)? - **DEVIATION**: Spec allows 10s demo load (relaxed from 3s)
- [x] Has performance profiling been planned? - Yes, can use existing lighting benchmark patterns if needed

### V. ECS Architecture Adherence
- [x] Does design follow Bevy ECS patterns? - Yes, reuses existing level_loader system architecture
- [x] Does each system have single clear purpose? - Yes, demo_level_loader handles demo initialization
- [x] Are game systems logically modularized? - Yes, demo level system integrates with existing module structure
- [x] Is resource management and ownership clear? - Yes, follows existing AssetHandles and GameState patterns

**Violations**: Performance requirements relaxed for demo purposes - see Complexity Tracking below

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
├── components/          # ECS components (player, inventory, lighting, etc.)
├── resources/           # Global state (GameState, AssetHandles, InputConfig, MapState)
├── systems/             # Game logic systems
│   ├── demo_level.rs    # NEW: Demo level auto-loading system
│   ├── level_loader.rs  # Existing level loading system (to be extended)
│   ├── player_movement.rs
│   ├── tilemap.rs
│   ├── collision.rs
│   └── ... (other existing systems)
├── audio/               # Sound events
├── ui/                  # HUD components
├── entities/            # Entity spawning utilities
├── main.rs              # Entry point (to add demo plugin)
└── lib.rs               # Library root

tests/
├── demo_level_loading.rs      # NEW: Integration test for demo level
├── demo_asset_fallback.rs     # NEW: Test placeholder graphics behavior
├── demo_performance.rs        # NEW: Verify 30 FPS and load time requirements
└── ... (other existing tests)

assets/                  # NEW: Demo level assets
├── demo_level.ron       # Demo level data file
├── sprites/
│   └── placeholder.png  # Fallback sprite for missing assets
└── tiles/
    └── demo_tileset.png # Demo tilemap tiles
```

**Structure Decision**: Single project structure with Bevy ECS game engine. Demo level implementation adds new system (demo_level.rs), level data file (demo_level.ron), placeholder assets, and integration tests. Leverages existing level_loader system for loading logic.

## Phase 0: Outline & Research ✅

**Research completed** - See [research.md](research.md)

Key decisions:
1. **Auto-loading mechanism**: Bevy `OnEnter(GameState::Loading)` state system
2. **Asset fallback**: `Option<Handle<Image>>` with placeholder sprite (magenta for visibility)
3. **Level format**: RON files matching existing level_loader system
4. **Performance validation**: Bevy diagnostic plugin + custom load timers
5. **Architecture**: `DemoPlugin` integrating with existing game systems

All NEEDS CLARIFICATION items resolved. No new dependencies required - leveraging existing Bevy, bevy_ecs_tilemap, serde/ron stack.

## Phase 1: Design & Contracts ✅

**Design artifacts completed**:

1. **Data Model** → [data-model.md](data-model.md)
   - Core entities: DemoLevel, SpawnPoint, InteractiveObject, PlaceholderAsset
   - Resources: DemoLevelState, DemoAssetHandles
   - Components: DemoMarker, InteractableDemo
   - Validation rules and state transitions documented

2. **System Contracts** → [contracts/demo_level_interface.md](contracts/demo_level_interface.md)
   - load_demo_level: Auto-loads on GameState::Loading
   - handle_asset_fallback: Placeholder graphics on failure
   - cleanup_demo_level: Despawn demo entities
   - handle_demo_interaction: Player-object interactions
   - Performance contracts: 30 FPS, <10s load, <50ms input lag

3. **Test Scenarios** → [quickstart.md](quickstart.md)
   - Manual validation steps for all 8 functional requirements
   - Automated test specifications
   - Success criteria checklist
   - Troubleshooting guide

4. **Agent Context Updated** → CLAUDE.md
   - Demo level feature tech stack added
   - RON level data format documented
   - Recent changes tracked

**Phase 1 Re-check**: Constitution compliance verified ✅
- No new violations introduced
- Existing performance deviations documented in Complexity Tracking
- All systems follow Bevy ECS patterns

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
The `/tasks` command will generate implementation tasks following TDD principles and leveraging existing game engine systems:

1. **Asset Creation Tasks** (can run in parallel):
   - Create demo_level.ron with tilemap data
   - Create placeholder.png sprite (32x32 magenta)
   - Create demo tileset texture
   - Create interactive object sprites (door, item, etc.)

2. **Data Structure Tasks** (foundational):
   - Define DemoLevel struct with serde derives
   - Define DemoLevelState resource
   - Define DemoAssetHandles resource
   - Define DemoMarker and InteractableDemo components

3. **Contract Test Tasks** (TDD - tests first):
   - Test: demo_level_loading (loads within 10s, spawns correctly) [P]
   - Test: demo_asset_fallback (uses placeholder on missing assets) [P]
   - Test: demo_performance (30 FPS, <50ms input lag) [P]
   - Test: demo_interaction (player can interact with objects) [P]

4. **System Implementation Tasks** (to make tests pass):
   - Implement load_demo_level system
   - Implement handle_asset_fallback system
   - Implement cleanup_demo_level system
   - Implement handle_demo_interaction system
   - Create DemoPlugin and register systems

5. **Integration Tasks**:
   - Add DemoPlugin to main.rs
   - Configure GameState to trigger demo on startup
   - Verify all FR requirements in quickstart.md

**Ordering Strategy**:
- Assets → Data structures → Tests → Implementation
- Mark [P] for parallel execution where no dependencies exist
- TDD principle: Tests defined before implementation
- Leverage existing systems (level_loader, player_movement, tilemap, collision)

**Estimated Output**: 18-22 numbered, ordered tasks in tasks.md

**Key Dependencies**:
- Tasks 1-4 (assets) can run in parallel
- Tasks 5-8 (data structures) depend on assets existing
- Tasks 9-12 (tests) depend on data structures
- Tasks 13-16 (implementation) depend on tests
- Tasks 17+ (integration) depend on implementation

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Constitutional principle deviations documented and justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Input lag 50ms (vs 16ms) | Demo is testing/validation tool, not production gameplay | 16ms requirement too strict for demo purposes - 50ms acceptable for validating functionality |
| 30 FPS target (vs 60 FPS) | Demo showcases capabilities, not final performance | 30 FPS sufficient to validate movement, interaction, rendering work correctly - optimization happens in production |
| 10s load time (vs 3s) | Demo level may include tutorial content, more assets | 3s limit for production levels - demo can be larger/heavier for comprehensive testing |

**Justification**: These relaxed constraints apply ONLY to the demo level feature. Production game levels will maintain constitutional standards (16ms input lag, 60 FPS, 3s load times). The demo is explicitly a developer/tester validation tool, not player-facing content.


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [✅] Phase 0: Research complete (/plan command)
- [✅] Phase 1: Design complete (/plan command)
- [✅] Phase 2: Task planning complete (/plan command - describe approach only)
- [✅] Phase 3: Tasks generated (/tasks command) - 27 tasks in tasks.md
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [✅] Initial Constitution Check: PASS (with documented deviations)
- [✅] Post-Design Constitution Check: PASS
- [✅] All NEEDS CLARIFICATION resolved (via /clarify)
- [✅] Complexity deviations documented (performance constraints relaxed for demo)

**Artifacts Generated**:
- [✅] research.md - Technical decisions and alternatives
- [✅] data-model.md - Entities, resources, components, validation
- [✅] contracts/demo_level_interface.md - System contracts and performance contracts
- [✅] quickstart.md - Manual validation steps and success criteria
- [✅] CLAUDE.md - Updated agent context

**Ready for**: `/tasks` command to generate tasks.md

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
