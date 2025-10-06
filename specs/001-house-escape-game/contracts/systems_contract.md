# Systems Contract: House Escape Game

**Date**: 2025-10-03
**Feature**: House Escape Game (001)

## Overview

System interfaces define the contract between game logic (systems) and game state (components/resources). Each system has clear inputs, outputs, and ordering constraints.

---

## Player Movement System

**Input**:
- Query: `(Entity, &Player, &mut Transform, &mut Velocity, &JumpState, &Collider)`
- Resource: `Res<InputConfig>`, `Res<Input<KeyCode>>`, `Res<Time>`
- Resource: `Res<GameState>` (check if playing)

**Output**:
- Mutates: `Transform.translation` (horizontal movement)
- Mutates: `Velocity.y` (jump physics)
- Mutates: `JumpState` (grounded/jumping/falling transitions)
- Emits: `PlayerMovedEvent`

**Behavior**:
1. Read input from configured keys (left/right/jump)
2. Apply horizontal velocity based on input
3. Handle jump logic:
   - If grounded + jump input → set upward velocity, transition to Jumping
   - If jumping + double jump unlocked + jump input → add upward velocity, transition to DoubleJumping
   - If airborne → apply gravity to velocity
4. Update Transform.translation based on Velocity * delta_time
5. Check collision with CollisionMap, resolve penetration
6. Update JumpState based on ground contact

**Ordering**:
- Before: CollisionSystem
- After: InputHandlingSystem

**Failure Modes**:
- Player falls through floor if CollisionSystem not run after
- Player moves while paused if GameState check missing

---

## Candle Burn System

**Input**:
- Query: `(Entity, &Candle, &mut CandleWax, &mut CandleState, &mut VisibilityRadius)`
- Resource: `Res<Time>`
- Resource: `Res<GameState>` (check if playing)

**Output**:
- Mutates: `CandleWax.0` (decreases over time if lit)
- Mutates: `CandleState` (lit → extinguished when wax depletes)
- Mutates: `VisibilityRadius.0` (adjusts based on candle state)
- Emits: `CandleExtinguishedEvent` (when wax == 0.0)

**Behavior**:
1. For each Candle with CandleState::Lit:
   - Decrease CandleWax by BurnRate * delta_time
   - Clamp CandleWax to [0.0, 100.0]
2. If CandleWax == 0.0:
   - Set CandleState to Extinguished
   - Set VisibilityRadius to 1.5 (minimal)
   - Emit CandleExtinguishedEvent
3. If CandleState changed:
   - Update VisibilityRadius (Lit: 7.0, Unlit: 1.5, Extinguished: 1.5)

**Ordering**:
- Before: LightingRenderSystem
- After: None (independent)

**Failure Modes**:
- Candle continues burning when paused if GameState check missing
- Negative wax if clamp missing
- Instant transition without audio cue if event not emitted

---

## Collision Detection System

**Input**:
- Query: `(Entity, &Transform, &Collider, &Player)` (for player)
- Query: `(Entity, &Transform, &Collider, &Trap)` (for traps)
- Query: `(Entity, &Transform, &Collider, &Item, &Collectible)` (for items)
- Query: `(Entity, &Transform, &Collider, &Door)` (for doors)

**Output**:
- Emits: `TrapTriggeredEvent(Entity, Entity)` (trap, player)
- Emits: `ItemCollectedEvent(Entity, ItemId)` (item, player)
- Emits: `DoorInteractEvent(Entity, Entity)` (door, player)

**Behavior**:
1. For each Player:
   - Check AABB overlap with all Trap colliders → emit TrapTriggeredEvent
   - Check AABB overlap with all Item colliders → emit ItemCollectedEvent
   - Check AABB overlap with all Door colliders (if within range) → enable interaction prompt
2. Broad-phase: Use spatial hashing to reduce O(n²) checks
3. Narrow-phase: AABB intersection test

**Ordering**:
- After: PlayerMovementSystem
- Before: TrapActivationSystem, InventorySystem, DoorSystem

**Failure Modes**:
- Missed collisions if run before movement updates positions
- False positives if collider sizes incorrect

---

## Trap Activation System

**Input**:
- EventReader: `TrapTriggeredEvent`
- Query: `(Entity, &mut TrapState, &Trap, &Transform)`
- Query: `(Entity, &mut Health, &Player)` (for player)

**Output**:
- Mutates: `TrapState` (Armed → Triggered)
- Mutates: `Health` (Alive → Dead)
- Emits: `PlayerDeathEvent`
- Emits: `TrapAnimationEvent` (for visual feedback)

**Behavior**:
1. For each TrapTriggeredEvent:
   - Set TrapState to Triggered
   - Emit TrapAnimationEvent (play trap animation)
   - Set Player Health to Dead
   - Emit PlayerDeathEvent
2. For each Trap with TrapState::Triggered:
   - Advance animation state
   - On animation complete → set TrapState to Resetting
3. For each Trap with TrapState::Resetting:
   - Wait for reset timer → set TrapState to Armed

