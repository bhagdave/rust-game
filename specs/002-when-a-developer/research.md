# Phase 0: Research & Technical Decisions

## Codebase Analysis

### Existing Systems Inventory
**Findings from source code review**:
- ✅ `level_loader.rs` - RON parsing with `LevelData` struct already implemented
- ✅ `GameState` resource exists but is NOT a Bevy `State<T>` - it's a plain Resource with `GameMode` enum
- ✅ `AssetHandles` resource uses `HashMap<SpriteType, Handle<Image>>` pattern
- ✅ Existing level format: `ground_floor_entry.ron` with tiles, entities, connections structure
- ✅ Assets directory structure: `assets/sprites/`, `assets/levels/`, `assets/audio/`, `assets/fonts/`
- ❌ No existing placeholder/fallback asset system - needs implementation
- ❌ No Bevy state hooks (`OnEnter`, `OnExit`) currently used - systems use `Update`/`FixedUpdate` schedules
- ❌ No demo or first-run detection mechanism

**Impact on implementation**:
- Cannot use `OnEnter(GameState)` state hook (GameState is not a Bevy state)
- Must work with `GameMode` enum transitions instead
- Can reuse existing `LevelData` struct and RON parsing
- Can follow existing `AssetHandles` pattern for demo assets
- Must create new placeholder fallback system (no precedent in codebase)

## Research Areas

### 1. Bevy App Startup and Auto-Loading Systems
**Decision**: Use startup system with `GameMode` check to trigger demo level loading
**Rationale**:
- **CORRECTION**: Codebase uses `GameState` Resource with `GameMode` enum, NOT Bevy's `State<T>` system
- Default `GameMode` is `Menu` - need to detect first run and transition to demo
- Use `Startup` schedule for one-time initialization, then `Update` for mode transitions
- Check for absence of save file to determine first run (existing pattern from save_load system)
- Transition `GameMode::Menu` → `GameMode::Playing` when loading demo

**Updated approach**:
```rust
// In Startup schedule
fn init_demo_system(mut game_state: ResMut<GameState>) {
    if !save_file_exists() {  // First run detection
        game_state.game_mode = GameMode::Playing;
        // Trigger demo load
    }
}

// In Update schedule
fn load_demo_on_first_run(
    game_state: Res<GameState>,
    mut commands: Commands,
    mut demo_loaded: Local<bool>,
) {
    if game_state.game_mode == GameMode::Playing && !*demo_loaded {
        // Load demo level using existing LevelData pattern
        *demo_loaded = true;
    }
}
```

**Alternatives considered**:
- Bevy State<T> system - Would require major refactor of existing GameState resource
- Command-line arguments - Violates requirement of "no manual configuration"
- Environment variables - Adds external dependency
- Always load demo - Would interfere with save/load system on subsequent runs

### 2. Asset Fallback and Placeholder Graphics
**Decision**: Extend `AssetHandles` resource with `DemoPlaceholder` sprite type and load fallback at startup
**Rationale**:
- **CORRECTION**: Existing codebase uses `AssetHandles` resource with `HashMap<SpriteType, Handle<Image>>`
- Add `SpriteType::DemoPlaceholder` enum variant to existing system
- Load placeholder sprite (32x32 magenta PNG) in demo startup system
- On asset load failure, check `AssetServer::load_state()` and substitute placeholder handle
- Follows existing asset management pattern - no new resource types needed

**Updated approach**:
```rust
// Extend existing SpriteType enum (in resources/asset_handles.rs)
pub enum SpriteType {
    Player,
    Candle,
    // ... existing types
    DemoPlaceholder,  // NEW: fallback for missing demo assets
}

// In demo load system
fn load_demo_assets(
    asset_server: Res<AssetServer>,
    mut asset_handles: ResMut<AssetHandles>,
) {
    // Load placeholder first (guaranteed to exist)
    let placeholder = asset_server.load("sprites/demo_placeholder.png");
    asset_handles.sprites.insert(SpriteType::DemoPlaceholder, placeholder.clone());

    // Try to load demo sprites, fallback to placeholder on failure
    let door_sprite = match asset_server.load_state("sprites/door.png") {
        LoadState::Loaded(handle) => handle,
        _ => placeholder.clone(),
    };
}
```

