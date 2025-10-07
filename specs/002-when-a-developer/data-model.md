# Data Model: Demo Level

## Core Entities

### DemoLevel
Represents the demo level configuration and state.

**Fields**:
- `tilemap_data: Vec<TileData>` - 2D tile array for environment rendering
- `player_spawn: SpawnPoint` - Initial player position (x, y coordinates)
- `interactive_objects: Vec<InteractiveObject>` - Doors, items, pickups
- `bounds: LevelBounds` - Level dimensions (width, height in tiles)

**Validation Rules**:
- `player_spawn` must be within `bounds`
- `tilemap_data.len()` must equal `bounds.width * bounds.height`
- All `interactive_objects` positions must be within `bounds`

**State Transitions**:
```
Unloaded → Loading → Loaded → Active
                ↓
            LoadFailed (show placeholders)
```

**Serialization**: RON format via serde
```ron
DemoLevel(
    tilemap_data: [...],
    player_spawn: (x: 100.0, y: 100.0),
    interactive_objects: [
        InteractiveObject(type: Door, position: (200.0, 150.0), ...),
    ],
    bounds: (width: 50, height: 30),
)
```

### SpawnPoint
Player or object spawn location.

**Fields**:
- `x: f32` - X coordinate in world space
- `y: f32` - Y coordinate in world space

**Validation Rules**:
- Must be non-negative
- Must be within parent level bounds

### InteractiveObject
Objects player can interact with (doors, items, pickups).

**Fields**:
- `object_type: ObjectType` - Enum: Door, Item, Pickup, Trigger
- `position: SpawnPoint` - World position
- `sprite_path: Option<String>` - Asset path (None = use placeholder)
- `interaction_type: InteractionType` - Enum: Collect, Activate, Open

**Validation Rules**:
- `position` must be within level bounds
- `sprite_path` if Some must be valid asset path format
- `object_type` determines valid `interaction_type` combinations

**Relationships**:
- Belongs to one `DemoLevel`
- May reference sprite asset (optional)

### PlaceholderAsset
Fallback visual representation when assets fail to load.

**Fields**:
- `color: Color` - RGB color (default: magenta #FF00FF for visibility)
- `size: Vec2` - Dimensions (default: 32x32 pixels)
- `shape: PlaceholderShape` - Enum: Square, Circle

**Usage**:
- Loaded when primary asset fails
- Clearly visible to indicate missing asset
- Allows continued testing without crashes

## Resource Types (Bevy ECS Resources)

### DemoLevelState
Tracks demo level loading and runtime state.

**Fields**:
- `is_loaded: bool` - Whether demo level is active
- `load_start_time: Option<Instant>` - For measuring load duration
- `asset_failures: Vec<String>` - Paths of failed-to-load assets

**Lifecycle**: Created on app startup, persists until shutdown

### DemoAssetHandles
Stores handles to demo-specific assets.

**Fields**:
- `tileset_handle: Option<Handle<Image>>` - Demo tilemap texture
- `sprite_handles: HashMap<String, Handle<Image>>` - Interactive object sprites
- `placeholder_handle: Handle<Image>` - Fallback placeholder sprite

**Validation Rules**:
- `placeholder_handle` must always be valid (embedded or guaranteed asset)
- Failed asset loads use `placeholder_handle` as fallback

## Component Types (Bevy ECS Components)

### DemoMarker
Marker component for entities spawned by demo level.

**Purpose**: Easy cleanup/identification of demo entities

### InteractableDemo
Component for objects player can interact with in demo.

**Fields**:
- `object_id: String` - Unique identifier
- `interaction_prompt: String` - UI text (e.g., "Press E to open")

## Data Flow

```
App Startup
    ↓
GameState::Loading (OnEnter)
    ↓
Load demo_level.ron → DemoLevel
    ↓
Try load assets → DemoAssetHandles
    ↓ (on failure)
Use PlaceholderAsset
    ↓
Spawn entities (Player, Tilemap, InteractiveObjects)
    ↓
Attach DemoMarker components
    ↓
GameState::Active
    ↓
Demo level playable
```

## Persistence

**Demo Level Data**:
- Stored in `assets/demo_level.ron`
- Read-only (no demo state saved)
- Loaded fresh on each startup

**No Save State**: Demo level does not persist player progress (per FR-007: "accessible on every first run")

## Performance Considerations

**Memory**:
- Single demo level kept in memory (<1MB estimated)
- Assets unloaded when transitioning away from demo
- Placeholder sprites shared across all failures (single allocation)

**Load Time**:
- Tilemap texture: ~0.5s (typical 512x512 PNG)
- Level data parsing: <0.1s (small RON file)
- Entity spawning: <0.2s (dozens of entities)
- **Total estimate**: 1-2s (well under 10s requirement)
