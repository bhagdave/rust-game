# Data Model: House Escape Game

**Date**: 2025-10-03
**Feature**: House Escape Game (001)

## Overview

Entity-Component-System architecture using Bevy ECS. Entities represent game objects (player, items, traps), Components store data, Systems process logic, Resources hold global state.

---

## Core Entities

### 1. Player

**Components**:
- `Player` (marker component)
- `Transform` (Bevy built-in: position, rotation, scale)
- `Velocity` (Vec2: horizontal/vertical speed)
- `JumpState` (enum: Grounded, Jumping, Falling, DoubleJumping)
- `Inventory` (Vec<ItemId>: held items, max capacity)
- `CandleHolder` (Option<Entity>: reference to candle entity)
- `Health` (enum: Alive, Dead)
- `Sprite` (Bevy built-in: texture rendering)
- `Collider` (AABB: for collision detection)

**Relationships**:
- Owns Candle entity (1:1)
- Contains Items (1:many)
- Exists in one Room (many:1)

**State Transitions**:
```
Alive --[trap collision]--> Dead
Dead --[respawn timer]--> Alive
Grounded --[jump input]--> Jumping
Jumping --[double jump unlocked + input]--> DoubleJumping
Jumping/DoubleJumping --[velocity.y < 0]--> Falling
Falling --[ground collision]--> Grounded
```

**Validation**:
- Inventory.len() <= MAX_INVENTORY_SIZE (e.g., 10 items)
- CandleHolder must reference valid Candle entity or None
- Transform.translation must be within current room bounds

---

### 2. Candle

**Components**:
- `Candle` (marker component)
- `CandleWax` (f32: 0.0 to 100.0, depletes over time when lit)
- `CandleState` (enum: Unlit, Lit, Extinguished)
- `VisibilityRadius` (f32: tiles, 1.0-2.0 when unlit, 6.0-8.0 when lit)
- `BurnRate` (f32: wax units per second, e.g., 1.0)
- `Transform` (position follows player)
- `Sprite` (flame animation when lit)
- `LightSource` (rendering data: color, intensity)

**Relationships**:
- Held by Player (1:1)

**State Transitions**:
```
Unlit --[use match]--> Lit
Lit --[wax == 0.0]--> Extinguished
Lit --[manual toggle]--> Unlit
Lit --[environmental hazard]--> Extinguished
Extinguished --[use match]--> Lit
```

**Validation**:
- CandleWax: 0.0 <= wax <= 100.0
- VisibilityRadius: 1.0 <= radius <= 8.0
- BurnRate > 0.0
- CandleState::Lit requires wax > 0.0

---

### 3. Item

**Components**:
- `Item` (enum: Match, Key(KeyType), Tool(ToolType), PuzzleItem(PuzzleItemType), DoubleJumpItem, DiaryPage(usize))
- `StackableItem` (Option<u32>: stack count for matches, None for unique items)
- `Transform` (world position when not in inventory)
- `Sprite` (icon/texture)
- `Collider` (AABB: for pickup detection)
- `Collectible` (marker component: can be picked up)

**Item Types**:
```rust
enum KeyType { Brass, Iron, Ornate, Master }
enum ToolType { Wrench, Crowbar, WireCutters, Magnet, OilCan, Ladder }
enum PuzzleItemType { Fuse, Gemstone(Color), CircuitComponent }
```

**Relationships**:
- Contained in Player Inventory (many:1)
- Placed in Rooms (many:1)

**Validation**:
- StackableItem count > 0 if Some
- Item::Match requires StackableItem::Some(count)
- Item::DiaryPage(id) requires unique id (0-N)

---

### 4. Room

**Components**:
- `Room` (struct: id: RoomId, floor: Floor, name: String)
- `RoomBounds` (struct: min: Vec2, max: Vec2)
- `RoomConnections` (Vec<RoomConnection>: doors, stairs, ladders to other rooms)
- `TileMap` (bevy_ecs_tilemap: visual tiles)
- `CollisionMap` (2D grid: walkable/solid tiles)
- `Explored` (bool: for auto-revealing map)

**Sub-Components**:
```rust
struct RoomConnection {
    target_room: RoomId,
    connection_type: ConnectionType, // Door, Staircase, Ladder, Hidden
    position: Vec2, // location in current room
    locked: Option<KeyType>, // None if unlocked
}

enum Floor { Ground, First, Second, Basement }
```

