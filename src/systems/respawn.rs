use crate::components::player::{Health, Player};
use crate::resources::game_state::GameState;
use crate::systems::trap::PlayerDeathEvent;
use bevy::prelude::*;

const RESPAWN_DELAY: f32 = 1.0; // seconds

#[derive(Component)]
pub struct DeathTimer(pub Timer);

pub fn respawn_system(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut death_events: EventReader<PlayerDeathEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Health, Option<&mut DeathTimer>), With<Player>>,
) {
    // Add death timer on death event
    for event in death_events.read() {
        if let Ok((entity, _, _, _)) = query.get_mut(event.player) {
            commands
                .entity(entity)
                .insert(DeathTimer(Timer::from_seconds(
                    RESPAWN_DELAY,
                    TimerMode::Once,
                )));
        }
    }

    // Tick timers and respawn when complete
    for (entity, mut transform, mut health, timer) in &mut query {
        if let Some(mut timer) = timer {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                // Respawn
                transform.translation = game_state.player_spawn_point.extend(0.0);
                *health = Health::Alive;
                commands.entity(entity).remove::<DeathTimer>();
            }
        }
    }
}