**Codebase findings**:
- ❌ No existing fallback/placeholder pattern in codebase - this is new functionality
- ✅ Can extend `AssetHandles::sprites` HashMap without breaking changes
- ✅ `AssetServer::load()` returns `Handle<Image>` immediately (async loading)
- ✅ Can check load status with `AssetServer::load_state()` or `Assets<Image>::get()`

**Alternatives considered**:
- Separate `DemoAssetHandles` resource - Redundant, adds complexity
- Bevy's `AssetServer::watch_for_changes()` - Overkill for demo purposes
- Embedded placeholder image data - Harder to replace, not following asset pattern
- Skip rendering on failure - Less visible for testing (violates FR-008)

### 3. Demo Level Data Format
**Decision**: Reuse existing `LevelData` struct from `level_loader.rs` - no new format needed
**Rationale**:
- **CORRECTION**: Existing `level_loader.rs` already defines complete `LevelData` struct with:
  - `id: usize`, `floor: Floor`, `name: String`
  - `bounds: Bounds` (min/max coordinates)
  - `tiles: Vec<Vec<u32>>` (2D tile grid)
  - `entities: Vec<EntitySpawn>` (with entity_type, position, optional target_room, locked, key_type)
  - `connections: Vec<RoomConnection>` (target_room, connection_type, position, locked)
- Existing `load_level_data(path)` function reads RON files from `assets/levels/`
- Reference implementation exists: `assets/levels/ground_floor_entry.ron`
- No new data structures needed - demo level is just another room file

**Actual format** (from `ground_floor_entry.ron`):
```ron
(
    id: 0,
    floor: Ground,
    name: "Entry Hall",
    bounds: (min: (0.0, 0.0), max: (1920.0, 1080.0)),
    tiles: [
        [1, 1, 1, ...],  // 20x15 grid, 1=wall, 0=floor
        [1, 0, 0, ...],
        // ...
    ],
    entities: [
        (entity_type: "PlayerSpawn", position: (960.0, 540.0)),
        (entity_type: "Match", position: (300.0, 200.0)),
        (entity_type: "Key", position: (200.0, 900.0), key_type: Some(Brass)),
        (entity_type: "Door", position: (1840.0, 540.0), target_room: Some(1), locked: Some(Brass)),
    ],
    connections: [
        (target_room: 1, connection_type: Door, position: (1840.0, 540.0), locked: Some(Brass)),
    ],
)
```

**Implementation implications**:
- ✅ Create `assets/levels/demo.ron` following existing format
- ✅ Call existing `load_level_data("levels/demo.ron")` function
- ✅ Reuse existing entity spawn patterns (Match, Key, Door entity types)
- ❌ No need for new `DemoLevel` struct - use `LevelData`
- ⚠️ May need to extend `EntitySpawn.entity_type` with demo-specific types

**Alternatives considered**:
- New DemoLevel struct - Redundant, already have LevelData
- Simplified format - Inconsistent with existing levels, harder to transition to full game
- Hardcoded demo - Less flexible, violates existing data-driven pattern

### 4. Performance Measurement and Validation
**Decision**: Use Bevy's diagnostic plugin for FPS tracking, custom startup timer for load time
**Rationale**:
- `bevy::diagnostic::FrameTimeDiagnosticsPlugin` provides built-in FPS measurement
- Simple `Instant::now()` timer can measure demo level load duration
- Integration test can assert performance requirements (30 FPS, <10s load)
- Existing `benches/lighting_bench.rs` shows Criterion is available for deeper profiling

**Test approach**:
```rust
#[test]
fn demo_meets_performance_requirements() {
    // Measure load time with Instant::now()
    // Run for N frames and check FPS >= 30
    // Measure input lag with timestamp deltas
}
```

**Alternatives considered**:
- Manual testing only - Not automatable, violates testing discipline
- External profiling tools - Overkill for simple requirements
- Frame-by-frame assertions - Too granular, may cause flaky tests

### 5. Integration with Existing Game Systems
**Decision**: Create `DemoPlugin` that registers demo-specific systems and loads on first run
**Rationale**:
- Bevy's plugin system provides modular organization
- Can check `GameState` to determine if it's first run (no save file exists)
- Reuses existing systems: `level_loader`, `player_movement`, `tilemap`, `collision`
- Demo plugin can be conditionally enabled/disabled