**Relationships**:
- Contains Entities (player, items, traps, puzzles) (1:many)
- Connected to other Rooms (many:many via RoomConnections)

**Validation**:
- RoomId must be unique across all rooms
- RoomBounds.min < RoomBounds.max
- RoomConnections must reference valid RoomId
- CollisionMap dimensions match TileMap dimensions

---

### 5. Door

**Components**:
- `Door` (marker component)
- `DoorState` (enum: Locked(KeyType), Unlocked, Open)
- `TargetRoom` (RoomId: destination room)
- `Transform` (position in current room)
- `Sprite` (visual representation)
- `Collider` (AABB: for interaction detection)
- `Interactable` (marker: can be activated)

**Relationships**:
- Exists in Room (many:1)
- Connects two Rooms (many:2)

**State Transitions**:
```
Locked --[use correct key]--> Unlocked
Unlocked --[player interaction]--> Open
Open --[player exits]--> Unlocked
```

**Validation**:
- DoorState::Locked requires valid KeyType
- TargetRoom must be valid RoomId
- Transform.translation within current room bounds

---

### 6. Trap

**Components**:
- `Trap` (enum: Spikes, FallingChandelier, CollapsingFloor, Pendulum, ArrowTrap)
- `TrapTrigger` (enum: PressurePlate, Proximity(f32), Timed(f32))
- `TrapState` (enum: Armed, Triggered, Resetting)
- `InstantDeath` (marker: causes player death on collision)
- `Transform` (position)
- `Sprite` (visual + animation)
- `Collider` (AABB: for damage detection)

**Relationships**:
- Exists in Room (many:1)

**State Transitions**:
```
Armed --[trigger condition met]--> Triggered
Triggered --[animation complete]--> Resetting
Resetting --[reset timer]--> Armed
```

**Validation**:
- TrapTrigger::Proximity radius > 0.0
- TrapTrigger::Timed duration > 0.0
- Transform.translation within room bounds

---

### 7. Environmental Hazard

**Components**:
- `EnvironmentalHazard` (enum: DraftyWindow, WaterPuddle, BrokenFloor, FanBlade, SteamVent)
- `HazardEffect` (enum: ExtinguishCandle, SlowMovement, FallDamage)
- `Transform` (position)
- `Sprite` (visual + animation)
- `Collider` (AABB: for effect trigger)

**Relationships**:
- Exists in Room (many:1)

**Validation**:
- Transform.translation within room bounds
- HazardEffect matches EnvironmentalHazard type

---

### 8. Puzzle

**Components**:
- `Puzzle` (enum: CircuitBreaker, PressurePlate, SymbolMatch, MirrorReflection, LeverCombination)
- `PuzzleState` (enum: Unsolved, InProgress, Solved)
- `PuzzleReward` (enum: UnlockDoor(RoomId), RevealPassage(RoomId), SpawnItem(ItemType))
- `Transform` (position)
- `Sprite` (visual)
- `Interactable` (marker)

**Sub-Components** (puzzle-specific):
```rust
// Circuit Breaker
struct CircuitBreakerPuzzle {
    fuse_slots: Vec<Option<ItemId>>, // required fuses
    correct_sequence: Vec<usize>,
}

// Pressure Plate
struct PressurePlatePuzzle {
    plates: Vec<Entity>, // plate entities
    required_items: Vec<ItemId>, // items to place
}

// Symbol Match
struct SymbolMatchPuzzle {
    input_sequence: Vec<Symbol>,
    correct_sequence: Vec<Symbol>,
}

// Lever Combination
struct LeverCombinationPuzzle {
    levers: Vec<Entity>, // lever entities with LeverState
    correct_states: Vec<LeverState>,
}
```

**Relationships**:
- Exists in Room (many:1)
- May reference Items, Doors, or other Entities (1:many)

**State Transitions**:
```
Unsolved --[player interaction]--> InProgress
InProgress --[correct solution]--> Solved
InProgress --[incorrect attempt]--> Unsolved (or stay InProgress)
```

**Validation**:
- PuzzleReward references valid entities (RoomId, ItemType)
- Sub-component data is consistent (e.g., lever count matches correct_states length)

---

### 9. Map

**Components**:
- `MapState` (Resource, not entity): HashMap<RoomId, ExploredStatus>
- `ExploredStatus` (struct: visited: bool, layout_data: Option<TileGrid>)

**Relationships**:
- Tracks all Rooms (1:many)

**Validation**:
- MapState contains entry for every RoomId
- layout_data populated only if visited == true

