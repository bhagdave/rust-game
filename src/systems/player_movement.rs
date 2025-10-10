use crate::components::player::*;
use crate::resources::game_state::{GameMode, GameState};
use crate::resources::input_config::PlayerAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Query type for player movement system to reduce type complexity
type PlayerMovementQuery<'a> = (
    &'a mut Transform,
    &'a mut Velocity,
    &'a mut JumpState,
    &'a ActionState<PlayerAction>,
    Option<&'a DoubleJumpUnlocked>,
);

/// System for player movement, jump physics, and horizontal velocity
///
/// Handles:
/// - Horizontal movement (A/D or Arrow keys)
/// - Jump mechanics (Space when grounded)
/// - Double jump (if DoubleJumpUnlocked component present)
/// - Gravity application
/// - Position updates based on velocity
///
/// From quickstart.md Test Scenario 2: Player Movement and Jump Mechanics
pub fn player_movement_system(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<PlayerMovementQuery, With<Player>>,
) {
    // Don't process movement if game is not in Playing mode (paused, menu, etc.)
    if game_state.game_mode != GameMode::Playing {
        return;
    }

    for (mut transform, mut velocity, mut jump_state, actions, double_jump_unlocked) in &mut query {
        // Horizontal movement
        let mut move_dir = 0.0;
        if actions.pressed(&PlayerAction::MoveLeft) {
            move_dir -= 1.0;
        }
        if actions.pressed(&PlayerAction::MoveRight) {
            move_dir += 1.0;
        }

        // Set horizontal velocity (200 pixels per second)
        velocity.0.x = move_dir * 200.0;

        // Jump logic
        if actions.just_pressed(&PlayerAction::Jump) {
            match *jump_state {
                JumpState::Grounded => {
                    // Single jump from ground
                    velocity.0.y = 400.0; // upward velocity
                    *jump_state = JumpState::Jumping;
                }
                JumpState::Jumping | JumpState::Falling => {
                    // Double jump if unlocked
                    if double_jump_unlocked.is_some() {
                        velocity.0.y = 400.0;
                        *jump_state = JumpState::DoubleJumping;
                    }
                    // Otherwise, ignore jump input (not grounded, no double jump)
                }
                JumpState::DoubleJumping => {
                    // Already used double jump, ignore further jump inputs
                }
            }
        }

        // Apply gravity to vertical velocity
        if *jump_state != JumpState::Grounded {
            velocity.0.y -= 980.0 * time.delta_secs(); // 980 px/s^2 gravity
        }

        // Update position based on velocity
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // TODO: Collision detection and ground check
        // This will be implemented in collision system (T028)
        // Collision system will:
        // 1. Detect collisions with floors/platforms
        // 2. Set jump_state back to Grounded when player lands
        // 3. Resolve vertical velocity to 0 when grounded
        // 4. Prevent movement through walls (horizontal collision)

        // Simple ground check (placeholder until collision system exists)
        // Tilemap is 15 tiles high (480 pixels), centered at y=0
        // Bottom wall is at y = -240 + 16 = -224 (center of bottom tile)
        // Player should stand on floor tiles just above bottom wall at y = -224 + 32 = -192
        let ground_level = -192.0;
        if transform.translation.y <= ground_level {
            transform.translation.y = ground_level;
            velocity.0.y = 0.0;
            *jump_state = JumpState::Grounded;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_movement_system_compiles() {
        // Verify the system function signature is correct
        fn check_system<Params, S: IntoSystem<(), (), Params>>(s: S) -> S {
            s
        }

        check_system(player_movement_system);
    }

    // Note: Testing leafwing-input-manager action states properly requires
    // more complex setup with plugin systems. These tests verify the system compiles
    // and that game modes are respected. Full input testing is done via integration tests.

    #[test]
    fn paused_game_stops_movement() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // GameState in Paused mode
        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Paused, // Paused!
            deaths: 0,
        });

        app.add_systems(Update, player_movement_system);

        let player_entity = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Velocity(Vec2::ZERO),
                JumpState::Grounded,
                InputMap::<PlayerAction>::default(),
                ActionState::<PlayerAction>::default(),
            ))
            .id();

        // Set action as pressed
        {
            let mut action_state = app
                .world_mut()
                .get_mut::<ActionState<PlayerAction>>(player_entity)
                .unwrap();
            action_state.press(&PlayerAction::MoveRight);
        }

        let initial_x = {
            let transform = app.world().get::<Transform>(player_entity).unwrap();
            transform.translation.x
        };

        // Run one update
        app.update();

        // Verify player did NOT move (game is paused)
        let final_x = {
            let transform = app.world().get::<Transform>(player_entity).unwrap();
            transform.translation.x
        };

        assert_eq!(
            final_x, initial_x,
            "Player should not move when game is paused"
        );
    }

    // Note: Integration tests for movement with input system are complex
    // and require leafwing-input-manager plugin setup. Testing movement logic
    // is done through the paused_game_stops_movement test and other unit tests.

    #[test]
    fn gravity_applies_when_not_grounded() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Playing,
            deaths: 0,
        });

        app.add_systems(Update, player_movement_system);

        // Player starts with upward velocity while jumping
        let player_entity = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, 100.0, 0.0),
                Velocity(Vec2::new(0.0, 100.0)), // upward velocity
                JumpState::Jumping,
                InputMap::<PlayerAction>::default(),
                ActionState::<PlayerAction>::default(),
            ))
            .id();

        let initial_velocity = {
            let vel = app.world().get::<Velocity>(player_entity).unwrap();
            vel.0.y
        };

        // Run several updates to let gravity apply
        app.update();
        app.update();
        app.update();

        // Verify velocity decreased due to gravity
        let final_velocity = {
            let vel = app.world().get::<Velocity>(player_entity).unwrap();
            vel.0.y
        };

        assert!(
            final_velocity < initial_velocity,
            "Gravity should reduce upward velocity"
        );
    }

    #[test]
    fn player_lands_on_ground_at_y_zero() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: std::time::Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Playing,
            deaths: 0,
        });

        app.add_systems(Update, player_movement_system);

        // Player falling below ground level
        let player_entity = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, -10.0, 0.0), // below y=0
                Velocity(Vec2::new(0.0, -50.0)),      // falling
                JumpState::Falling,
                InputMap::<PlayerAction>::default(),
                ActionState::<PlayerAction>::default(),
            ))
            .id();

        // Run one update
        app.update();

        // Verify player snapped to ground and stopped falling
        let transform = app.world().get::<Transform>(player_entity).unwrap();
        assert_eq!(
            transform.translation.y, 0.0,
            "Player should be at ground level"
        );

        let velocity = app.world().get::<Velocity>(player_entity).unwrap();
        assert_eq!(velocity.0.y, 0.0, "Player should have no vertical velocity");

        let jump_state = app.world().get::<JumpState>(player_entity).unwrap();
        assert_eq!(
            *jump_state,
            JumpState::Grounded,
            "Player should be grounded"
        );
    }
}
