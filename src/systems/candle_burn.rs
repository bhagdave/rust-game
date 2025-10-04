use bevy::prelude::*;
use crate::components::lighting::*;
use crate::resources::game_state::{GameState, GameMode};

/// System for candle wax depletion and state transitions
///
/// Handles:
/// - Wax depletion over time when candle is lit
/// - Automatic extinguishing when wax reaches 0.0
/// - Visibility radius updates based on candle state
/// - Only runs when game is in Playing mode (not Paused/Menu)
///
/// From quickstart.md Test Scenario 1: Candle and Lighting System
pub fn candle_burn_system(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<
        (&mut CandleWax, &mut CandleState, &mut VisibilityRadius, &BurnRate),
        With<Candle>,
    >,
) {
    // Don't process candle burn if game is not in Playing mode
    if game_state.game_mode != GameMode::Playing {
        return;
    }

    for (mut wax, mut state, mut radius, burn_rate) in &mut query {
        // Only deplete wax when candle is lit
        if *state == CandleState::Lit {
            // Deplete wax based on burn rate and delta time
            wax.0 -= burn_rate.0 * time.delta_secs();
            wax.0 = wax.0.max(0.0); // Clamp to 0, never negative

            // Check for auto-extinguish at 0 wax
            if wax.0 == 0.0 {
                *state = CandleState::Extinguished;
                radius.0 = 1.5; // Minimal visibility when extinguished
                // TODO: Emit CandleExtinguishedEvent (for sound effects, UI notifications)
            }
        }

        // Update visibility radius based on candle state
        // This ensures radius is correct even if state changes externally
        match *state {
            CandleState::Lit => {
                radius.0 = 7.0; // Large visibility when lit
            }
            CandleState::Unlit | CandleState::Extinguished => {
                radius.0 = 1.5; // Minimal visibility when not lit
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn candle_burn_system_compiles() {
        // Verify the system function signature is correct
        fn check_system<Params, S: IntoSystem<(), (), Params>>(s: S) -> S {
            s
        }

        check_system(candle_burn_system);
    }

    #[test]
    fn candle_depletes_wax_when_lit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add required resources
        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Playing,
            deaths: 0,
        });

        // Add the candle burn system
        app.add_systems(Update, candle_burn_system);

        // Spawn a lit candle with 100.0 wax and burn rate 1.0
        let candle_entity = app
            .world_mut()
            .spawn((
                Candle,
                CandleWax(100.0),
                CandleState::Lit,
                BurnRate(1.0),
                VisibilityRadius(7.0),
            ))
            .id();

        // Get initial wax
        let initial_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        assert_eq!(initial_wax, 100.0);

        // Run multiple update cycles to ensure time passes
        // (MinimalPlugins may have very small or zero time delta on first frame)
        for _ in 0..10 {
            app.update();
        }

        // Verify wax decreased
        let final_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        assert!(
            final_wax < initial_wax,
            "Wax should decrease when candle is lit (initial: {}, final: {})",
            initial_wax,
            final_wax
        );
    }

    #[test]
    fn unlit_candle_does_not_deplete() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Playing,
            deaths: 0,
        });

        app.add_systems(Update, candle_burn_system);

        // Spawn an UNLIT candle
        let candle_entity = app
            .world_mut()
            .spawn((
                Candle,
                CandleWax(100.0),
                CandleState::Unlit,
                BurnRate(1.0),
                VisibilityRadius(1.5),
            ))
            .id();

        let initial_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        // Run one update
        app.update();

        // Verify wax did NOT decrease
        let final_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        assert_eq!(
            final_wax, initial_wax,
            "Unlit candle should not burn"
        );
    }

    #[test]
    fn paused_game_stops_candle_burn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // GameState in Paused mode
        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Paused,
            deaths: 0,
        });

        app.add_systems(Update, candle_burn_system);

        let candle_entity = app
            .world_mut()
            .spawn((
                Candle,
                CandleWax(100.0),
                CandleState::Lit,
                BurnRate(1.0),
                VisibilityRadius(7.0),
            ))
            .id();

        let initial_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        // Run update (game is paused)
        app.update();

        // Verify wax did NOT decrease
        let final_wax = {
            let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
            wax.0
        };

        assert_eq!(
            final_wax, initial_wax,
            "Candle should not burn when game is paused"
        );
    }

    #[test]
    fn visibility_radius_updates_based_on_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.insert_resource(GameState {
            current_room: 0,
            player_spawn_point: Vec2::new(100.0, 100.0),
            completion_time: Duration::ZERO,
            collected_secrets: std::collections::HashSet::new(),
            game_mode: GameMode::Playing,
            deaths: 0,
        });

        app.add_systems(Update, candle_burn_system);

        // Spawn lit candle
        let candle_entity = app
            .world_mut()
            .spawn((
                Candle,
                CandleWax(50.0),
                CandleState::Lit,
                BurnRate(1.0),
                VisibilityRadius(1.5), // Start with wrong radius
            ))
            .id();

        // Run system once
        app.update();

        // Verify radius updated to lit value
        let radius = {
            let r = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
            r.0
        };

        assert_eq!(radius, 7.0, "Lit candle should have large radius");

        // Manually change to unlit
        {
            let mut state = app.world_mut().get_mut::<CandleState>(candle_entity).unwrap();
            *state = CandleState::Unlit;
        }

        // Run system again
        app.update();

        // Verify radius updated to unlit value
        let radius = {
            let r = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
            r.0
        };

        assert_eq!(radius, 1.5, "Unlit candle should have small radius");
    }
}
