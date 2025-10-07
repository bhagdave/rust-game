# Tasks: Demo Level on First Run

**Input**: Design documents from `/home/dave/Projects/rust-game/specs/002-when-a-developer/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Extract: Rust 1.75+, Bevy 0.16.1, bevy_ecs_tilemap, RON serialization
   → Structure: Single project (src/, tests/ at repo root)
2. Load design documents:
   → research.md: Use existing LevelData, extend AssetHandles, implement entity spawning
   → data-model.md: Reuse LevelData (not DemoLevel), extend SpriteType enum
   → contracts/: Demo level system contracts, performance contracts
   → quickstart.md: 8 manual validation scenarios + automated tests
3. Generate tasks by category:
   → Setup: Assets, data structures
   → Tests: Contract tests, integration tests, performance tests (TDD)
   → Core: Demo level loading, entity spawning, asset fallback
   → Integration: DemoPlugin, main.rs integration
   → Polish: Documentation, clippy, formatting, quickstart validation
4. Apply task rules:
   → Different files = mark [P] for parallel execution
   → Same file = sequential (no [P])
   → Tests before implementation (TDD principle)
5. Number tasks sequentially (T001-T025)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- All file paths are absolute from repository root
- **CRITICAL**: Research findings show entity spawning NOT implemented in existing codebase - must build from scratch

## Phase 3.1: Asset Creation & Setup
**Note**: Asset tasks can run in parallel - different files, no code dependencies

- [X] **T001** [P] Create placeholder sprite at `assets/sprites/demo_placeholder.png` (32x32 magenta #FF00FF PNG for missing asset visibility)
- [X] **T002** [P] Create demo level RON file at `assets/levels/demo.ron` following existing `LevelData` format from `ground_floor_entry.ron` (include tiles array, player spawn, interactive objects: 2-3 doors, 2-3 items)
- [X] **T003** [P] Verify demo tileset exists at `assets/sprites/tileset.png` or create simple 2-tile version (floor + wall, 32x32 each)

## Phase 3.2: Data Structure Extensions
**Note**: These extend existing files - must be done sequentially per file

- [X] **T004** Extend `SpriteType` enum in `src/resources/asset_handles.rs` to add `DemoPlaceholder` variant for fallback graphics
- [X] **T005** Create `DemoMarker` component in new file `src/components/demo.rs` as marker for demo-spawned entities (simple unit struct with `#[derive(Component)]`)
- [X] **T006** Add `pub mod demo;` to `src/components/mod.rs` to expose DemoMarker component

## Phase 3.3: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.4
**CRITICAL**: These tests MUST be written and MUST FAIL before ANY implementation
**Constitutional Requirement**: Testing Discipline (Principle II) - 80% coverage, deterministic, <30s execution

- [X] **T007** [P] Create contract test `tests/demo_level_loading.rs`:
  - Test: demo level loads from `assets/levels/demo.ron` within 10 seconds
  - Test: demo level spawns player at correct position from level data
  - Test: all entities from level data are spawned with correct components
  - Test: DemoMarker component attached to all demo entities
  - **Expected**: All tests FAIL (no implementation yet)

- [X] **T008** [P] Create contract test `tests/demo_asset_fallback.rs`:
  - Test: when sprite asset fails to load, placeholder handle is used
  - Test: placeholder sprite (magenta) is visibly rendered
  - Test: game continues running without crash when assets missing
  - Test: warning logged to console about missing asset
  - **Expected**: All tests FAIL (no fallback system yet)

- [X] **T009** [P] Create performance test `tests/demo_performance.rs`:
  - Test: demo maintains minimum 30 FPS over 100 frames
  - Test: demo loads within 10 seconds (measure with `Instant::now()`)
  - Test: input lag <50ms (measure timestamp delta from input to player movement)
  - Use Bevy's `FrameTimeDiagnosticsPlugin` for FPS measurement
  - **Expected**: Tests FAIL or cannot run (no demo implementation)

- [X] **T010** [P] Create integration test `tests/demo_interaction.rs`:
  - Test: player can move with keyboard input (WASD/arrows)
  - Test: interaction prompt appears when near interactive object
  - Test: interaction executes on key press (E key)
  - Test: verify interaction completes within 50ms
  - **Expected**: Tests FAIL (no interaction system for demo)

- [X] **T011** Verify all tests are deterministic and complete under 30 seconds total (run `cargo test --lib` and check timing)

