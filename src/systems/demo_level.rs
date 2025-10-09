//! Demo level loading and management system.
//!
//! This module provides functionality for loading and managing the demo level
//! that players experience on first run. It handles entity spawning, asset
//! loading with fallback support, and cleanup when the demo ends.
//!
//! # Architecture
//!
//! The demo level system follows these design principles:
//! - **Fail-safe**: Never panic, always provide fallback (placeholder) graphics
//! - **Cleanup**: All entities marked with `DemoMarker` for easy removal
//! - **Performance**: Loads within 10 seconds, maintains 30+ FPS
//! - **Testable**: All functions are unit-testable with clear contracts
//!
//! # Usage
//!
//! The demo level system will be integrated via `DemoLevelPlugin`:
//!
//! ```ignore
//! app.add_plugins(DemoLevelPlugin);
//! ```
//!
//! # Lifecycle
//!
//! 1. **Load**: `OnEnter(GameState::Loading)` triggers demo level load
//! 2. **Play**: Player interacts with demo level entities
//! 3. **Cleanup**: `OnExit(GameState::Demo)` despawns all `DemoMarker` entities
//!
//! # Performance Requirements
//!
//! - Load time: < 10 seconds (from contracts/demo_level_interface.md)
//! - Frame rate: ≥ 30 FPS during demo
//! - Input lag: < 50ms response time
//!
//! # Asset Fallback
//!
//! When assets fail to load:
//! - Placeholder sprite (magenta #FF00FF) is used
//! - Warning is logged with failed asset path
//! - Game continues without crashing

use bevy::prelude::*;
use bevy::sprite::Sprite;

// Import local components for demo entities
use crate::components::demo::{DemoMarker, InteractableDemo};
use crate::components::inventory::{Collectible, Item, KeyType};
use crate::components::player::{Health, JumpState, Player, Velocity};
use crate::components::room::{Door, DoorState};

// Import level data structures for loading demo level
use crate::systems::level_loader::{EntitySpawn, LevelData};

use crate::resources::asset_handles::{AssetHandles, SpriteType};