**Ordering**:
- After: CollisionDetectionSystem
- Before: RespawnSystem

**Failure Modes**:
- Player death not triggered if Health mutation missing
- Trap stays triggered forever if state machine incomplete

---

## Respawn System

**Input**:
- EventReader: `PlayerDeathEvent`
- Query: `(Entity, &mut Transform, &mut Health, &Player)`
- Resource: `Res<GameState>` (spawn_point)

**Output**:
- Mutates: `Transform.translation` (reset to spawn point)
- Mutates: `Health` (Dead → Alive)
- Mutates: `CandleState` (preserved per FR-041)
- Emits: `PlayerRespawnedEvent`

**Behavior**:
1. On PlayerDeathEvent:
   - Wait for death animation duration (e.g., 1 second)
   - Set Player Transform to GameState.player_spawn_point
   - Set Health to Alive
   - Preserve Candle state (wax, lit/unlit)
   - Reset all Trap states in current room to Armed
   - Emit PlayerRespawnedEvent

**Ordering**:
- After: TrapActivationSystem
- Before: None

**Failure Modes**:
- Player respawns at wrong location if spawn_point not updated on room entry
- Inventory loss if not preserved (violates FR-039)

---

## Inventory System

**Input**:
- EventReader: `ItemCollectedEvent`, `ItemUsedEvent`
- Query: `(Entity, &mut Inventory, &Player)`
- Query: `(Entity, &Item, &Transform)` (for items in world)

**Output**:
- Mutates: `Inventory` (add/remove items)
- Commands: Despawn collected items
- Emits: `InventoryUpdatedEvent`
- Emits: `ItemPickupSoundEvent`

**Behavior**:
1. On ItemCollectedEvent:
   - Check Inventory.len() < MAX_INVENTORY_SIZE
   - If stackable (Match):
     - Find existing stack or add new entry
     - Increment stack count
   - Else (unique item):
     - Add ItemId to Inventory
   - Despawn item entity from world
   - Emit InventoryUpdatedEvent
   - Emit ItemPickupSoundEvent
2. On ItemUsedEvent:
   - Remove item from Inventory (or decrement stack)
   - Apply item effect (e.g., Match → light candle, Key → unlock door)
   - Emit InventoryUpdatedEvent

**Ordering**:
- After: CollisionDetectionSystem
- Before: UI Update System

**Failure Modes**:
- Inventory overflow if capacity check missing
- Item duplication if despawn missing
- Stack count underflow if decrement not bounds-checked

---

## Room Transition System

**Input**:
- EventReader: `DoorInteractEvent`
- Query: `(Entity, &Door, &DoorState, &TargetRoom)`
- Query: `(Entity, &mut Transform, &Player)`
- Resource: `ResMut<GameState>`
- Resource: `ResMut<MapState>`

**Output**:
- Mutates: `GameState.current_room`
- Mutates: `Player Transform` (move to new room spawn)
- Mutates: `MapState.explored_rooms` (mark new room as explored)
- Commands: Despawn old room entities, spawn new room entities
- Emits: `RoomChangedEvent`, `AutoSaveEvent`

**Behavior**:
1. On DoorInteractEvent:
   - Check DoorState (if Locked, require Key in Inventory)
   - If unlocked:
     - Despawn all entities in current room (except Player, Candle)
     - Load new room level data (tiles, entities, items)
     - Spawn new room entities
     - Set Player Transform to new room spawn point
     - Update GameState.current_room
     - Mark new room as explored in MapState
     - Emit RoomChangedEvent
     - Emit AutoSaveEvent (per FR: auto-save on room entry)

**Ordering**:
- After: CollisionDetectionSystem, DoorSystem
- Before: SaveLoadSystem

**Failure Modes**:
- Room data not loaded → crash or empty room
- Old entities not despawned → memory leak
- Auto-save not triggered → progress loss on crash

---

## Save/Load System

**Input**:
- EventReader: `AutoSaveEvent`, `ManualSaveEvent`, `LoadGameEvent`
- Query: All relevant entities (Player, Inventory, Candle, Rooms, Puzzles)
- Resource: `Res<GameState>`, `Res<MapState>`

**Output**:
- File I/O: Write SaveData to disk (RON format)
- Commands: Spawn loaded entities, despawn old entities
- Emits: `SaveCompleteEvent`, `LoadCompleteEvent`

**Behavior**:
1. On AutoSaveEvent / ManualSaveEvent:
   - Collect current state:
     - Player position, inventory, candle state
     - Current room, explored rooms
     - Puzzle states, unlocked doors, collected items
     - Game stats (time, deaths, diary pages)
   - Serialize to SaveData struct
   - Write to platform-specific save directory
   - Emit SaveCompleteEvent
2. On LoadGameEvent:
   - Read SaveData from disk
   - Deserialize to structs
   - Despawn all current entities
   - Spawn loaded state (player, room, items, etc.)
   - Update GameState, MapState resources
   - Emit LoadCompleteEvent

