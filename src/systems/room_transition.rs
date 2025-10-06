use crate::components::player::Player;
use crate::components::room::{Room, RoomId};
use crate::resources::game_state::GameState;
use crate::resources::map_state::MapState;
use bevy::prelude::*;

/// Event emitted when the player transitions from one room to another
///
/// This event triggers room loading/unloading, map state updates, and player repositioning.
///
/// # Fields
/// * `old_room` - The RoomId of the room the player is leaving
/// * `new_room` - The RoomId of the room the player is entering
///
/// # Examples
/// ```ignore
/// fn door_interaction_system(
///     mut events: EventWriter<RoomChangedEvent>,
/// ) {
///     events.write(RoomChangedEvent {
///         old_room: 0,
///         new_room: 1,
///     });
/// }
/// ```
#[derive(Event)]
pub struct RoomChangedEvent {
    /// ID of the room the player is leaving
    pub old_room: RoomId,
    /// ID of the room the player is entering
    pub new_room: RoomId,
}

/// System that handles room transitions
///
/// This system manages the complete room transition flow:
///
/// # Behavior
/// For each `RoomChangedEvent`:
/// 1. **Despawn old room**: Removes all entities associated with the old room
/// 2. **Load new room**: Spawns entities for the new room (TODO: load from assets)
/// 3. **Update game state**: Sets current_room to new_room
/// 4. **Update map state**: Marks new room as explored
/// 5. **Move player**: Repositions player to spawn point in new room
/// 6. **Trigger auto-save**: (TODO: emit AutoSaveEvent)
///
/// # System Dependencies
/// - **Upstream**: Door interaction system or other trigger emits `RoomChangedEvent`
/// - **Related**: Uses `GameState.player_spawn_point` for player position
/// - **Related**: Updates `MapState` to track explored rooms
///
/// # Room Loading
/// Currently uses a simplified approach without tilemap loading.
/// Future enhancement: Load room layout from assets/levels/ using bevy_ecs_tilemap
///
/// # Performance
/// - O(n) where n = number of entities in old room
/// - Expected n: 50-100 entities per room (tiles, items, traps, etc.)
/// - Frame impact: <1ms per room transition (one-time cost)
///
/// From tasks.md T030: RoomTransitionSystem
pub fn room_transition_system(
    mut events: EventReader<RoomChangedEvent>,
    mut game_state: ResMut<GameState>,
    mut map_state: ResMut<MapState>,
    mut commands: Commands,
    room_query: Query<(Entity, &Room)>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for event in events.read() {
        // Despawn old room entities
        for (entity, room) in &room_query {
            if room.id == event.old_room {
                commands.entity(entity).despawn();
            }
        }

        // TODO: Load new room from assets/levels/
        // For now, we just update state without spawning new room entities
        // Future: Load tilemap and spawn room entities from level data

        // Update game state
        game_state.current_room = event.new_room;

        // Mark room as explored in map
        map_state.mark_explored(event.new_room);

        // Move player to spawn point in new room
        for mut transform in &mut player_query {
            transform.translation = game_state.player_spawn_point.extend(0.0);
        }

        // TODO: Emit AutoSaveEvent for save system (T031)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_transition_system_compiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        // System compiles and can be added to app
        assert!(true);
    }

    #[test]
    fn room_changed_event_updates_game_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        // Insert GameState and MapState
        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: crate::resources::game_state::GameMode::Playing,
            deaths: 0,
        });

        app.insert_resource(MapState::default());

        // Verify initial state
        {
            let game_state = app.world().resource::<GameState>();
            assert_eq!(game_state.current_room, 0);
        }

        // Send room changed event
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 0,
            new_room: 1,
        });

        // Run system
        app.update();

        // Verify game state updated
        {
            let game_state = app.world().resource::<GameState>();
            assert_eq!(game_state.current_room, 1, "Current room should be updated");
        }
    }

    #[test]
    fn room_transition_marks_room_as_explored() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: crate::resources::game_state::GameMode::Playing,
            deaths: 0,
        });

        app.insert_resource(MapState::default());

        // Verify room not explored initially
        {
            let map_state = app.world().resource::<MapState>();
            assert!(
                !map_state.is_visited(5),
                "Room 5 should not be explored initially"
            );
        }

        // Send room changed event
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 0,
            new_room: 5,
        });

        app.update();

        // Verify room marked as explored
        {
            let map_state = app.world().resource::<MapState>();
            assert!(
                map_state.is_visited(5),
                "Room 5 should be marked as explored"
            );
        }
    }

    #[test]
    fn room_transition_moves_player_to_spawn_point() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        let spawn_point = Vec2::new(200.0, 150.0);
        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: spawn_point,
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: crate::resources::game_state::GameMode::Playing,
            deaths: 0,
        });

        app.insert_resource(MapState::default());

        // Spawn player at different location
        let player = app
            .world_mut()
            .spawn((Player, Transform::from_xyz(500.0, 500.0, 0.0)))
            .id();

        // Verify player at initial position
        {
            let transform = app.world().get::<Transform>(player).unwrap();
            assert_eq!(transform.translation.truncate(), Vec2::new(500.0, 500.0));
        }

        // Send room changed event
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 0,
            new_room: 1,
        });

        app.update();

        // Verify player moved to spawn point
        {
            let transform = app.world().get::<Transform>(player).unwrap();
            assert_eq!(
                transform.translation.truncate(),
                spawn_point,
                "Player should be moved to spawn point"
            );
        }
    }

    #[test]
    fn room_transition_despawns_old_room_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: crate::resources::game_state::GameMode::Playing,
            deaths: 0,
        });

        app.insert_resource(MapState::default());

        // Spawn old room entity
        let old_room_entity = app
            .world_mut()
            .spawn(Room {
                id: 0,
                floor: crate::components::room::Floor::Ground,
                name: "Old Room".to_string(),
            })
            .id();

        // Spawn new room entity (should not be despawned)
        let new_room_entity = app
            .world_mut()
            .spawn(Room {
                id: 1,
                floor: crate::components::room::Floor::First,
                name: "New Room".to_string(),
            })
            .id();

        // Verify both rooms exist
        assert!(app.world().get::<Room>(old_room_entity).is_some());
        assert!(app.world().get::<Room>(new_room_entity).is_some());

        // Send room changed event
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 0,
            new_room: 1,
        });

        app.update();

        // Verify old room despawned, new room still exists
        assert!(
            app.world().get::<Room>(old_room_entity).is_none(),
            "Old room entity should be despawned"
        );
        assert!(
            app.world().get::<Room>(new_room_entity).is_some(),
            "New room entity should still exist"
        );
    }

    #[test]
    fn multiple_room_transitions_work_correctly() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<RoomChangedEvent>();
        app.add_systems(Update, room_transition_system);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: crate::resources::game_state::GameMode::Playing,
            deaths: 0,
        });

        app.insert_resource(MapState::default());

        let player = app.world_mut().spawn((Player, Transform::default())).id();

        // Transition 0 -> 1
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 0,
            new_room: 1,
        });
        app.update();

        {
            let game_state = app.world().resource::<GameState>();
            assert_eq!(game_state.current_room, 1);
            let map_state = app.world().resource::<MapState>();
            assert!(map_state.is_visited(1));
        }

        // Transition 1 -> 2
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 1,
            new_room: 2,
        });
        app.update();

        {
            let game_state = app.world().resource::<GameState>();
            assert_eq!(game_state.current_room, 2);
            let map_state = app.world().resource::<MapState>();
            assert!(map_state.is_visited(2));
        }

        // Transition 2 -> 0 (back to start)
        app.world_mut().send_event(RoomChangedEvent {
            old_room: 2,
            new_room: 0,
        });
        app.update();

        {
            let game_state = app.world().resource::<GameState>();
            assert_eq!(game_state.current_room, 0);
            let map_state = app.world().resource::<MapState>();
            assert!(map_state.is_visited(0));
        }

        // Verify player still exists
        assert!(app.world().get::<Player>(player).is_some());
    }
}
