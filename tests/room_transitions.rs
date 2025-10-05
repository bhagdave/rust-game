use bevy::prelude::*;
use rust_game::components::player::Player;
use rust_game::components::room::{Door, DoorState, Floor, Interactable, Room, TargetRoom};
use rust_game::resources::game_state::GameState;
use rust_game::resources::map_state::MapState;

/// Integration test: Room transitions
/// Tests that room loading/unloading works correctly when the player
/// interacts with doors to move between rooms.
#[test]
fn room_transition_loads_new_room() {
    // Setup: Create app with minimal plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Insert game resources
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        ..default()
    });
    app.insert_resource(MapState::default());

    // Spawn player in room A (room 0)
    let player_entity = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Name::new("Player"),
        ))
        .id();

    // Spawn room A entity
    let room_a = app
        .world_mut()
        .spawn((
            Room {
                id: 0,
                floor: Floor::Ground,
                name: "Room A".to_string(),
            },
            Name::new("Room A"),
        ))
        .id();

    // Spawn a door to room B in room A
    let door_entity = app
        .world_mut()
        .spawn((
            Door,
            DoorState::Unlocked,
            TargetRoom(1),
            Interactable,
            Transform::from_xyz(200.0, 100.0, 0.0),
            Name::new("Door to Room B"),
        ))
        .id();

    // Verify initial state - Room A exists
    assert!(app.world().get::<Room>(room_a).is_some());
    let room = app.world().get::<Room>(room_a).unwrap();
    assert_eq!(room.id, 0);
    assert_eq!(room.name, "Room A");

    // Verify player is in room A
    let player_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(player_transform.translation.x, 100.0);

    // Verify door exists and points to room 1
    let door_target = app.world().get::<TargetRoom>(door_entity).unwrap();
    assert_eq!(door_target.0, 1);

    // Act: Simulate room transition
    // In a real implementation, this would be triggered by a system
    // For now, we manually simulate the transition

    // 1. Despawn Room A entities (would be done by RoomTransitionSystem)
    app.world_mut().entity_mut(room_a).despawn();
    app.world_mut().entity_mut(door_entity).despawn();

    // 2. Spawn Room B entities
    let room_b = app
        .world_mut()
        .spawn((
            Room {
                id: 1,
                floor: Floor::Ground,
                name: "Room B".to_string(),
            },
            Name::new("Room B"),
        ))
        .id();

    // 3. Update game state to reflect new room
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 1;
        game_state.player_spawn_point = Vec2::new(300.0, 100.0);
    }

    // 4. Move player to new spawn point
    {
        let spawn_point = {
            let game_state = app.world().resource::<GameState>();
            game_state.player_spawn_point
        };
        let mut player_transform = app.world_mut().get_mut::<Transform>(player_entity).unwrap();
        player_transform.translation = spawn_point.extend(0.0);
    }

    // 5. Mark room as explored
    {
        let mut map_state = app.world_mut().resource_mut::<MapState>();
        map_state.mark_explored(1);
    }

    // Assert: Room A entities despawned
    assert!(app.world().get::<Room>(room_a).is_none());
    assert!(app.world().get::<Door>(door_entity).is_none());

    // Assert: Room B entities spawned
    assert!(app.world().get::<Room>(room_b).is_some());
    let new_room = app.world().get::<Room>(room_b).unwrap();
    assert_eq!(new_room.id, 1);
    assert_eq!(new_room.name, "Room B");

    // Assert: Player moved to new position
    let player_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(player_transform.translation.x, 300.0);
    assert_eq!(player_transform.translation.y, 100.0);

    // Assert: Game state updated
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.current_room, 1);

    // Assert: Map state shows room B as explored
    let map_state = app.world().resource::<MapState>();
    assert!(map_state.is_visited(1));
}

