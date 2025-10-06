use bevy::prelude::*;

/// Marker component for the player character.
///
/// There should only be one player entity in the game world at any time.
/// This component is used to identify and query the player entity.
#[derive(Component)]
pub struct Player;

/// Component storing entity velocity in pixels per second.
///
/// Used for both horizontal and vertical movement. The velocity is applied
/// to the entity's transform each frame by the movement system.
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Component tracking the player's current jump state.
///
/// The jump state machine transitions:
/// - `Grounded` -> `Jumping` (when jump pressed while on ground)
/// - `Jumping` -> `Falling` (when upward velocity stops)
/// - `Falling` -> `DoubleJumping` (when jump pressed with DoubleJumpUnlocked)
/// - `Falling` -> `Grounded` (when landing on ground)
/// - `DoubleJumping` -> `Falling` (when upward velocity stops)
#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum JumpState {
    /// Player is on the ground and can jump
    Grounded,
    /// Player is performing a single jump
    Jumping,
    /// Player is falling (can double jump if unlocked)
    Falling,
    /// Player is performing a double jump
    DoubleJumping,
}

/// Marker component indicating the player has unlocked double jump ability.
///
/// When present, the player can perform a second jump while in the `Falling` state.
/// This is typically acquired by collecting a special item.
#[derive(Component)]
pub struct DoubleJumpUnlocked;

/// Component tracking the player's health status.
///
/// Used to determine if the player is alive or dead. When set to `Dead`,
/// the respawn system will trigger.
#[derive(Component, Debug, PartialEq)]
pub enum Health {
    /// Player is alive and can be controlled
    Alive,
    /// Player is dead and awaiting respawn
    Dead,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_player_components_to_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn entity with player components
        let entity = app
            .world_mut()
            .spawn((
                Player,
                Velocity(Vec2::new(100.0, 0.0)),
                JumpState::Grounded,
                Health::Alive,
            ))
            .id();

        // Verify components exist
        let player = app.world().get::<Player>(entity);
        assert!(player.is_some());

        let velocity = app.world().get::<Velocity>(entity);
        assert!(velocity.is_some());
        assert_eq!(velocity.unwrap().0, Vec2::new(100.0, 0.0));

        let jump_state = app.world().get::<JumpState>(entity);
        assert!(jump_state.is_some());
        assert_eq!(*jump_state.unwrap(), JumpState::Grounded);

        let health = app.world().get::<Health>(entity);
        assert!(health.is_some());
        assert_eq!(*health.unwrap(), Health::Alive);
    }

    #[test]
    fn jump_state_transitions() {
        assert_eq!(JumpState::Grounded, JumpState::Grounded);
        assert_ne!(JumpState::Grounded, JumpState::Jumping);
        assert_ne!(JumpState::Jumping, JumpState::DoubleJumping);
    }

    #[test]
    fn health_states() {
        assert_eq!(Health::Alive, Health::Alive);
        assert_ne!(Health::Alive, Health::Dead);
    }
}