## Phase 3.4: Core Implementation (ONLY after tests are failing)
**Critical Finding from research.md**: Entity spawning NOT implemented in existing codebase - must build from scratch

### 3.4.1: Entity Spawning Infrastructure

- [X] **T012** Create base file `src/systems/demo_level.rs` with module structure:
  - Add file header and module-level rustdoc
  - Import required Bevy types and local components
  - Create placeholder for future functions
  - No implementation yet - just structure

- [X] **T013** Extend `src/components/demo.rs` to add `InteractableDemo` component:
  - Fields: `object_id: String`, `interaction_prompt: String`
  - Derive: `Component`, `Debug`, `Clone`
  - Add rustdoc explaining usage for demo interactive objects
  - Note: Already completed in T005-T006

### 3.4.2: Player Spawning

- [X] **T014** Implement `spawn_player()` helper function in `src/systems/demo_level.rs`:
  - Parameters: `commands: &mut Commands`, `position: Vec2`, `asset_handles: &AssetHandles`
  - Spawn entity with `Player`, `Velocity`, `JumpState`, `Health` components
  - Add `DemoMarker` component
  - Add `Sprite` component with player sprite from AssetHandles
  - Add rustdoc with parameter descriptions and usage example
  - Return spawned entity ID

### 3.4.3: Interactive Object Spawning

- [ ] **T015** Implement `spawn_door()` helper function in `src/systems/demo_level.rs`:
  - Parameters: `commands: &mut Commands`, `entity_spawn: &EntitySpawn`, `asset_handles: &AssetHandles`
  - Spawn entity at position from entity_spawn
  - Add `InteractableDemo` component with object_id and prompt "Press E to open"
  - Add `DemoMarker` component
  - Add `SpriteBundle` with door sprite from AssetHandles
  - Handle locked doors if entity_spawn has locked field
  - Add rustdoc and return spawned entity ID

- [ ] **T016** Implement `spawn_item()` helper function in `src/systems/demo_level.rs`:
  - Parameters: `commands: &mut Commands`, `entity_spawn: &EntitySpawn`, `asset_handles: &AssetHandles`
  - Spawn entity at position from entity_spawn
  - Add `InteractableDemo` component with object_id and prompt "Press E to collect"
  - Add `DemoMarker` component
  - Add `SpriteBundle` with appropriate sprite (Match or Key) from AssetHandles
  - Differentiate between Match and Key entity types
  - Add rustdoc and return spawned entity ID

### 3.4.4: Entity Spawning Orchestration

- [ ] **T017** Implement `spawn_demo_entities()` orchestrator function in `src/systems/demo_level.rs`:
  - Parameters: `level_data: &LevelData`, `commands: &mut Commands`, `asset_handles: &AssetHandles`
  - Iterate through `level_data.entities`
  - Match on `entity_spawn.entity_type`: "PlayerSpawn", "Door", "Match", "Key"
  - Call appropriate spawn helper (spawn_player, spawn_door, spawn_item)
  - Log warning for unknown entity types
  - Track and return count of spawned entities
  - Add rustdoc with example usage

### 3.4.5: Demo Level Loading System

- [ ] **T018** Implement `load_demo_level` system in `src/systems/demo_level.rs`:
  - System signature with Commands, AssetHandles, AssetServer resources
  - Load demo level using existing `load_level_data("levels/demo.ron")` function
  - Record load start time with `Instant::now()` for performance measurement
  - Store load time in a Local<Option<Instant>> resource
  - Handle load errors gracefully with warnings
  - Add rustdoc explaining system purpose and timing

- [ ] **T019** Extend `load_demo_level` system to spawn tilemap:
  - Extract `level_data.tiles` 2D array
  - Create basic tilemap using bevy_ecs_tilemap (research existing tilemap.rs for pattern)
  - Use existing tileset from AssetHandles
  - Set tilemap position and layer order
  - Log tilemap spawn success with dimensions

- [ ] **T020** Extend `load_demo_level` system to spawn entities:
  - Call `spawn_demo_entities()` with level_data, commands, and asset_handles
  - Extract player spawn position from spawned entities
  - Log successful demo load with level name and entity count
  - Calculate and log total load duration
  - Set demo loaded flag to prevent re-loading

### 3.4.6: Asset Fallback System