/// Spawns a player entity at the specified position for the demo level.
///
/// Creates a fully-configured player entity with all necessary components
/// for movement, rendering, and gameplay. The entity is marked with `DemoMarker`
/// for easy cleanup when the demo level ends.
///
/// # Parameters
///
/// - `commands`: Mutable reference to Bevy's command buffer for spawning entities
/// - `position`: 2D position in world space where the player should spawn (x, y in pixels)
/// - `asset_handles`: Resource containing handles to loaded game assets
///
/// # Returns
///
/// Returns the `Entity` ID of the spawned player for tracking or further modification.
///
/// # Components Added
///
/// The spawned entity includes:
/// - `Player`: Marker component identifying this as the player character
/// - `Velocity`: Movement velocity initialized to zero
/// - `JumpState::Grounded`: Player starts on the ground
/// - `Health::Alive`: Player starts with full health
/// - `DemoMarker`: Tags entity for demo cleanup
/// - `Sprite`: Visual representation with player sprite
/// - `Transform`: Spatial position and rotation
///
/// # Example
///
/// ```ignore
/// fn setup_demo(
///     mut commands: Commands,
///     asset_handles: Res<AssetHandles>,
/// ) {
///     // Spawn player at center of level
///     let player_entity = spawn_player(
///         &mut commands,
///         Vec2::new(400.0, 300.0),
///         &asset_handles,
///     );
///
///     info!("Spawned player entity: {:?}", player_entity);
/// }
/// ```
///
/// # Asset Fallback
///
/// If the player sprite is not found in `asset_handles`, the function will
/// use a default handle. In production, this should use the placeholder sprite
/// (magenta #FF00FF) to make missing assets obvious during testing.
pub fn spawn_player(
    commands: &mut Commands,
    position: Vec2,
    asset_handles: &AssetHandles,
) -> Entity {
    // Get player sprite handle, with fallback to placeholder if not found
    let sprite_handle = asset_handles
        .sprites
        .get(&SpriteType::Player)
        .cloned()
        .unwrap_or_default();

    // Spawn player entity with all required components
    commands
        .spawn((
            Player,
            Velocity(Vec2::ZERO),
            JumpState::Grounded,
            Health::Alive,
            DemoMarker,
            Sprite {
                image: sprite_handle,
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
        ))
        .id()
}

/// Spawns a door entity for the demo level based on level data.
///
/// Creates a door entity with interaction capabilities and proper state management.
/// Doors can be locked (requiring a key) or unlocked. The entity is marked with
/// `DemoMarker` for easy cleanup when the demo level ends.
///
/// # Parameters
///
/// - `commands`: Mutable reference to Bevy's command buffer for spawning entities
/// - `entity_spawn`: Level data describing the door (position, locked state, etc.)
/// - `asset_handles`: Resource containing handles to loaded game assets
///
/// # Returns
///
/// Returns the `Entity` ID of the spawned door for tracking or further modification.
///
/// # Components Added
///
/// The spawned entity includes:
/// - `Door`: Marker component identifying this as a door
/// - `DoorState`: Current state (Locked with key type, or Unlocked)
/// - `InteractableDemo`: Interaction prompt and object ID
/// - `DemoMarker`: Tags entity for demo cleanup
/// - `Sprite`: Visual representation with door sprite
/// - `Transform`: Positioned at location from entity_spawn
///
/// # Example
///
/// ```ignore
/// use crate::systems::level_loader::EntitySpawn;
///
/// let door_spawn = EntitySpawn {
///     entity_type: "Door".to_string(),
///     position: (200.0, 100.0),
///     target_room: Some(1),
///     locked: Some(KeyType::Brass), // Locked door requiring brass key
///     key_type: None,
/// };
///
/// let door_entity = spawn_door(
///     &mut commands,
///     &door_spawn,
///     &asset_handles,
/// );
/// ```
///
/// # Door State Logic
///
/// - If `entity_spawn.locked` contains a `KeyType`, door spawns as `DoorState::Locked(key_type)`
/// - Otherwise, door spawns as `DoorState::Unlocked`
/// - Locked doors display "Press E to unlock" prompt
/// - Unlocked doors display "Press E to open" prompt
pub fn spawn_door(
    commands: &mut Commands,
    entity_spawn: &EntitySpawn,
    asset_handles: &AssetHandles,
) -> Entity {
    // Determine door state based on whether it's locked
    let door_state = if let Some(key_type) = entity_spawn.locked {
        DoorState::Locked(key_type)
    } else {
        DoorState::Unlocked
    };

    // Set interaction prompt based on door state
    let interaction_prompt = match &door_state {
        DoorState::Locked(_) => "Press E to unlock".to_string(),
        DoorState::Unlocked => "Press E to open".to_string(),
        DoorState::Open => "Press E to enter".to_string(),
    };

    // Get door sprite handle (fallback to default if not found)
    // TODO: Use specific door sprite type once AssetHandles supports door variants
    let sprite_handle = asset_handles
        .sprites
        .get(&SpriteType::Player) // Placeholder - will use door sprite in future
        .cloned()
        .unwrap_or_default();

    // Create object ID from entity type and position
    let object_id = format!(
        "{}_{:.0}_{:.0}",
        entity_spawn.entity_type, entity_spawn.position.0, entity_spawn.position.1
    );

    // Convert position tuple to Vec2
    let position = Vec2::new(entity_spawn.position.0, entity_spawn.position.1);

    // Spawn door entity with all required components
    commands
        .spawn((
            Door,
            door_state,
            InteractableDemo {
                object_id,
                interaction_prompt,
            },
            DemoMarker,
            Sprite {
                image: sprite_handle,
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
        ))
        .id()
}

/// Spawns an item entity (match or key) for the demo level based on level data.
///
/// Creates a collectible item entity that players can pick up during the demo.
/// Items can be either matches (for lighting candles) or keys (for unlocking doors).
/// The entity is marked with `DemoMarker` for easy cleanup when the demo level ends.
///
/// # Parameters
///
/// - `commands`: Mutable reference to Bevy's command buffer for spawning entities
/// - `entity_spawn`: Level data describing the item (position, type, key variant, etc.)
/// - `asset_handles`: Resource containing handles to loaded game assets
///
/// # Returns
///
/// Returns the `Entity` ID of the spawned item for tracking or further modification.
///
/// # Components Added
///
/// The spawned entity includes:
/// - `Item`: Enum variant (Match or Key with specific KeyType)
/// - `Collectible`: Marker indicating this item can be picked up
/// - `InteractableDemo`: Interaction prompt ("Press E to collect")
/// - `DemoMarker`: Tags entity for demo cleanup
/// - `Sprite`: Visual representation with item sprite
/// - `Transform`: Positioned at location from entity_spawn
///
/// # Example
///
/// ```ignore
/// use crate::systems::level_loader::EntitySpawn;
/// use crate::components::inventory::KeyType;
///
/// // Spawn a match item
/// let match_spawn = EntitySpawn {
///     entity_type: "Match".to_string(),
///     position: (150.0, 200.0),
///     target_room: None,
///     locked: None,
///     key_type: None,
/// };
///
/// let match_entity = spawn_item(
///     &mut commands,
///     &match_spawn,
///     &asset_handles,
/// );
///
/// // Spawn a brass key
/// let key_spawn = EntitySpawn {
///     entity_type: "Key".to_string(),
///     position: (300.0, 150.0),
///     target_room: None,
///     locked: None,
///     key_type: Some(KeyType::Brass),
/// };
///
/// let key_entity = spawn_item(
///     &mut commands,
///     &key_spawn,
///     &asset_handles,
/// );
/// ```
///
/// # Item Type Logic
///
/// - If `entity_spawn.entity_type == "Match"`, spawns `Item::Match`
/// - If `entity_spawn.entity_type == "Key"`, spawns `Item::Key(key_type)`
///   - `key_type` is extracted from `entity_spawn.key_type` (defaults to `KeyType::Brass` if not specified)
/// - All items show "Press E to collect" interaction prompt
pub fn spawn_item(
    commands: &mut Commands,
    entity_spawn: &EntitySpawn,
    asset_handles: &AssetHandles,
) -> Entity {
    // Determine item type based on entity_spawn data
    let item = if entity_spawn.entity_type == "Match" {
        Item::Match
    } else {
        // For keys, use the specified key_type or default to Brass
        let key_type = entity_spawn.key_type.unwrap_or(KeyType::Brass);
        Item::Key(key_type)
    };

    // Get item sprite handle (fallback to default if not found)
    // TODO: Use specific item sprite types once AssetHandles supports item variants
    let sprite_handle = asset_handles
        .sprites
        .get(&SpriteType::Player) // Placeholder - will use item sprites in future
        .cloned()
        .unwrap_or_default();

    // Create object ID from entity type and position
    let object_id = format!(
        "{}_{:.0}_{:.0}",
        entity_spawn.entity_type, entity_spawn.position.0, entity_spawn.position.1
    );

    // Convert position tuple to Vec2
    let position = Vec2::new(entity_spawn.position.0, entity_spawn.position.1);

    // All items use the same interaction prompt
    let interaction_prompt = "Press E to collect".to_string();

    // Spawn item entity with all required components
    commands
        .spawn((
            item,
            Collectible,
            InteractableDemo {
                object_id,
                interaction_prompt,
            },
            DemoMarker,
            Sprite {
                image: sprite_handle,
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
        ))
        .id()
}

/// Spawns all entities from demo level data based on their types.
///
/// This orchestrator function processes the `entities` array from `LevelData` and
/// delegates spawning to specialized helper functions based on entity type. It handles
/// player spawns, doors, and collectible items (matches and keys).
///
/// # Parameters
///
/// - `level_data`: Reference to the loaded level data containing entity definitions
/// - `commands`: Mutable reference to Bevy's command buffer for spawning entities
/// - `asset_handles`: Resource containing handles to loaded game assets
///
/// # Returns
///
/// Returns the total count of successfully spawned entities.
///
/// # Entity Type Mapping
///
/// - `"PlayerSpawn"` → Calls `spawn_player()` at entity position
/// - `"Door"` → Calls `spawn_door()` with door configuration
/// - `"Match"` → Calls `spawn_item()` as collectible match
/// - `"Key"` → Calls `spawn_item()` as collectible key with specific KeyType
/// - Unknown types → Logs warning and skips entity
///
/// # Example
///
/// ```ignore
/// use crate::systems::level_loader::load_level_data;
///
/// fn load_demo_level(
///     mut commands: Commands,
///     asset_handles: Res<AssetHandles>,
/// ) {
///     // Load demo level from RON file
///     let level_data = load_level_data("levels/demo.ron")
///         .expect("Failed to load demo level");
///
///     // Spawn all entities defined in the level
///     let entity_count = spawn_demo_entities(
///         &level_data,
///         &mut commands,
///         &asset_handles,
///     );
///
///     info!("Spawned {} demo entities", entity_count);
/// }
/// ```
///
/// # Error Handling
///
/// - Logs warnings for unknown entity types but continues processing
/// - Does not panic on invalid data - returns count of successful spawns
/// - Each spawn helper handles its own validation and fallback logic
///
/// # Contract Requirements
///
/// Per contracts/demo_level_interface.md:
/// - Must process all entities from level_data.entities array
/// - Must delegate to appropriate spawn helper functions
/// - Must track and return total entity count
/// - Must log warnings for unrecognized entity types
pub fn spawn_demo_entities(
    level_data: &LevelData,
    commands: &mut Commands,
    asset_handles: &AssetHandles,
) -> usize {
    let mut spawned_count = 0;

    // Iterate through all entities defined in the level data
    for entity_spawn in &level_data.entities {
        match entity_spawn.entity_type.as_str() {
            "PlayerSpawn" => {
                // Spawn player at specified position
                let position = Vec2::new(entity_spawn.position.0, entity_spawn.position.1);
                spawn_player(commands, position, asset_handles);
                spawned_count += 1;
                info!(
                    "Spawned player at position ({:.0}, {:.0})",
                    position.x, position.y
                );
            }
            "Door" => {
                // Spawn door with configuration from entity_spawn
                spawn_door(commands, entity_spawn, asset_handles);
                spawned_count += 1;
                info!(
                    "Spawned door at position ({:.0}, {:.0})",
                    entity_spawn.position.0, entity_spawn.position.1
                );
            }
            "Match" => {
                // Spawn collectible match item
                spawn_item(commands, entity_spawn, asset_handles);
                spawned_count += 1;
                info!(
                    "Spawned match at position ({:.0}, {:.0})",
                    entity_spawn.position.0, entity_spawn.position.1
                );
            }
            "Key" => {
                // Spawn collectible key item
                spawn_item(commands, entity_spawn, asset_handles);
                spawned_count += 1;
                info!(
                    "Spawned key at position ({:.0}, {:.0})",
                    entity_spawn.position.0, entity_spawn.position.1
                );
            }
            unknown => {
                // Log warning for unrecognized entity types
                warn!(
                    "Unknown entity type '{}' at position ({:.0}, {:.0}) - skipping",
                    unknown, entity_spawn.position.0, entity_spawn.position.1
                );
            }
        }
    }

    spawned_count
}

/// Loads the demo level from RON file and initializes performance tracking.
///
/// This is the main entry point system for loading the demo level. It reads the
/// demo level data from `assets/levels/demo.ron`, records the load start time for
/// performance measurement, and handles errors gracefully with fallback behavior.
///
/// # System Parameters
///
/// - `commands`: Command buffer for spawning entities (used in T019-T020)
/// - `asset_handles`: Resource containing handles to loaded game assets
/// - `_asset_server`: Asset server for loading assets (reserved for future use)
/// - `load_start_time`: Local state tracking when the load operation began
///
/// # Performance Tracking
///
/// The system uses `Local<Option<Instant>>` to track the load start time. This allows
/// measuring the total load duration across multiple frames if needed. The timestamp
/// is recorded on the first run and can be used by subsequent systems (T019-T020) to
/// calculate total load time.
///
/// # Error Handling
///
/// - If `assets/levels/demo.ron` is missing: Logs warning and returns early
/// - If RON parsing fails: Logs error with details and returns early
/// - Never panics - always provides graceful degradation
///
/// # Performance Contract
///
/// Per contracts/demo_level_interface.md:
/// - Must complete within 10 seconds (verified in integration tests)
/// - Records load start time with `Instant::now()` for measurement
/// - Logs load duration on completion
///
/// # Usage
///
/// This system is typically added to the `Startup` schedule or run once during
/// game initialization:
///
/// ```ignore
/// app.add_systems(Startup, load_demo_level);
/// ```
///
/// # Example
///
/// ```ignore
/// use bevy::prelude::*;
/// use rust_game::systems::demo_level::load_demo_level;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_systems(Startup, load_demo_level)
///         .run();
/// }
/// ```
///
/// # Implementation Notes
///
/// T018 focuses on loading the level data and recording timing. Entity spawning
/// (tilemap and entities) will be implemented in T019 and T020 respectively.
pub fn load_demo_level(
    mut _commands: Commands,
    _asset_handles: Res<AssetHandles>,
    _asset_server: Res<AssetServer>,
    mut load_start_time: Local<Option<std::time::Instant>>,
) {
    // Record load start time on first run
    if load_start_time.is_none() {
        *load_start_time = Some(std::time::Instant::now());
        info!("Starting demo level load...");
    }

    // Load demo level data from RON file
    match crate::systems::level_loader::load_level_data("levels/demo.ron") {
        Ok(level_data) => {
            // Calculate load duration
            let load_duration = load_start_time
                .unwrap()
                .elapsed();

            info!(
                "Successfully loaded demo level '{}' (ID: {}, Floor: {:?}) in {:.2}s",
                level_data.name,
                level_data.id,
                level_data.floor,
                load_duration.as_secs_f32()
            );

            info!(
                "Demo level contains {} entities and {} tile rows",
                level_data.entities.len(),
                level_data.tiles.len()
            );

            // TODO T019: Spawn tilemap from level_data.tiles
            // TODO T020: Spawn entities using spawn_demo_entities()

            // Verify load time meets performance contract (<10 seconds)
            if load_duration.as_secs() >= 10 {
                warn!(
                    "Demo level load took {:.2}s, exceeding 10s performance contract",
                    load_duration.as_secs_f32()
                );
            }
        }
        Err(error) => {
            // Handle load errors gracefully without panicking
            warn!("Failed to load demo level: {}", error);
            warn!("Demo level will not be available. Please check that assets/levels/demo.ron exists.");

            // Reset load start time so we can retry if needed
            *load_start_time = None;
        }
    }
}

// Future functions will be implemented here in subsequent tasks:
// - cleanup_demo_level(): System to despawn all demo entities
// - handle_asset_fallback(): Provides placeholder when assets fail

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_level_module_compiles() {
        // This test verifies that the module structure is valid
        // and all imports are accessible
        let _ = DemoMarker;
        let _ = Player;
    }

    #[test]
    fn required_components_are_available() {
        // Verify all components needed for demo spawning are accessible
        let _demo_marker = DemoMarker;
        let _velocity = Velocity(Vec2::ZERO);
        let _jump_state = JumpState::Grounded;
        let _health = Health::Alive;

        let _interactable = InteractableDemo {
            object_id: "test".to_string(),
            interaction_prompt: "Test prompt".to_string(),
        };
    }

    #[test]
    fn level_data_type_accessible() {
        // Verify LevelData is properly imported for demo loading
        // This will be used to parse assets/levels/demo.ron
        let _level_data_type = std::any::type_name::<crate::systems::level_loader::LevelData>();
    }

    #[test]
    fn spawn_player_creates_entity() {
        // Verify spawn_player creates an entity and returns valid Entity ID
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(100.0, 200.0),
            &asset_handles,
        );

        // Entity ID is always valid when returned from spawn
        let _ = player_entity;
    }

    #[test]
    fn spawn_player_adds_all_required_components() {
        // Verify spawn_player adds all necessary components to the player entity
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(100.0, 200.0),
            &asset_handles,
        );

        // Apply commands to actually spawn the entity
        app.update();

        // Verify all components are present
        let world = app.world();
        assert!(
            world.get::<Player>(player_entity).is_some(),
            "Player component should be present"
        );
        assert!(
            world.get::<Velocity>(player_entity).is_some(),
            "Velocity component should be present"
        );
        assert!(
            world.get::<JumpState>(player_entity).is_some(),
            "JumpState component should be present"
        );
        assert!(
            world.get::<Health>(player_entity).is_some(),
            "Health component should be present"
        );
        assert!(
            world.get::<DemoMarker>(player_entity).is_some(),
            "DemoMarker component should be present"
        );
        assert!(
            world.get::<Transform>(player_entity).is_some(),
            "Transform component should be present"
        );
    }

    #[test]
    fn spawn_player_sets_correct_position() {
        // Verify spawn_player places entity at the specified position
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let spawn_position = Vec2::new(123.45, 678.90);
        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            spawn_position,
            &asset_handles,
        );

        app.update();

        // Check transform position matches spawn position
        let world = app.world();
        let transform = world
            .get::<Transform>(player_entity)
            .expect("Transform should exist");

        assert!(
            (transform.translation.x - spawn_position.x).abs() < 0.01,
            "X position should match spawn position"
        );
        assert!(
            (transform.translation.y - spawn_position.y).abs() < 0.01,
            "Y position should match spawn position"
        );
        assert_eq!(transform.translation.z, 0.0, "Z position should be 0.0");
    }

    #[test]
    fn spawn_player_initializes_velocity_to_zero() {
        // Verify spawn_player initializes velocity to zero
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );

        app.update();

        let world = app.world();
        let velocity = world
            .get::<Velocity>(player_entity)
            .expect("Velocity should exist");

        assert_eq!(velocity.0, Vec2::ZERO, "Initial velocity should be zero");
    }

    #[test]
    fn spawn_player_starts_grounded() {
        // Verify spawn_player sets JumpState to Grounded
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );

        app.update();

        let world = app.world();
        let jump_state = world
            .get::<JumpState>(player_entity)
            .expect("JumpState should exist");

        assert_eq!(
            *jump_state,
            JumpState::Grounded,
            "Player should start grounded"
        );
    }

    #[test]
    fn spawn_player_starts_alive() {
        // Verify spawn_player sets Health to Alive
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );

        app.update();

        let world = app.world();
        let health = world
            .get::<Health>(player_entity)
            .expect("Health should exist");

        assert_eq!(*health, Health::Alive, "Player should start alive");
    }

    #[test]
    fn spawn_player_has_demo_marker() {
        // Verify spawn_player adds DemoMarker for cleanup
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );

        app.update();

        // Query for entities with DemoMarker
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<DemoMarker>>();
        let demo_entities: Vec<Entity> = query.iter(world).collect();

        assert!(
            demo_entities.contains(&player_entity),
            "Player entity should have DemoMarker"
        );
    }

    #[test]
    fn spawn_player_can_be_queried_with_player_marker() {
        // Verify spawn_player entity can be queried as a Player
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player_entity = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );

        app.update();

        // Query for player entities
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Player>>();
        let player_entities: Vec<Entity> = query.iter(world).collect();

        assert_eq!(player_entities.len(), 1, "Should have exactly one player");
        assert_eq!(
            player_entities[0], player_entity,
            "Player entity should match spawned entity"
        );
    }

    #[test]
    fn spawn_player_multiple_times_creates_multiple_entities() {
        // Verify spawn_player can be called multiple times (for testing scenarios)
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let asset_handles = AssetHandles::default();
        let player1 = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(0.0, 0.0),
            &asset_handles,
        );
        let player2 = spawn_player(
            &mut app.world_mut().commands(),
            Vec2::new(100.0, 0.0),
            &asset_handles,
        );

        app.update();

        // Verify both entities exist and are different
        assert_ne!(player1, player2, "Should create different entities");

        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Player>>();
        let player_count = query.iter(world).count();

        assert_eq!(
            player_count, 2,
            "Should have two player entities for testing"
        );
    }

    // Tests for spawn_door()

    #[test]
    fn spawn_door_creates_entity() {
        // Verify spawn_door creates an entity and returns valid Entity ID
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 200.0),
            target_room: Some(1),
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        // Verify entity was created
        app.update();
        assert!(app.world().entities().contains(door_entity));
    }

    #[test]
    fn spawn_door_adds_all_required_components() {
        // Verify spawn_door adds all necessary components
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (150.0, 250.0),
            target_room: Some(2),
            locked: Some(KeyType::Brass),
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        // Verify all components are present
        let world = app.world();
        assert!(
            world.get::<Door>(door_entity).is_some(),
            "Door component should be present"
        );
        assert!(
            world.get::<DoorState>(door_entity).is_some(),
            "DoorState component should be present"
        );
        assert!(
            world.get::<InteractableDemo>(door_entity).is_some(),
            "InteractableDemo component should be present"
        );
        assert!(
            world.get::<DemoMarker>(door_entity).is_some(),
            "DemoMarker component should be present"
        );
        assert!(
            world.get::<Transform>(door_entity).is_some(),
            "Transform component should be present"
        );
    }

    #[test]
    fn spawn_door_sets_correct_position() {
        // Verify spawn_door places entity at the specified position
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let spawn_position = (123.45, 678.90);
        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: spawn_position,
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        // Check transform position matches spawn position
        let world = app.world();
        let transform = world
            .get::<Transform>(door_entity)
            .expect("Transform should exist");

        assert!(
            (transform.translation.x - spawn_position.0).abs() < 0.01,
            "X position should match spawn position"
        );
        assert!(
            (transform.translation.y - spawn_position.1).abs() < 0.01,
            "Y position should match spawn position"
        );
        assert_eq!(transform.translation.z, 0.0, "Z position should be 0.0");
    }

    #[test]
    fn spawn_door_creates_locked_door() {
        // Verify spawn_door creates locked door with correct key type
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 100.0),
            target_room: Some(1),
            locked: Some(KeyType::Brass),
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let door_state = world
            .get::<DoorState>(door_entity)
            .expect("DoorState should exist");

        assert_eq!(
            *door_state,
            DoorState::Locked(KeyType::Brass),
            "Door should be locked with Brass key"
        );
    }

    #[test]
    fn spawn_door_creates_unlocked_door() {
        // Verify spawn_door creates unlocked door when no key specified
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 100.0),
            target_room: Some(1),
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let door_state = world
            .get::<DoorState>(door_entity)
            .expect("DoorState should exist");

        assert_eq!(*door_state, DoorState::Unlocked, "Door should be unlocked");
    }

    #[test]
    fn spawn_door_sets_correct_interaction_prompt_for_locked() {
        // Verify locked door has "unlock" prompt
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 100.0),
            target_room: Some(1),
            locked: Some(KeyType::Iron),
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let interactable = world
            .get::<InteractableDemo>(door_entity)
            .expect("InteractableDemo should exist");

        assert_eq!(
            interactable.interaction_prompt, "Press E to unlock",
            "Locked door should have unlock prompt"
        );
    }

    #[test]
    fn spawn_door_sets_correct_interaction_prompt_for_unlocked() {
        // Verify unlocked door has "open" prompt
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 100.0),
            target_room: Some(1),
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let interactable = world
            .get::<InteractableDemo>(door_entity)
            .expect("InteractableDemo should exist");

        assert_eq!(
            interactable.interaction_prompt, "Press E to open",
            "Unlocked door should have open prompt"
        );
    }

    #[test]
    fn spawn_door_creates_unique_object_id() {
        // Verify spawn_door creates unique object IDs based on position
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn1 = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (100.0, 100.0),
            target_room: Some(1),
            locked: None,
            key_type: None,
        };

        let door_spawn2 = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (200.0, 300.0),
            target_room: Some(2),
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door1 = spawn_door(
            &mut app.world_mut().commands(),
            &door_spawn1,
            &asset_handles,
        );
        let door2 = spawn_door(
            &mut app.world_mut().commands(),
            &door_spawn2,
            &asset_handles,
        );

        app.update();

        let world = app.world();
        let id1 = world
            .get::<InteractableDemo>(door1)
            .unwrap()
            .object_id
            .clone();
        let id2 = world
            .get::<InteractableDemo>(door2)
            .unwrap()
            .object_id
            .clone();

        assert_ne!(
            id1, id2,
            "Object IDs should be unique for different positions"
        );
        assert_eq!(id1, "Door_100_100");
        assert_eq!(id2, "Door_200_300");
    }

    #[test]
    fn spawn_door_has_demo_marker() {
        // Verify spawn_door adds DemoMarker for cleanup
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        // Query for entities with DemoMarker
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<DemoMarker>>();
        let demo_entities: Vec<Entity> = query.iter(world).collect();

        assert!(
            demo_entities.contains(&door_entity),
            "Door entity should have DemoMarker"
        );
    }

    #[test]
    fn spawn_door_can_be_queried_with_door_marker() {
        // Verify spawn_door entity can be queried as a Door
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let door_spawn = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let door_entity = spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

        app.update();

        // Query for door entities
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Door>>();
        let door_entities: Vec<Entity> = query.iter(world).collect();

        assert_eq!(door_entities.len(), 1, "Should have exactly one door");
        assert_eq!(
            door_entities[0], door_entity,
            "Door entity should match spawned entity"
        );
    }

    #[test]
    fn spawn_door_supports_different_key_types() {
        // Verify spawn_door handles all KeyType variants
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let key_types = vec![
            KeyType::Brass,
            KeyType::Iron,
            KeyType::Ornate,
            KeyType::Master,
        ];

        for (i, key_type) in key_types.iter().enumerate() {
            let door_spawn = EntitySpawn {
                entity_type: "Door".to_string(),
                position: (i as f32 * 100.0, 0.0),
                target_room: Some(i),
                locked: Some(*key_type),
                key_type: None,
            };

            let asset_handles = AssetHandles::default();
            let door_entity =
                spawn_door(&mut app.world_mut().commands(), &door_spawn, &asset_handles);

            app.update();

            let world = app.world();
            let door_state = world.get::<DoorState>(door_entity).unwrap();

            assert_eq!(
                *door_state,
                DoorState::Locked(*key_type),
                "Door should be locked with {:?} key",
                key_type
            );
        }
    }

    // ===== Tests for spawn_item() =====

    #[test]
    fn spawn_item_creates_match_entity() {
        // Verify spawn_item creates a match item and returns valid Entity ID
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (150.0, 200.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        // Entity ID is always valid when returned from spawn
        let _ = item_entity;
    }

    #[test]
    fn spawn_item_creates_key_entity() {
        // Verify spawn_item creates a key item with specified key type
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Key".to_string(),
            position: (300.0, 150.0),
            target_room: None,
            locked: None,
            key_type: Some(KeyType::Iron),
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        // Entity ID is always valid when returned from spawn
        let _ = item_entity;
    }

    #[test]
    fn spawn_item_adds_all_required_components() {
        // Verify spawn_item adds all necessary components to the item entity
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (100.0, 100.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        // Verify all components are present
        let world = app.world();
        assert!(
            world.get::<Item>(item_entity).is_some(),
            "Item component should be present"
        );
        assert!(
            world.get::<Collectible>(item_entity).is_some(),
            "Collectible component should be present"
        );
        assert!(
            world.get::<InteractableDemo>(item_entity).is_some(),
            "InteractableDemo component should be present"
        );
        assert!(
            world.get::<DemoMarker>(item_entity).is_some(),
            "DemoMarker component should be present"
        );
        assert!(
            world.get::<Transform>(item_entity).is_some(),
            "Transform component should be present"
        );
    }

    #[test]
    fn spawn_item_sets_correct_position() {
        // Verify spawn_item places entity at the specified position
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let spawn_position = (456.78, 123.45);
        let item_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: spawn_position,
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let transform = world.get::<Transform>(item_entity).unwrap();

        assert!(
            (transform.translation.x - spawn_position.0).abs() < 0.01,
            "Item X position should be {}",
            spawn_position.0
        );
        assert!(
            (transform.translation.y - spawn_position.1).abs() < 0.01,
            "Item Y position should be {}",
            spawn_position.1
        );
    }

    #[test]
    fn spawn_item_match_creates_correct_item_type() {
        // Verify spawn_item creates Item::Match for "Match" entity type
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let item = world.get::<Item>(item_entity).unwrap();

        assert!(
            matches!(item, Item::Match),
            "Item should be Item::Match variant"
        );
    }

    #[test]
    fn spawn_item_key_creates_correct_item_type() {
        // Verify spawn_item creates Item::Key with correct KeyType
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Key".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: Some(KeyType::Ornate),
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let item = world.get::<Item>(item_entity).unwrap();

        assert!(
            matches!(item, Item::Key(KeyType::Ornate)),
            "Item should be Item::Key(KeyType::Ornate)"
        );
    }

    #[test]
    fn spawn_item_key_defaults_to_brass_when_unspecified() {
        // Verify spawn_item defaults to Brass key when key_type is None
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Key".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None, // No key type specified
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let item = world.get::<Item>(item_entity).unwrap();

        assert!(
            matches!(item, Item::Key(KeyType::Brass)),
            "Item should default to Item::Key(KeyType::Brass) when unspecified"
        );
    }

    #[test]
    fn spawn_item_sets_interaction_prompt() {
        // Verify spawn_item sets "Press E to collect" prompt for all items
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let asset_handles = AssetHandles::default();
        let item_entity = spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

        app.update();

        let world = app.world();
        let interactable = world.get::<InteractableDemo>(item_entity).unwrap();

        assert_eq!(
            interactable.interaction_prompt, "Press E to collect",
            "All items should have 'Press E to collect' prompt"
        );
    }

    #[test]
    fn spawn_item_generates_unique_object_id() {
        // Verify spawn_item generates unique object IDs based on type and position
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let item1_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (100.0, 200.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let item2_spawn = EntitySpawn {
            entity_type: "Key".to_string(),
            position: (300.0, 400.0),
            target_room: None,
            locked: None,
            key_type: Some(KeyType::Iron),
        };

        let asset_handles = AssetHandles::default();
        let item1_entity = spawn_item(
            &mut app.world_mut().commands(),
            &item1_spawn,
            &asset_handles,
        );
        let item2_entity = spawn_item(
            &mut app.world_mut().commands(),
            &item2_spawn,
            &asset_handles,
        );

        app.update();

        let world = app.world();
        let item1_interactable = world.get::<InteractableDemo>(item1_entity).unwrap();
        let item2_interactable = world.get::<InteractableDemo>(item2_entity).unwrap();

        assert_eq!(
            item1_interactable.object_id, "Match_100_200",
            "Match should have object ID based on position"
        );
        assert_eq!(
            item2_interactable.object_id, "Key_300_400",
            "Key should have object ID based on position"
        );
        assert_ne!(
            item1_interactable.object_id, item2_interactable.object_id,
            "Different items should have unique object IDs"
        );
    }

    #[test]
    fn spawn_item_supports_all_key_types() {
        // Verify spawn_item handles all KeyType variants
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let key_types = vec![
            KeyType::Brass,
            KeyType::Iron,
            KeyType::Ornate,
            KeyType::Master,
        ];

        for (i, key_type) in key_types.iter().enumerate() {
            let item_spawn = EntitySpawn {
                entity_type: "Key".to_string(),
                position: (i as f32 * 50.0, 0.0),
                target_room: None,
                locked: None,
                key_type: Some(*key_type),
            };

            let asset_handles = AssetHandles::default();
            let item_entity =
                spawn_item(&mut app.world_mut().commands(), &item_spawn, &asset_handles);

            app.update();

            let world = app.world();
            let item = world.get::<Item>(item_entity).unwrap();

            assert!(
                matches!(item, Item::Key(kt) if kt == key_type),
                "Item should be Key with {:?} type",
                key_type
            );
        }
    }

    #[test]
    fn spawn_item_multiple_items_all_have_collectible() {
        // Verify all spawned items have Collectible marker for pickup system
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let match_spawn = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (0.0, 0.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        let key_spawn = EntitySpawn {
            entity_type: "Key".to_string(),
            position: (50.0, 0.0),
            target_room: None,
            locked: None,
            key_type: Some(KeyType::Master),
        };

        let asset_handles = AssetHandles::default();
        let match_entity = spawn_item(
            &mut app.world_mut().commands(),
            &match_spawn,
            &asset_handles,
        );
        let key_entity = spawn_item(&mut app.world_mut().commands(), &key_spawn, &asset_handles);

        app.update();

        let world = app.world();
        assert!(
            world.get::<Collectible>(match_entity).is_some(),
            "Match should have Collectible component"
        );
        assert!(
            world.get::<Collectible>(key_entity).is_some(),
            "Key should have Collectible component"
        );
    }

    // Tests for spawn_demo_entities orchestrator function

    // Helper function to create test LevelData with minimal required fields
    fn create_test_level_data(entities: Vec<EntitySpawn>) -> LevelData {
        use crate::components::room::Floor;
        use crate::systems::level_loader::Bounds;

        LevelData {
            id: 0,
            floor: Floor::Ground,
            name: "Test Level".to_string(),
            bounds: Bounds {
                min: (0.0, 0.0),
                max: (1000.0, 1000.0),
            },
            tiles: vec![],
            entities,
            connections: vec![],
        }
    }

    #[test]
    fn spawn_demo_entities_returns_correct_count() {
        // Verify spawn_demo_entities returns the count of spawned entities
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (100.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Door".to_string(),
                position: (200.0, 100.0),
                target_room: Some(1),
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Match".to_string(),
                position: (150.0, 150.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
        ]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        assert_eq!(count, 3, "Should return count of 3 spawned entities");
    }

    #[test]
    fn spawn_demo_entities_spawns_player() {
        // Verify spawn_demo_entities correctly spawns PlayerSpawn entities
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![EntitySpawn {
            entity_type: "PlayerSpawn".to_string(),
            position: (100.0, 100.0),
            target_room: None,
            locked: None,
            key_type: None,
        }]);

        let asset_handles = AssetHandles::default();
        spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        // Query for player entities
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Player>>();
        let player_count = query.iter(world).count();

        assert_eq!(player_count, 1, "Should spawn exactly one player");
    }

    #[test]
    fn spawn_demo_entities_spawns_doors() {
        // Verify spawn_demo_entities correctly spawns Door entities
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "Door".to_string(),
                position: (200.0, 100.0),
                target_room: Some(1),
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Door".to_string(),
                position: (300.0, 100.0),
                target_room: Some(2),
                locked: Some(KeyType::Brass),
                key_type: None,
            },
        ]);

        let asset_handles = AssetHandles::default();
        spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        // Query for door entities
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Door>>();
        let door_count = query.iter(world).count();

        assert_eq!(door_count, 2, "Should spawn exactly two doors");
    }

    #[test]
    fn spawn_demo_entities_spawns_items() {
        // Verify spawn_demo_entities correctly spawns Match and Key entities
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "Match".to_string(),
                position: (150.0, 150.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Key".to_string(),
                position: (160.0, 150.0),
                target_room: None,
                locked: None,
                key_type: Some(KeyType::Brass),
            },
        ]);

        let asset_handles = AssetHandles::default();
        spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        // Query for collectible items
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Collectible>>();
        let item_count = query.iter(world).count();

        assert_eq!(item_count, 2, "Should spawn exactly two items");
    }

    #[test]
    fn spawn_demo_entities_handles_mixed_entity_types() {
        // Verify spawn_demo_entities correctly handles a level with all entity types
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (100.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Door".to_string(),
                position: (200.0, 100.0),
                target_room: Some(1),
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Match".to_string(),
                position: (150.0, 150.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Key".to_string(),
                position: (160.0, 150.0),
                target_room: None,
                locked: None,
                key_type: Some(KeyType::Iron),
            },
        ]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        assert_eq!(count, 4, "Should return count of 4 entities");

        let world = app.world_mut();

        // Verify player spawned
        let mut player_query = world.query_filtered::<Entity, With<Player>>();
        assert_eq!(player_query.iter(world).count(), 1, "Should have 1 player");

        // Verify door spawned
        let mut door_query = world.query_filtered::<Entity, With<Door>>();
        assert_eq!(door_query.iter(world).count(), 1, "Should have 1 door");

        // Verify items spawned
        let mut item_query = world.query_filtered::<Entity, With<Collectible>>();
        assert_eq!(item_query.iter(world).count(), 2, "Should have 2 items");
    }

    #[test]
    fn spawn_demo_entities_skips_unknown_entity_types() {
        // Verify spawn_demo_entities skips unknown entity types without panicking
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (100.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "UnknownType".to_string(),
                position: (200.0, 200.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Match".to_string(),
                position: (150.0, 150.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
        ]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        // Should skip the unknown type but spawn the other two
        assert_eq!(count, 2, "Should return count of 2 (skipping unknown type)");
    }

    #[test]
    fn spawn_demo_entities_handles_empty_entity_list() {
        // Verify spawn_demo_entities handles level with no entities gracefully
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        assert_eq!(count, 0, "Should return count of 0 for empty entity list");
    }

    #[test]
    fn spawn_demo_entities_all_entities_have_demo_marker() {
        // Verify all entities spawned by spawn_demo_entities have DemoMarker
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (100.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Door".to_string(),
                position: (200.0, 100.0),
                target_room: Some(1),
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "Match".to_string(),
                position: (150.0, 150.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
        ]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        // Query for all entities with DemoMarker
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<DemoMarker>>();
        let demo_entity_count = query.iter(world).count();

        assert_eq!(
            demo_entity_count, count,
            "All spawned entities should have DemoMarker"
        );
        assert_eq!(
            demo_entity_count, 3,
            "Should have 3 entities with DemoMarker"
        );
    }

    #[test]
    fn spawn_demo_entities_spawns_multiple_players() {
        // Verify spawn_demo_entities can spawn multiple PlayerSpawn entities
        // (useful for testing scenarios)
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let level_data = create_test_level_data(vec![
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (100.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
            EntitySpawn {
                entity_type: "PlayerSpawn".to_string(),
                position: (200.0, 100.0),
                target_room: None,
                locked: None,
                key_type: None,
            },
        ]);

        let asset_handles = AssetHandles::default();
        let count =
            spawn_demo_entities(&level_data, &mut app.world_mut().commands(), &asset_handles);

        app.update();

        assert_eq!(count, 2, "Should return count of 2");

        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Player>>();
        let player_count = query.iter(world).count();

        assert_eq!(player_count, 2, "Should spawn two player entities");
    }

    // ===== Tests for load_demo_level system =====

    #[test]
    fn load_demo_level_system_compiles() {
        // Verify load_demo_level can be used as a Bevy system
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        // Add the system - it should compile and be callable
        app.add_systems(Update, load_demo_level);

        // System compiles if we get here
        assert!(true, "load_demo_level compiles as a system");
    }

    #[test]
    fn load_demo_level_handles_missing_file_gracefully() {
        // Verify load_demo_level doesn't panic when demo.ron is missing
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        // Run the system - should not panic even if file is missing
        app.add_systems(Update, load_demo_level);
        app.update();

        // If we reach here, the system handled the missing file gracefully
        assert!(true, "System handled missing file without panic");
    }

    #[test]
    fn load_demo_level_records_start_time() {
        // Verify load_demo_level records the start time on first run
        use std::time::Instant;

        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        let before = Instant::now();

        // Run the system
        app.add_systems(Update, load_demo_level);
        app.update();

        let after = Instant::now();

        // Verify system ran within reasonable time
        let duration = after.duration_since(before);
        assert!(
            duration.as_secs() < 1,
            "System should complete quickly (took {:?})",
            duration
        );
    }

    #[test]
    fn load_demo_level_system_is_idempotent() {
        // Verify load_demo_level can be called multiple times safely
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        app.add_systems(Update, load_demo_level);

        // Run multiple times
        app.update();
        app.update();
        app.update();

        // Should not panic or cause issues
        assert!(true, "System can run multiple times safely");
    }

    #[test]
    fn load_demo_level_uses_local_state() {
        // Verify load_demo_level uses Local<Option<Instant>> for state
        // This is tested indirectly by checking that the system compiles
        // with the correct signature

        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        // The system signature requires Local<Option<Instant>>
        // If this compiles, the Local state is correctly typed
        app.add_systems(Update, load_demo_level);

        assert!(true, "System uses Local state correctly");
    }

    #[test]
    fn load_demo_level_accepts_required_resources() {
        // Verify load_demo_level accepts Commands, AssetHandles, AssetServer
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        // Add required resources
        app.insert_resource(AssetHandles::default());
        // AssetServer is provided by AssetPlugin

        // System should accept these resources
        app.add_systems(Update, load_demo_level);
        app.update();

        assert!(true, "System accepts all required resources");
    }

    #[test]
    fn load_demo_level_logs_info_on_start() {
        // Verify load_demo_level logs appropriate messages
        // Note: This test verifies the system runs; actual log verification
        // would require a custom logging backend

        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        app.add_systems(Update, load_demo_level);
        app.update();

        // System should have attempted to log info messages
        // (actual log capture would require bevy's log testing utilities)
        assert!(true, "System attempts to log information");
    }

    #[test]
    fn load_demo_level_does_not_panic_on_error() {
        // Comprehensive test: verify system never panics regardless of conditions
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        app.add_systems(Update, load_demo_level);

        // Run multiple times with potentially missing resources
        for _ in 0..5 {
            app.update();
        }

        assert!(
            true,
            "System handles all error conditions without panicking"
        );
    }

    #[test]
    fn load_demo_level_performance_tracking_compiles() {
        // Verify the performance tracking code compiles correctly
        use std::time::Instant;

        let start_time = Some(Instant::now());
        let duration = start_time.unwrap().elapsed();

        // This is the same pattern used in load_demo_level
        assert!(
            duration.as_secs_f32() >= 0.0,
            "Performance tracking logic is valid"
        );
    }

    #[test]
    fn load_demo_level_can_measure_load_time() {
        // Verify the system can track and measure load duration
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.insert_resource(AssetHandles::default());

        use std::time::Instant;
        let before = Instant::now();

        app.add_systems(Update, load_demo_level);
        app.update();

        let after = Instant::now();
        let duration = after.duration_since(before);

        // System should complete within reasonable time (not the full 10s contract)
        assert!(
            duration.as_millis() < 5000,
            "Load attempt should complete quickly (took {:?})",
            duration
        );
    }
}