**Plugin structure**:
```rust
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_demo_level)
           .add_systems(Update, handle_demo_interactions);
    }
}
```

**Alternatives considered**:
- Modify main.rs directly - Less modular, harder to disable
- Separate demo binary - Unnecessary complexity, violates "single executable" requirement
- Feature flag - Overengineering for demo purposes

### 6. Integration with Existing Entity Spawning
**Research question**: How are entities actually spawned from LevelData in the current codebase?
**Findings**:
- ❌ `load_level_system` in `level_loader.rs` is a **placeholder** - it only logs entities, doesn't spawn them
- ❌ No existing entity spawning implementation found in codebase
- ❌ TODO comments in code: `// TODO: Spawn entities based on level_data.entities`
- ⚠️ This is a **GAP** - demo level implementation must create entity spawning logic

**Decision**: Implement basic entity spawning system for demo level
**Approach**:
```rust
fn spawn_demo_entities(
    level_data: &LevelData,
    commands: &mut Commands,
    asset_handles: &AssetHandles,
) {
    for entity_spawn in &level_data.entities {
        match entity_spawn.entity_type.as_str() {
            "PlayerSpawn" => spawn_player(commands, entity_spawn.position, asset_handles),
            "Door" => spawn_door(commands, entity_spawn, asset_handles),
            "Match" | "Key" => spawn_item(commands, entity_spawn, asset_handles),
            _ => warn!("Unknown entity type: {}", entity_spawn.entity_type),
        }
    }
}
```

**Implementation required**:
- ⚠️ Need to implement entity spawning functions (not in current codebase)
- ⚠️ This is MORE work than initially assumed in plan.md
- ✅ Can start simple - just spawn sprites at positions, add components
- ✅ Leverage existing component types (Player, Velocity, etc.)

### 7. Tilemap Integration Research
**Research question**: How does bevy_ecs_tilemap integrate with the level data?
**Findings**:
- ✅ `tilemap.rs` system exists in codebase
- ⚠️ Need to investigate how `tiles: Vec<Vec<u32>>` maps to tilemap entities
- ⚠️ Need to understand existing tilemap setup (likely needs research during implementation)
- ✅ `tileset.png` exists in `assets/sprites/` - can use for demo

**Decision**: Research deferred to task implementation phase
**Rationale**: Tilemap setup is complex enough to warrant dedicated investigation during implementation. Existing `tilemap.rs` system should provide examples to follow.

## Unknowns Resolved

Technical unknowns from specification (updated with codebase findings):
- ✅ How to auto-load on first run: **CORRECTED** - Use `Startup` system + `GameMode` check, not OnEnter state hook
- ✅ Asset fallback mechanism: **CORRECTED** - Extend `AssetHandles` with `DemoPlaceholder` sprite type
- ✅ Performance measurement: Diagnostic plugin + custom timers (unchanged)
- ✅ Level data format: **CORRECTED** - Reuse existing `LevelData` struct, not new format
- ✅ Integration approach: DemoPlugin with existing systems (unchanged)

**New unknowns discovered**:
- ⚠️ Entity spawning implementation needed (not in current codebase)
- ⚠️ Tilemap setup details need investigation during implementation
- ⚠️ First-run detection mechanism (check for save file)

## Dependencies Confirmed

All required dependencies already present in Cargo.toml:
- ✅ bevy 0.16.1 - Core engine
- ✅ bevy_ecs_tilemap 0.16.0 - Tilemap rendering
- ✅ serde + ron - Level data deserialization
- ✅ leafwing-input-manager 0.17.0 - Input handling

No new dependencies required.

## Impact on Plan.md Assumptions

**Corrections needed in plan.md**:
1. ❌ Cannot use Bevy `State<T>` system - GameState is a Resource
2. ❌ `DemoLevel` struct is redundant - use existing `LevelData`
3. ❌ `DemoAssetHandles` resource is redundant - extend `AssetHandles`
4. ⚠️ Entity spawning is more complex - not implemented in existing codebase
5. ✅ Plugin architecture is correct - matches existing `FixedTimestepPlugin` pattern

**Complexity increase**:
- Entity spawning system needs implementation (not just reuse)
- First-run detection mechanism needs implementation
- Asset fallback system is new (no existing pattern)
