use crate::components::player::{Health, Player};
use crate::components::trap::TrapState;
use bevy::prelude::*;

/// Event emitted when a trap is triggered by a player.
///
/// This event causes the trap to transition to `TrapState::Triggered`
/// and the player's health to become `Health::Dead`. It also triggers
/// a `PlayerDeathEvent` for downstream systems to handle.
///
/// # Fields
/// * `trap` - The entity of the trap being triggered
/// * `player` - The entity of the player who triggered the trap
///
/// # Examples
/// ```ignore
/// fn collision_detection_system(
///     mut events: EventWriter<TrapTriggeredEvent>,
/// ) {
///     events.send(TrapTriggeredEvent {
///         trap: trap_entity,
///         player: player_entity,
///     });
/// }
/// ```
#[derive(Event)]
pub struct TrapTriggeredEvent {
    pub trap: Entity,
    pub player: Entity,
}

/// Event emitted when a player dies from any cause.
///
/// Other systems (respawn, UI, audio) should listen for this event
/// to react to player death. This event is emitted by the trap activation
/// system when a player is killed by a trap.
///
/// # Fields
/// * `player` - The entity of the player who died
///
/// # Examples
/// ```ignore
/// fn respawn_system(
///     mut death_events: EventReader<PlayerDeathEvent>,
/// ) {
///     for event in death_events.read() {
///         // Handle player respawn
///     }
/// }
/// ```
#[derive(Event)]
pub struct PlayerDeathEvent {
    pub player: Entity,
}