---

### 10. Diary Page

**Components**:
- `DiaryPage` (struct: page_id: usize, content: String)
- `Item` (DiaryPage variant)
- `Transform` (position when not collected)
- `Sprite` (icon)
- `Collider` (AABB)
- `Collectible` (marker)

**Relationships**:
- Hidden in Rooms (many:1)
- Collected in Player Inventory (many:1)

**Validation**:
- page_id must be unique (0 to max_pages)
- content.len() > 0

---

## Global Resources

### GameState

```rust
struct GameState {
    current_room: RoomId,
    player_spawn_point: Vec2, // last checkpoint
    completion_time: Duration, // time since game start
    collected_secrets: HashSet<ItemId>, // diary pages, bonus items
    game_mode: GameMode, // Playing, Paused, GameOver, Victory
}

enum GameMode { Menu, Playing, Paused, GameOver, Victory }
```

### InputConfig

```rust
struct InputConfig {
    key_bindings: HashMap<PlayerAction, Vec<KeyCode>>,
    gamepad_bindings: HashMap<PlayerAction, Vec<GamepadButtonType>>,
}

enum PlayerAction {
    MoveLeft, MoveRight, Jump, Interact, ToggleCandle,
    UseItem, OpenInventory, OpenMap, Pause
}
```

### AssetHandles

```rust
struct AssetHandles {
    sprites: HashMap<SpriteType, Handle<Image>>,
    audio: HashMap<SoundType, Handle<AudioSource>>,
    fonts: HashMap<FontType, Handle<Font>>,
    levels: HashMap<RoomId, Handle<LevelData>>,
}
```

### MapState

```rust
struct MapState {
    explored_rooms: HashMap<RoomId, ExploredStatus>,
}
```

---

## Data Relationships Diagram

```
Player (1) --owns--> (1) Candle
Player (1) --contains--> (*) Item
Player (*) --exists_in--> (1) Room

Room (1) --contains--> (*) Item
Room (1) --contains--> (*) Trap
Room (1) --contains--> (*) EnvironmentalHazard
Room (1) --contains--> (*) Puzzle
Room (1) --contains--> (*) Door
Room (*) --connected_to--> (*) Room

Door (*) --connects--> (2) Room
Door (*) --unlocks_with--> (1) Key

Puzzle (1) --rewards--> (1) Door | Item | Passage

MapState (1) --tracks--> (*) Room
GameState (1) --references--> (1) Room (current)
GameState (1) --references--> (1) Player (spawn point)
```

---

## Serialization Schema (Save/Load)

### SaveData

```rust
struct SaveData {
    version: u32, // for migration
    player_state: PlayerSaveState,
    room_state: HashMap<RoomId, RoomSaveState>,
    map_state: HashMap<RoomId, bool>, // explored flag
    game_stats: GameStats,
}

struct PlayerSaveState {
    current_room: RoomId,
    spawn_position: Vec2,
    inventory: Vec<Item>,
    candle_wax: f32,
    candle_state: CandleState,
    double_jump_unlocked: bool,
}

struct RoomSaveState {
    collected_items: HashSet<ItemId>, // removed from room
    puzzle_states: HashMap<PuzzleId, PuzzleState>,
    unlocked_doors: HashSet<DoorId>,
}

struct GameStats {
    completion_time: Duration,
    collected_diary_pages: HashSet<usize>,
    deaths: u32,
}
```

---

## Validation Rules Summary

1. **Player**:
   - Inventory size <= MAX_INVENTORY_SIZE
   - Position within room bounds
   - CandleHolder references valid entity

2. **Candle**:
   - Wax: 0.0 <= wax <= 100.0
   - Radius: 1.0 <= radius <= 8.0
   - Lit state requires wax > 0.0

3. **Item**:
   - Stackable items have count > 0
   - Diary page IDs are unique

4. **Room**:
   - RoomId unique
   - Bounds min < max
   - Connections reference valid rooms
   - CollisionMap matches TileMap size

5. **Door**:
   - TargetRoom is valid RoomId
   - Position within room bounds

6. **Trap**:
   - Trigger parameters > 0.0
   - Position within room bounds

7. **Puzzle**:
   - Reward references valid entities
   - Sub-component data consistent

8. **Global State**:
   - GameState.current_room is valid
   - MapState has entry for all rooms
   - AssetHandles populated at startup

---

**Status**: Data model complete. Ready for contract generation.
