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
use crate::components::inventory::KeyType;
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

// Future functions will be implemented here in subsequent tasks:
// - spawn_item(): Spawns items (matches, keys) for demo level
// - load_demo_level(): Main system to load demo from RON file
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
        let _level_data_type = std::any::type_name::<LevelData>();
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
}