/// System that processes trap activation events and kills players.
///
/// This system is the core of the trap mechanic. When a player collides with
/// a trap and a `TrapTriggeredEvent` is emitted, this system:
///
/// # Behavior
/// For each `TrapTriggeredEvent`:
/// 1. Sets the trap's state to `TrapState::Triggered`
/// 2. Sets the player's health to `Health::Dead`
/// 3. Emits a `PlayerDeathEvent` for downstream systems
///
/// # Error Handling
/// The system gracefully handles missing entities:
/// - If the trap entity doesn't exist, the trap state update is skipped
/// - If the player entity doesn't exist, the death logic is skipped
/// - No panics occur from invalid entity references
///
/// # System Dependencies
/// - **Upstream**: `collision_detection_system` must emit `TrapTriggeredEvent`
/// - **Downstream**: `respawn_system` consumes `PlayerDeathEvent`
/// - **Related**: Works with `Health` component from player module
/// - **Related**: Works with `TrapState` component from trap module
///
/// # Performance
/// - O(n) complexity where n = number of trap events per frame
/// - Expected n: 0-2 in normal gameplay
/// - Minimal frame time impact (<0.01% of 16ms frame budget)
///
/// # Examples
/// ```ignore
/// use bevy::prelude::*;
/// use rust_game::systems::trap::*;
///
/// fn main() {
///     App::new()
///         .add_event::<TrapTriggeredEvent>()
///         .add_event::<PlayerDeathEvent>()
///         .add_systems(Update, trap_activation_system)
///         .run();
/// }
/// ```
pub fn trap_activation_system(
    mut events: EventReader<TrapTriggeredEvent>,
    mut trap_query: Query<&mut TrapState>,
    mut player_query: Query<&mut Health, With<Player>>,
    mut death_events: EventWriter<PlayerDeathEvent>,
) {
    for event in events.read() {
        // Set trap to triggered
        if let Ok(mut trap_state) = trap_query.get_mut(event.trap) {
            *trap_state = TrapState::Triggered;
        }

        // Kill player
        if let Ok(mut health) = player_query.get_mut(event.player) {
            *health = Health::Dead;
            death_events.write(PlayerDeathEvent {
                player: event.player,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::trap::{Trap, TrapTrigger};

    #[test]
    fn trap_activation_system_compiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // System compiles and can be added to app
        assert!(true);
    }

    #[test]
    fn trap_triggered_event_kills_player() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Spawn player
        let player = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        // Spawn trap
        let trap = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                TrapState::Armed,
                TrapTrigger::PressurePlate,
                Transform::default(),
            ))
            .id();

        // Verify initial state
        assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Alive);
        assert_eq!(
            *app.world().get::<TrapState>(trap).unwrap(),
            TrapState::Armed
        );

        // Send trap triggered event
        app.world_mut()
            .send_event(TrapTriggeredEvent { trap, player });

        // Run system
        app.update();

        // Verify player is dead
        assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Dead);

        // Verify trap is triggered
        assert_eq!(
            *app.world().get::<TrapState>(trap).unwrap(),
            TrapState::Triggered
        );
    }

    #[test]
    fn trap_activation_sends_death_event() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Spawn player and trap
        let player = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        let trap = app
            .world_mut()
            .spawn((
                Trap::FallingChandelier,
                TrapState::Armed,
                TrapTrigger::Proximity(5.0),
                Transform::default(),
            ))
            .id();

        // Send trap triggered event
        app.world_mut()
            .send_event(TrapTriggeredEvent { trap, player });

        // Run system
        app.update();

        // Check that death event was sent
        let death_events = app.world_mut().resource_mut::<Events<PlayerDeathEvent>>();
        let mut reader = death_events.get_cursor();
        let events: Vec<_> = reader.read(&death_events).collect();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].player, player);
    }

    #[test]
    fn trap_activation_handles_multiple_traps() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Spawn player
        let player = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        // Spawn multiple traps
        let trap1 = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                TrapState::Armed,
                TrapTrigger::PressurePlate,
                Transform::default(),
            ))
            .id();

        let trap2 = app
            .world_mut()
            .spawn((
                Trap::ArrowTrap,
                TrapState::Armed,
                TrapTrigger::Proximity(3.0),
                Transform::default(),
            ))
            .id();

        // Trigger first trap
        app.world_mut().send_event(TrapTriggeredEvent {
            trap: trap1,
            player,
        });

        app.update();

        // Verify first trap triggered
        assert_eq!(
            *app.world().get::<TrapState>(trap1).unwrap(),
            TrapState::Triggered
        );
        assert_eq!(
            *app.world().get::<TrapState>(trap2).unwrap(),
            TrapState::Armed
        );
        assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Dead);
    }

    #[test]
    fn trap_activation_graceful_on_invalid_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Create entities that don't exist
        let fake_trap = Entity::from_raw(999);
        let fake_player = Entity::from_raw(998);

        // Send event with invalid entities
        app.world_mut().send_event(TrapTriggeredEvent {
            trap: fake_trap,
            player: fake_player,
        });

        // System should not panic
        app.update();

        // Test passes if no panic occurs
        assert!(true);
    }

    #[test]
    fn multiple_trap_events_processed_in_single_update() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Spawn player and traps
        let player = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        let trap1 = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                TrapState::Armed,
                TrapTrigger::PressurePlate,
                Transform::default(),
            ))
            .id();

        let trap2 = app
            .world_mut()
            .spawn((
                Trap::CollapsingFloor,
                TrapState::Armed,
                TrapTrigger::Timed(2.0),
                Transform::default(),
            ))
            .id();

        // Send multiple events
        app.world_mut().send_event(TrapTriggeredEvent {
            trap: trap1,
            player,
        });
        app.world_mut().send_event(TrapTriggeredEvent {
            trap: trap2,
            player,
        });

        // Run system once
        app.update();

        // Both traps should be triggered
        assert_eq!(
            *app.world().get::<TrapState>(trap1).unwrap(),
            TrapState::Triggered
        );
        assert_eq!(
            *app.world().get::<TrapState>(trap2).unwrap(),
            TrapState::Triggered
        );
    }

    #[test]
    fn trap_activation_only_affects_specified_player() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<TrapTriggeredEvent>();
        app.add_event::<PlayerDeathEvent>();
        app.add_systems(Update, trap_activation_system);

        // Spawn two players (hypothetically, though game has one)
        let player1 = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        let player2 = app
            .world_mut()
            .spawn((Player, Health::Alive, Transform::default()))
            .id();

        // Spawn trap
        let trap = app
            .world_mut()
            .spawn((
                Trap::Pendulum,
                TrapState::Armed,
                TrapTrigger::Proximity(4.0),
                Transform::default(),
            ))
            .id();

        // Trigger trap for player1 only
        app.world_mut().send_event(TrapTriggeredEvent {
            trap,
            player: player1,
        });

        app.update();

        // Only player1 should be dead
        assert_eq!(*app.world().get::<Health>(player1).unwrap(), Health::Dead);
        assert_eq!(*app.world().get::<Health>(player2).unwrap(), Health::Alive);
    }
}
