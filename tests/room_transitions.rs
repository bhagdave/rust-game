use bevy::prelude::*;
use rust_game::components::player::Player;
use rust_game::components::room::{Door, DoorState, Floor, Interactable, Room, TargetRoom};
use rust_game::resources::game_state::GameState;
use rust_game::resources::map_state::MapState;
use rust_game::systems::room_transition::{RoomChangedEvent, room_transition_system};
use std::time::Duration;

/// Integration test: Room transitions using RoomTransitionSystem
/// Tests that room loading/unloading works correctly when the player
/// transitions between rooms.
#[test]
fn room_transition_loads_new_room() {
    // Setup: Create app with minimal plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add room transition system and events
    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    // Insert game resources
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
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

    // Verify initial game state
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 0);
    }

    // Act: Trigger room transition using RoomChangedEvent
    app.world_mut().send_event(RoomChangedEvent {
        old_room: 0,
        new_room: 1,
    });

    // Update game state spawn point for new room
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.player_spawn_point = Vec2::new(300.0, 100.0);
    }

    // Run system
    app.update();

    // Assert: Room A entities despawned by system
    assert!(
        app.world().get::<Room>(room_a).is_none(),
        "Old room should be despawned"
    );

    // Assert: Player moved to new spawn point by system
    let player_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(player_transform.translation.x, 300.0);
    assert_eq!(player_transform.translation.y, 100.0);

    // Assert: Game state updated by system
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.current_room, 1);

    // Assert: Map state shows room 1 as explored
    let map_state = app.world().resource::<MapState>();
    assert!(map_state.is_visited(1));
}

/// Test that multiple room transitions work correctly with RoomTransitionSystem
#[test]
fn multiple_room_transitions() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(0.0, 0.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

    // Spawn player
    let player_entity = app
        .world_mut()
        .spawn((Player, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();

    // Spawn room entities for rooms 0, 1, 2
    let room_0 = app
        .world_mut()
        .spawn(Room {
            id: 0,
            floor: Floor::Ground,
            name: "Room 0".to_string(),
        })
        .id();

    let room_1 = app
        .world_mut()
        .spawn(Room {
            id: 1,
            floor: Floor::First,
            name: "Room 1".to_string(),
        })
        .id();

    let room_2 = app
        .world_mut()
        .spawn(Room {
            id: 2,
            floor: Floor::Second,
            name: "Room 2".to_string(),
        })
        .id();

    // Transition 0 -> 1
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.player_spawn_point = Vec2::new(100.0, 0.0);
    }
    app.world_mut().send_event(RoomChangedEvent {
        old_room: 0,
        new_room: 1,
    });
    app.update();

    // Verify room 0 despawned, room 1 still exists
    assert!(app.world().get::<Room>(room_0).is_none());
    assert!(app.world().get::<Room>(room_1).is_some());
    assert!(app.world().get::<Room>(room_2).is_some());

    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 1);
        let map_state = app.world().resource::<MapState>();
        assert!(map_state.is_visited(1));
    }

    // Transition 1 -> 2
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.player_spawn_point = Vec2::new(200.0, 0.0);
    }
    app.world_mut().send_event(RoomChangedEvent {
        old_room: 1,
        new_room: 2,
    });
    app.update();

    // Verify room 1 despawned, room 2 still exists
    assert!(app.world().get::<Room>(room_1).is_none());
    assert!(app.world().get::<Room>(room_2).is_some());

    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 2);
    }

    let player_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(player_transform.translation.x, 200.0);

    // Assert: Rooms 1 and 2 marked as explored
    let map_state = app.world().resource::<MapState>();
    assert!(map_state.is_visited(1));
    assert!(map_state.is_visited(2));
}

/// Test that player position is preserved during room transition
#[test]
fn player_position_updates_on_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(50.0, 50.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

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
        game_state.player_spawn_point = Vec2::new(500.0, 600.0);
    }

    app.world_mut().send_event(RoomChangedEvent {
        old_room: 0,
        new_room: 1,
    });

    app.update();

    // Verify player moved to new spawn point by system
    let new_transform = app.world().get::<Transform>(player_entity).unwrap();
    assert_eq!(new_transform.translation.x, 500.0);
    assert_eq!(new_transform.translation.y, 600.0);
}

/// Test that doors can be locked and prevent transitions
/// Note: This tests door state, actual prevention logic would be in door interaction system
#[test]
fn locked_door_state_verification() {
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

    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(0.0, 0.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

    // Spawn room 0
    let room_0 = app
        .world_mut()
        .spawn(Room {
            id: 0,
            floor: Floor::Ground,
            name: "Start Room".to_string(),
        })
        .id();

    // Query for all rooms
    let room_count = {
        let world = app.world_mut();
        world.query::<&Room>().iter(world).count()
    };
    assert_eq!(room_count, 1);

    // Spawn room 1 (new room)
    let room_1 = app
        .world_mut()
        .spawn(Room {
            id: 1,
            floor: Floor::First,
            name: "Second Room".to_string(),
        })
        .id();

    // Trigger transition (this will despawn room 0 via system)
    app.world_mut().send_event(RoomChangedEvent {
        old_room: 0,
        new_room: 1,
    });

    app.update();

    // Room 0 should be despawned by system
    assert!(app.world().get::<Room>(room_0).is_none());

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

/// Test that room transition preserves player entity
#[test]
fn room_transition_preserves_player() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

    let player = app.world_mut().spawn((Player, Transform::default())).id();

    // Spawn some room entities
    app.world_mut().spawn(Room {
        id: 0,
        floor: Floor::Ground,
        name: "Room 0".to_string(),
    });

    // Trigger multiple transitions
    for i in 1..=3 {
        app.world_mut().send_event(RoomChangedEvent {
            old_room: i - 1,
            new_room: i,
        });
        app.update();

        // Verify player still exists after each transition
        assert!(
            app.world().get::<Player>(player).is_some(),
            "Player should still exist after transition to room {}",
            i
        );
    }
}

/// Test that map state accumulates explored rooms
#[test]
fn map_state_accumulates_explored_rooms() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<RoomChangedEvent>();
    app.add_systems(Update, room_transition_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(0.0, 0.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: rust_game::resources::game_state::GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

    // Transition through rooms 0 -> 1 -> 2 -> 3
    for i in 0..3 {
        app.world_mut().send_event(RoomChangedEvent {
            old_room: i,
            new_room: i + 1,
        });
        app.update();
    }

    // Verify all rooms marked as explored
    let map_state = app.world().resource::<MapState>();
    assert!(map_state.is_visited(1));
    assert!(map_state.is_visited(2));
    assert!(map_state.is_visited(3));
}