**Ordering**:
- After: RoomTransitionSystem (on AutoSaveEvent)
- Before: None (independent file I/O)

**Failure Modes**:
- Save corruption if incomplete write (use atomic write)
- Load failure if version mismatch (implement migration)
- File permissions issue (handle with Result, show error to user)

---

## Puzzle Interaction System

**Input**:
- EventReader: `PuzzleInteractEvent`
- Query: `(Entity, &mut PuzzleState, &Puzzle, &PuzzleReward)`
- Query: `(Entity, &Inventory, &Player)` (for item-based puzzles)
- Resource: `ResMut<GameState>`

**Output**:
- Mutates: `PuzzleState` (Unsolved → InProgress → Solved)
- Commands: Spawn reward items, unlock doors, reveal passages
- Emits: `PuzzleSolvedEvent`, `DoorUnlockedEvent`

**Behavior**:
1. On PuzzleInteractEvent:
   - Check puzzle type:
     - CircuitBreaker: Verify fuse items in inventory, correct sequence
     - PressurePlate: Verify items placed on plates
     - SymbolMatch: Check input sequence against correct sequence
     - LeverCombination: Check lever states
   - If correct solution:
     - Set PuzzleState to Solved
     - Apply PuzzleReward (unlock door, spawn item, reveal passage)
     - Emit PuzzleSolvedEvent
   - Else:
     - Provide feedback (visual/audio cue)
     - Keep PuzzleState as InProgress (or reset to Unsolved)

**Ordering**:
- After: CollisionDetectionSystem
- Before: DoorSystem, ItemSpawnSystem

**Failure Modes**:
- Puzzle solves without correct input if validation logic incomplete
- Reward not granted if command missing

---

## Lighting Render System

**Input**:
- Query: `(Entity, &Transform, &VisibilityRadius, &Candle)`
- Query: `(Entity, &Transform, &Sprite)` (all visible entities)
- Resource: `Res<CameraTransform>`

**Output**:
- Render commands: Apply lighting shader to screen
- Shader uniform: `light_position: Vec2`, `light_radius: f32`, `fog_of_war_texture: Handle<Image>`

**Behavior**:
1. For each Candle:
   - Pass Candle Transform and VisibilityRadius to shader
2. Render fog of war:
   - Use MapState explored rooms to generate fog texture
   - Apply fog texture as shader input
3. Fragment shader:
   - Calculate distance from pixel to light source
   - Apply circular gradient (bright at center, dark at edge)
   - Blend fog of war (darken unexplored areas)
4. Render final lit scene

**Ordering**:
- After: CandleBurnSystem (for updated VisibilityRadius)
- Before: UI Rendering

**Failure Modes**:
- Lighting flicker if radius changes too quickly (smooth interpolation needed)
- Performance drop if shader too expensive (optimize with lookup tables)

---

## UI HUD System

**Input**:
- Query: `(Entity, &Candle, &CandleWax)`
- Query: `(Entity, &Inventory, &Player)`
- Resource: `Res<GameState>`

**Output**:
- UI rendering: Candle meter, match count, inventory bar, room name

**Behavior**:
1. Update candle meter visual (progress bar from 0-100%)
2. Display match count (from Inventory, count StackableItem::Match)
3. Render inventory bar (icons for each item, max 10 slots)
4. Show room name on room entry (fade in/out after 2 seconds)

**Ordering**:
- After: InventorySystem, CandleBurnSystem
- Before: None (final UI pass)

**Failure Modes**:
- UI desync if queries run in wrong order
- Visual glitches if inventory exceeds UI slots

---

## System Execution Order Summary

```
1. InputHandlingSystem
2. PlayerMovementSystem
3. CollisionDetectionSystem
4. TrapActivationSystem
5. RespawnSystem
6. InventorySystem
7. PuzzleInteractionSystem
8. DoorSystem
9. RoomTransitionSystem
10. SaveLoadSystem
11. CandleBurnSystem
12. LightingRenderSystem
13. UIHUDSystem
14. AudioSystem
```

---

## Event Contracts

### PlayerMovedEvent
```rust
struct PlayerMovedEvent {
    entity: Entity,
    old_position: Vec2,
    new_position: Vec2,
}
```

### TrapTriggeredEvent
```rust
struct TrapTriggeredEvent {
    trap: Entity,
    player: Entity,
}
```

### PlayerDeathEvent
```rust
struct PlayerDeathEvent {
    player: Entity,
    cause: DeathCause, // Trap, Fall, etc.
}
```

### RoomChangedEvent
```rust
struct RoomChangedEvent {
    old_room: RoomId,
    new_room: RoomId,
    player: Entity,
}
```

### AutoSaveEvent
```rust
struct AutoSaveEvent {
    trigger: SaveTrigger, // RoomTransition, Manual, Periodic
}
```

### PuzzleSolvedEvent
```rust
struct PuzzleSolvedEvent {
    puzzle: Entity,
    reward: PuzzleReward,
}
```

---

**Status**: System contracts complete. Ready for quickstart.md generation.
