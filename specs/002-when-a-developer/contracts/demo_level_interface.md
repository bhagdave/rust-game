# Demo Level System Contracts

## System Interface: DemoLevelPlugin

### 1. Load Demo Level
**Trigger**: `OnEnter(GameState::Loading)` state transition
**Input**: None (automatic on startup)
**Output**: Demo level entities spawned, DemoLevelState resource updated

**Contract**:
```rust
/// Loads demo level from assets/demo_level.ron and spawns entities
fn load_demo_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut demo_state: ResMut<DemoLevelState>,
) {
    // MUST: Record load start time for performance measurement
    // MUST: Attempt to load demo_level.ron asset
    // MUST: Parse RON into DemoLevel struct
    // MUST: Spawn tilemap entities from tilemap_data
    // MUST: Spawn player at player_spawn position
    // MUST: Spawn interactive objects at specified positions
    // MUST: Attach DemoMarker component to all spawned entities
    // MUST: Update demo_state.is_loaded = true on success
}
```

**Preconditions**:
- GameState is Loading
- Asset server is initialized
- DemoLevelState resource exists

**Postconditions**:
- Demo level entities exist in world
- Player can move and interact
- DemoLevelState.is_loaded == true
- Load duration < 10 seconds (verified in tests)

**Error Handling**:
- If demo_level.ron missing: Use hardcoded minimal level
- If asset load fails: Use placeholder graphics (magenta sprite)
- Never panic - always provide fallback

---

### 2. Handle Asset Fallback
**Trigger**: Asset load failure event
**Input**: Failed asset path
**Output**: Placeholder sprite handle assigned

**Contract**:
```rust
/// Replaces failed asset with placeholder sprite
fn handle_asset_fallback(
    failed_path: &str,
    demo_assets: Res<DemoAssetHandles>,
) -> Handle<Image> {
    // MUST: Log warning with failed asset path
    // MUST: Return demo_assets.placeholder_handle
    // MUST NOT: Panic or halt execution
}
```

**Preconditions**:
- DemoAssetHandles resource initialized
- Placeholder asset is valid and loaded

**Postconditions**:
- Returned handle is valid
- Visual feedback shows missing asset (magenta color)
- Game continues running

---

### 3. Cleanup Demo Level
**Trigger**: State transition away from demo (e.g., to MainMenu)
**Input**: None
**Output**: All demo entities despawned

**Contract**:
```rust
/// Removes all demo level entities from world
fn cleanup_demo_level(
    mut commands: Commands,
    demo_entities: Query<Entity, With<DemoMarker>>,
    mut demo_state: ResMut<DemoLevelState>,
) {
    // MUST: Despawn all entities with DemoMarker component
    // MUST: Reset demo_state.is_loaded = false
    // MUST: Clear asset_failures list
}
```

**Preconditions**:
- Demo level was previously loaded

**Postconditions**:
- No DemoMarker entities remain
- DemoLevelState.is_loaded == false
- Memory freed for demo assets

---

## Component Interface: InteractableDemo

### Interaction Contract
**Trigger**: Player input (keyboard/gamepad) near InteractableDemo entity
**Input**: Player position, interaction key press
**Output**: Interaction feedback (visual/audio)

**Contract**:
```rust
/// Handles player interaction with demo objects
fn handle_demo_interaction(
    player_query: Query<&Transform, With<Player>>,
    interactable_query: Query<(&Transform, &InteractableDemo)>,
    input: Res<InputConfig>,
) {
    // MUST: Check distance < interaction_range (e.g., 50 pixels)
    // MUST: Display interaction_prompt when in range
    // MUST: Trigger interaction when key pressed
    // MUST: Provide visual feedback (highlight, animation)
    // MUST: Respond within <50ms input lag requirement
}
```

**Preconditions**:
- Player entity exists with Transform
- InteractableDemo entity exists
- Input system configured

**Postconditions**:
- UI shows interaction prompt when in range
- Interaction executes on key press
- Visual feedback is visible

---

## Data Contract: demo_level.ron

### RON File Schema
**Location**: `assets/demo_level.ron`
**Format**: Rusty Object Notation (RON)

**Schema**:
```ron
DemoLevel(
    tilemap_data: [
        // Array of tile IDs, length = bounds.width * bounds.height
        // Each u32 represents a tile type
    ],
    player_spawn: (
        x: f32,  // Must be >= 0 and < bounds.width * tile_size
        y: f32,  // Must be >= 0 and < bounds.height * tile_size
    ),
    interactive_objects: [
        (
            object_type: Door,  // ObjectType enum
            position: (x: f32, y: f32),
            sprite_path: Some("sprites/door.png"),  // Optional
            interaction_type: Open,  // InteractionType enum
        ),
        // ... more objects
    ],
    bounds: (
        width: u32,   // Number of tiles wide
        height: u32,  // Number of tiles tall
    ),
)
```

**Validation Rules**:
1. `tilemap_data.len() == bounds.width * bounds.height`
2. `player_spawn.x < bounds.width * TILE_SIZE`
3. `player_spawn.y < bounds.height * TILE_SIZE`
4. All `interactive_objects[].position` within bounds
5. All `sprite_path` if Some() must be valid relative paths

**Error Cases**:
- File not found: Use minimal hardcoded level (single room, player spawn only)
- Parse error: Log error, use minimal hardcoded level
- Validation failure: Clamp out-of-bounds values, log warning

---

## Performance Contracts

### Load Time Contract
**Requirement**: Demo level loads within 10 seconds
**Measurement**: `Instant::now()` at load start → spawn complete

**Contract**:
```rust
#[test]
fn demo_loads_within_10_seconds() {
    let start = Instant::now();
    // ... trigger load_demo_level system
    let duration = start.elapsed();
    assert!(duration.as_secs() < 10, "Load took {:?}", duration);
}
```

### Frame Rate Contract
**Requirement**: Maintain minimum 30 FPS during demo
**Measurement**: Bevy FrameTimeDiagnosticsPlugin

**Contract**:
```rust
#[test]
fn demo_maintains_30fps() {
    // Run demo for 100 frames
    for _ in 0..100 {
        app.update();
    }
    let fps = get_average_fps(&app);
    assert!(fps >= 30.0, "FPS was {}", fps);
}
```

### Input Lag Contract
**Requirement**: Player input response within 50ms
**Measurement**: Timestamp delta between input event and player movement

**Contract**:
```rust
#[test]
fn input_lag_under_50ms() {
    let input_time = Instant::now();
    // ... send input event
    // ... wait for player movement
    let response_time = Instant::now();
    let lag = response_time.duration_since(input_time);
    assert!(lag.as_millis() < 50, "Input lag was {:?}", lag);
}
```
