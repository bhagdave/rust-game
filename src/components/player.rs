use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum JumpState {
    Grounded,
    Jumping,
    Falling,
    DoubleJumping,
}

#[derive(Component)]
pub struct DoubleJumpUnlocked;

#[derive(Component, Debug, PartialEq)]
pub enum Health {
    Alive,
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
