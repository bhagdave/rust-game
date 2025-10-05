use crate::components::player::{Health, Player};
use crate::components::trap::TrapState;
use bevy::prelude::*;

// Events
#[derive(Event)]
pub struct TrapTriggeredEvent {
    pub trap: Entity,
    pub player: Entity,
}

#[derive(Event)]
pub struct PlayerDeathEvent {
    pub player: Entity,
}

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