- [ ] **T021** Implement asset fallback system in `src/systems/demo_level.rs`:
  - Create `load_demo_assets_with_fallback()` function
  - Load placeholder sprite first: `asset_server.load("sprites/demo_placeholder.png")`
  - Insert into `AssetHandles` with `SpriteType::DemoPlaceholder` key
  - For each demo sprite, check load state with `asset_server.get_load_state()`
  - If load fails or not found, use placeholder handle and log warning
  - Return HashMap of entity types to sprite handles

### 3.4.7: First-Run Detection

- [ ] **T022** Implement first-run detection in `src/systems/demo_level.rs`:
  - Create `should_load_demo()` function
  - Check for save file existence using `directories` crate (existing dependency)
  - Return true if no save file exists (first run)
  - Add rustdoc explaining first-run logic

### 3.4.8: Demo Plugin Architecture

- [ ] **T023** Create `DemoPlugin` struct in `src/systems/demo_level.rs`:
  - Implement `Plugin` trait for `DemoPlugin`
  - In `build()`: Register demo level systems in `Startup` and `Update` schedules
  - System ordering: `init_demo` (Startup) → `load_demo_on_first_run` (Update)
  - Use `Local<bool>` state to ensure demo loads only once
  - Follow existing plugin pattern from `FixedTimestepPlugin`

### 3.4.9: Interaction Systems

- [ ] **T024** Implement `handle_demo_interaction` system in `src/systems/demo_level.rs`:
  - Query player position and `InteractableDemo` entities
  - Check distance < 50 pixels for interaction range
  - Display interaction prompt when in range (use existing UI system)
  - Execute interaction on key press (use existing `InputConfig`)
  - Provide visual feedback (log or simple effect)

### 3.4.10: Cleanup Systems

- [ ] **T025** Implement `cleanup_demo_level` system in `src/systems/demo_level.rs`:
  - Query all entities with `DemoMarker` component
  - Despawn all demo entities when transitioning away from demo
  - Reset any demo-specific state
  - Add to `OnExit` or similar transition hook (research GameMode transitions)

- [ ] **T026** Add `pub mod demo_level;` to `src/systems/mod.rs` to expose demo systems

## Phase 3.5: Integration
**Note**: These tasks modify main.rs and must be sequential

- [ ] **T027** Integrate `DemoPlugin` into `src/main.rs`:
  - Import `DemoPlugin` from `rust_game::systems::demo_level`
  - Add `.add_plugins(DemoPlugin)` to app builder after DefaultPlugins
  - Ensure `GameState` resource is initialized before DemoPlugin systems run
  - Add `AssetHandles` resource if not already present
  - Test that game compiles: `cargo build`

- [ ] **T028** Configure demo to auto-load on first run in `src/systems/demo_level.rs`:
  - In `Startup` schedule: Check `should_load_demo()` and set `GameMode::Playing`
  - In `Update` schedule with `Local<bool>`: Load demo if mode is Playing and not yet loaded
  - Ensure compatibility with existing `GameState` resource (not Bevy State<T>)
  - Use existing `GameMode` enum transitions

## Phase 3.6: Polish & Validation
**Note**: These can run in parallel - independent validation tasks

- [ ] **T029** [P] Run `cargo fmt --check` and `cargo clippy -- -D warnings` on all modified files:
  - Fix any formatting issues with `cargo fmt`
  - Fix any clippy warnings in demo_level.rs, demo.rs, asset_handles.rs
  - Ensure zero warnings before committing

- [ ] **T030** [P] Add rustdoc comments to all public APIs in `src/systems/demo_level.rs`:
  - Document `DemoPlugin` struct and implementation
  - Document all public functions with examples
  - Document performance expectations and constitutional deviations
  - Run `RUSTDOCFLAGS="-D warnings -D missing_docs" cargo doc --no-deps` to verify

- [ ] **T031** [P] Run full test suite and verify 80% coverage:
  - Execute `cargo test` and ensure all tests pass
  - Check that tests complete under 30 seconds total
  - Verify demo_level_loading, demo_asset_fallback, demo_performance, demo_interaction all pass
  - If available, run coverage tool (cargo tarpaulin) to verify 80% coverage

- [ ] **T032** [P] Execute quickstart.md manual validation scenarios:
  - Run `cargo run --release` and verify demo auto-loads
  - Test Steps 1-8 from quickstart.md (first run, visual assets, movement, interaction, tilemap, fallback, performance, repeat run)
  - Document any failures or deviations from expected behavior
  - Verify all functional requirements (FR-001 through FR-008) are met

