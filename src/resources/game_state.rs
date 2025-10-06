use crate::components::room::RoomId;
use bevy::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

/// Global game state resource tracking player progress and game status.
///
/// This resource maintains the current state of the game including
/// which room the player is in, completion metrics, and the current
/// game mode (menu, playing, paused, etc.).
#[derive(Resource)]
pub struct GameState {
    /// ID of the room the player is currently in
    pub current_room: RoomId,
    /// World position where player should spawn in current room
    pub player_spawn_point: Vec2,
    /// Total time elapsed since game start
    pub completion_time: Duration,
    /// Set of secret entities the player has discovered
    pub collected_secrets: HashSet<Entity>,
    /// Current game mode determining system behavior
    pub game_mode: GameMode,
    /// Number of times the player has died
    pub deaths: u32,
}

/// Enum representing the current mode/state of the game.
///
/// Different game modes affect which systems run and how input is handled.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameMode {
    /// Main menu screen
    Menu,
    /// Active gameplay
    Playing,
    /// Game paused by player
    Paused,
    /// Player died or failed
    GameOver,
    /// Player successfully escaped the house
    Victory,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: Duration::ZERO,
            collected_secrets: HashSet::new(),
            game_mode: GameMode::Menu,
            deaths: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert_game_state_as_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Insert GameState resource
        app.insert_resource(GameState::default());

        // Verify resource exists
        let game_state = app.world().get_resource::<GameState>();
        assert!(game_state.is_some());

        let game_state = game_state.unwrap();
        assert_eq!(game_state.current_room, 0);
        assert_eq!(game_state.player_spawn_point, Vec2::new(100.0, 100.0));
        assert_eq!(game_state.completion_time, Duration::ZERO);
        assert_eq!(game_state.collected_secrets.len(), 0);
        assert_eq!(game_state.game_mode, GameMode::Menu);
        assert_eq!(game_state.deaths, 0);
    }

    #[test]
    fn can_access_game_state_in_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GameState::default());

        // Add a system that accesses GameState
        fn test_system(game_state: Res<GameState>) {
            assert_eq!(game_state.game_mode, GameMode::Menu);
        }

        app.add_systems(Update, test_system);
        app.update();
    }

    #[test]
    fn can_modify_game_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GameState::default());

        // Modify game state
        {
            let mut game_state = app.world_mut().resource_mut::<GameState>();
            game_state.current_room = 5;
            game_state.game_mode = GameMode::Playing;
            game_state.deaths = 3;
            game_state.player_spawn_point = Vec2::new(500.0, 300.0);
        }

        // Verify modifications
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 5);
        assert_eq!(game_state.game_mode, GameMode::Playing);
        assert_eq!(game_state.deaths, 3);
        assert_eq!(game_state.player_spawn_point, Vec2::new(500.0, 300.0));
    }

    #[test]
    fn game_mode_transitions() {
        assert_eq!(GameMode::Menu, GameMode::Menu);
        assert_ne!(GameMode::Menu, GameMode::Playing);
        assert_ne!(GameMode::Playing, GameMode::Paused);
        assert_ne!(GameMode::Paused, GameMode::GameOver);
        assert_ne!(GameMode::GameOver, GameMode::Victory);
    }

    #[test]
    fn can_track_collected_secrets() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let mut game_state = GameState::default();

        // Spawn some entities and add to collected secrets
        let entity1 = app.world_mut().spawn_empty().id();
        let entity2 = app.world_mut().spawn_empty().id();

        game_state.collected_secrets.insert(entity1);
        game_state.collected_secrets.insert(entity2);

        assert_eq!(game_state.collected_secrets.len(), 2);
        assert!(game_state.collected_secrets.contains(&entity1));
        assert!(game_state.collected_secrets.contains(&entity2));
    }

    #[test]
    fn completion_time_tracking() {
        let mut game_state = GameState::default();

        // Simulate time passage
        game_state.completion_time = Duration::from_secs(120);

        assert_eq!(game_state.completion_time.as_secs(), 120);
    }

    #[test]
    fn death_counter() {
        let mut game_state = GameState::default();

        assert_eq!(game_state.deaths, 0);

        game_state.deaths += 1;
        assert_eq!(game_state.deaths, 1);

        game_state.deaths += 5;
        assert_eq!(game_state.deaths, 6);
    }
}