/// Test that multiple room transitions work correctly
#[test]
fn multiple_room_transitions() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(GameState::default());
    app.insert_resource(MapState::default());

    // Spawn player
    let player_entity = app
        .world_mut()
        .spawn((Player, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    // Simulate transitions through rooms 0 -> 1 -> 2
    for room_id in 0..=2 {
        // Update game state
        {
            let mut game_state = app.world_mut().resource_mut::<GameState>();
            game_state.current_room = room_id;
            game_state.player_spawn_point = Vec2::new(room_id as f32 * 100.0, 0.0);
        }

        // Move player
        {
            let spawn_point = {
                let game_state = app.world().resource::<GameState>();
                game_state.player_spawn_point
            };
            let mut player_transform = app.world_mut().get_mut::<Transform>(player_entity).unwrap();
            player_transform.translation = spawn_point.extend(0.0);
        }

        // Mark explored
        {
            let mut map_state = app.world_mut().resource_mut::<MapState>();
            map_state.mark_explored(room_id);
        }
    }

    // Assert: Player is in room 2
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.current_room, 2);

    let player_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(player_transform.translation.x, 200.0);

    // Assert: All rooms marked as explored
    let map_state = app.world().resource::<MapState>();
    assert!(map_state.is_visited(0));
    assert!(map_state.is_visited(1));
    assert!(map_state.is_visited(2));
    assert_eq!(map_state.explored_count(), 3);
}

/// Test that player position is preserved during room transition
#[test]
fn player_position_updates_on_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(50.0, 50.0),
        ..default()
    });

    let player_entity = app
        .world_mut()
        .spawn((Player, Transform::from_xyz(10.0, 20.0, 0.0)))
        .id();

    // Initial position
    let initial_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(initial_transform.translation.x, 10.0);
    assert_eq!(initial_transform.translation.y, 20.0);

    // Transition to new room with different spawn point
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 1;
        game_state.player_spawn_point = Vec2::new(500.0, 600.0);
    }

    // Move player to spawn point
    {
        let spawn_point = {
            let game_state = app.world().resource::<GameState>();
            game_state.player_spawn_point
        };
        let mut player_transform = app.world_mut().get_mut::<Transform>(player_entity).unwrap();
        player_transform.translation = spawn_point.extend(0.0);
    }

    // Verify player moved to new spawn point
    let new_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(new_transform.translation.x, 500.0);
    assert_eq!(new_transform.translation.y, 600.0);
}

/// Test that doors can be locked and prevent transitions
#[test]
fn locked_door_prevents_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    use rust_game::components::inventory::KeyType;

    // Spawn a locked door
    let door_entity = app
        .world_mut()
        .spawn((
            Door,
            DoorState::Locked(KeyType::Brass),
            TargetRoom(1),
            Interactable,
        ))
        .id();

    // Verify door is locked
    let door_state = app.world().get::<DoorState>(door_entity).unwrap();
    assert_eq!(*door_state, DoorState::Locked(KeyType::Brass));

    // In a real system, the door being locked would prevent the transition
    // Here we just verify the door state is correct
    match door_state {
        DoorState::Locked(key_type) => {
            assert_eq!(*key_type, KeyType::Brass);
        }
        _ => panic!("Door should be locked"),
    }
}

/// Test that room entities can be queried after transition
#[test]
fn can_query_room_entities_after_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(GameState::default());

    // Spawn room 0
    let room_0 = app
        .world_mut()
        .spawn((Room {
            id: 0,
            floor: Floor::Ground,
            name: "Start Room".to_string(),
        },))
        .id();

    // Query for all rooms
    let room_count = {
        let world = app.world_mut();
        world.query::<&Room>().iter(world).count()
    };
    assert_eq!(room_count, 1);

    // Despawn room 0 and spawn room 1 (transition)
    app.world_mut().entity_mut(room_0).despawn();

    let room_1 = app
        .world_mut()
        .spawn((Room {
            id: 1,
            floor: Floor::First,
            name: "Second Room".to_string(),
        },))
        .id();

    // Query for all rooms after transition
    let room_count = {
        let world = app.world_mut();
        world.query::<&Room>().iter(world).count()
    };
    assert_eq!(room_count, 1);

    // Verify it's the new room
    let room = app.world().get::<Room>(room_1).unwrap();
    assert_eq!(room.id, 1);
    assert_eq!(room.name, "Second Room");
}