- [ ] **T033** [P] Performance benchmarking (optional but recommended):
  - Run demo for 5 minutes and monitor FPS (should stay >= 30)
  - Measure actual load time with stopwatch (should be < 10s)
  - Test input responsiveness subjectively (<50ms perceptible lag)
  - Document actual performance metrics vs requirements

## Dependencies

**Phase Dependencies**:
- Phase 3.1 (T001-T003) → Phase 3.2 (T004-T006) → Phase 3.3 (T007-T011)
- Phase 3.3 (T007-T011) MUST COMPLETE before Phase 3.4 (T012-T026)
- Phase 3.4 (T012-T026) → Phase 3.5 (T027-T028) → Phase 3.6 (T029-T033)

**Specific Task Dependencies**:
- T004 must complete before T021 (SpriteType extension before asset loading)
- T005-T006 must complete before T012 (DemoMarker needed for entity spawning)
- T012-T013 must complete before T014-T016 (InteractableDemo component and file structure before spawning functions)
- T014-T016 must complete before T017 (individual spawn functions before orchestrator)
- T017 must complete before T018-T020 (orchestrator before level loading)
- T018-T020 must complete before T023 (level loading before plugin)
- T014-T026 must complete before T027 (all systems implemented before plugin integration)
- T027-T028 must complete before T029-T033 (integration done before validation)

**Parallel Execution Groups**:
- Group A (Assets): T001, T002, T003 - all different files
- Group B (Tests): T007, T008, T009, T010 - all different test files
- Group C (Validation): T029, T030, T031, T032, T033 - independent checks

## Parallel Execution Examples

### Launch Asset Creation Tasks (T001-T003)
```bash
# All asset files are independent - can create in parallel
# T001: Create placeholder sprite
# T002: Create demo level RON
# T003: Verify/create tileset
```

### Launch All Test Tasks (T007-T010)
```bash
# All test files are independent - can write in parallel
# T007: tests/demo_level_loading.rs
# T008: tests/demo_asset_fallback.rs
# T009: tests/demo_performance.rs
# T010: tests/demo_interaction.rs
```

### Launch Validation Tasks (T029-T033)
```bash
# All validation tasks are independent checks
# T029: Run fmt and clippy
# T030: Add rustdoc comments
# T031: Run test suite
# T032: Execute quickstart validation
# T033: Performance benchmarking
```

## Notes

### Critical Findings from Research
- ⚠️ **GameState is NOT Bevy State<T>** - it's a plain Resource with GameMode enum
  - Cannot use `OnEnter(GameState)` state hooks
  - Must use `Startup` + `Update` schedules with GameMode checks
- ⚠️ **Entity spawning NOT implemented** - load_level_system only logs, doesn't spawn
  - T012 requires implementing entity spawning from scratch
  - More complex than initially assumed
- ✅ **LevelData struct exists** - reuse existing, no need for DemoLevel struct
- ✅ **AssetHandles pattern exists** - extend with DemoPlaceholder variant

### Constitutional Deviations (Documented in plan.md)
- Input lag: 50ms (vs 16ms constitutional requirement)
- Frame rate: 30 FPS minimum (vs 60 FPS constitutional requirement)
- Load time: 10s (vs 3s constitutional requirement)
- **Justification**: Demo is testing/validation tool, not production gameplay

### Task Execution Tips
- Verify tests FAIL in T007-T010 before proceeding to T012
- Use `cargo test --test demo_level_loading` to run individual test files
- Use `cargo build` after each system implementation to catch errors early
- Break down entity spawning into small incremental steps (T012-T020)
- Test each spawn function individually before integrating
- Commit after completing each phase for clean rollback points

## Validation Checklist
*GATE: Checked before marking feature complete*

- [ ] All tests from T007-T010 passing
- [ ] All functional requirements (FR-001 through FR-008) validated via T032
- [ ] Performance requirements met (30 FPS, <10s load, <50ms input) via T009, T033
- [ ] Zero clippy warnings (T029)
- [ ] All public APIs documented (T030)
- [ ] Test coverage >= 80% (T031)
- [ ] Quickstart scenarios all pass (T032)
- [ ] Demo loads automatically on first run with no configuration
- [ ] Placeholder graphics work when assets missing
- [ ] Game continues running after multiple demo runs

---

**Total Tasks**: 33 (T001-T033)
**Estimated Parallel Groups**: 3 groups (9 tasks can run in parallel)
**Estimated Sequential Tasks**: 24 tasks (entity spawning, integration, core systems)
**Critical Path**: T001→T004→T005→T012→T013→T014→T017→T018→T023→T027→T032
