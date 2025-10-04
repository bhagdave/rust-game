use bevy::prelude::*;
use rust_game::components::lighting::*;
use rust_game::resources::game_state::*;
use rust_game::systems::candle_burn::candle_burn_system;
use std::time::Duration;

#[test]
fn candle_burns_when_lit() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Insert GameState resource in Playing mode
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Candle with 100.0 wax, CandleState::Lit, BurnRate 1.0
    let candle_entity = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(100.0),
            CandleState::Lit,
            BurnRate(1.0), // 1.0 wax per second
            VisibilityRadius(7.0),
        ))
        .id();

    // Assert: Initial wax is 100.0
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 100.0, "Candle should start with 100.0 wax");
    }

    // Assert: Candle is lit
    {
        let state = app.world().get::<CandleState>(candle_entity).unwrap();
        assert_eq!(*state, CandleState::Lit, "Candle should be lit");
    }

    // Act: Add CandleBurnSystem and advance time
    app.add_systems(Update, candle_burn_system);

    // Run updates to simulate ~10 seconds of game time
    // With default MinimalPlugins time, each update advances a small amount
    // Running many updates ensures we burn enough wax
    for _ in 0..600 {
        app.update();
    }

    // Assert: Wax should have decreased (exact amount depends on time deltas)
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert!(wax.0 < 100.0, "Wax should decrease over time (wax: {})", wax.0);
    }

    // Assert: Visibility radius remains large while lit (candle hasn't extinguished)
    {
        let radius = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
        assert_eq!(radius.0, 7.0, "Visibility radius should remain large while lit");
    }
}

#[test]
fn candle_extinguishes_at_zero_wax() {
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

    // Setup: Candle with 1.0 wax, lit, BurnRate 1.0
    let candle_entity = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(1.0),
            CandleState::Lit,
            BurnRate(1.0),
            VisibilityRadius(7.0),
        ))
        .id();

    // Assert: Candle starts with 1.0 wax and is lit
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 1.0, "Candle should start with 1.0 wax");

        let state = app.world().get::<CandleState>(candle_entity).unwrap();
        assert_eq!(*state, CandleState::Lit, "Candle should start lit");
    }

    // Act: Add system and burn enough to extinguish
    app.add_systems(Update, candle_burn_system);

    // Run updates to burn wax
    for _ in 0..1000 {
        app.update();
    }

    // Assert: Wax has decreased
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert!(wax.0 < 1.0, "Wax should have decreased (actual: {})", wax.0);
    }

    // Now manually set wax to 0 to test extinguish behavior
    {
        let mut wax = app.world_mut().get_mut::<CandleWax>(candle_entity).unwrap();
        wax.0 = 0.0;
    }

    // Run one more update to trigger extinguish logic
    app.update();

    // Assert: Candle state changes to Extinguished when wax hits 0
    {
        let state = app.world().get::<CandleState>(candle_entity).unwrap();
        assert_eq!(*state, CandleState::Extinguished,
                   "Candle should extinguish at 0 wax");
    }

    // Assert: Wax is 0.0
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 0.0, "Wax should remain at 0.0");
    }

    // Assert: Visibility radius shrinks to 1.5
    {
        let radius = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
        assert_eq!(radius.0, 1.5, "Visibility radius should shrink when extinguished");
    }
}

#[test]
fn unlit_candle_does_not_burn() {
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

    // Setup: Candle with 100.0 wax, Unlit
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

    // Act: Add system
    app.add_systems(Update, candle_burn_system);

    // Run many updates (simulating time passing)
    for _ in 0..600 {
        app.update();
    }

    // Assert: Wax should still be 100.0 (no burning when unlit)
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 100.0, "Unlit candle should not burn");
    }
}

#[test]
fn paused_game_stops_candle_burn() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Setup: GameState in Paused mode
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Paused, // Game is paused
        deaths: 0,
    });

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

    // Act: Add system
    app.add_systems(Update, candle_burn_system);

    // Run many updates (but game is paused)
    for _ in 0..600 {
        app.update();
    }

    // Assert: Wax should still be 100.0 (no burning when paused)
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 100.0, "Candle should not burn when game is paused");
    }
}

#[test]
fn visibility_radius_updates_based_on_candle_state() {
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

    let candle_entity = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(50.0),
            CandleState::Lit,
            BurnRate(1.0),
            VisibilityRadius(7.0),
        ))
        .id();

    // Act: Add system
    app.add_systems(Update, candle_burn_system);

    // Run once to update radius
    app.update();

    // Assert: Lit candle has large visibility radius (7.0)
    {
        let radius = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
        assert_eq!(radius.0, 7.0, "Lit candle should have large visibility radius");
    }

    // Act: Manually change candle to Unlit
    {
        let mut state = app.world_mut().get_mut::<CandleState>(candle_entity).unwrap();
        *state = CandleState::Unlit;
    }

    // Run system to update visibility radius
    app.update();

    // Assert: Unlit candle has small visibility radius (1.5)
    {
        let radius = app.world().get::<VisibilityRadius>(candle_entity).unwrap();
        assert_eq!(radius.0, 1.5, "Unlit candle should have small visibility radius");
    }
}

#[test]
fn burn_rate_affects_depletion_speed() {
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

    // Setup: Two candles with different burn rates
    let fast_candle = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(100.0),
            CandleState::Lit,
            BurnRate(2.0), // Burns twice as fast
            VisibilityRadius(7.0),
        ))
        .id();

    let slow_candle = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(100.0),
            CandleState::Lit,
            BurnRate(0.5), // Burns half as fast
            VisibilityRadius(7.0),
        ))
        .id();

    // Act: Add system and run updates
    app.add_systems(Update, candle_burn_system);

    for _ in 0..600 {
        app.update();
    }

    // Assert: Fast candle depleted more than slow candle
    {
        let fast_wax = app.world().get::<CandleWax>(fast_candle).unwrap();
        let slow_wax = app.world().get::<CandleWax>(slow_candle).unwrap();

        assert!(fast_wax.0 < slow_wax.0,
                "Fast burning candle should deplete more (fast: {}, slow: {})",
                fast_wax.0, slow_wax.0);
        assert!(fast_wax.0 < 100.0, "Fast candle should have burned some wax");
        assert!(slow_wax.0 < 100.0, "Slow candle should have burned some wax");
    }
}

#[test]
fn multiple_candles_burn_independently() {
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

    // Setup: Multiple candles with different states
    let lit_candle = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(100.0),
            CandleState::Lit,
            BurnRate(1.0),
            VisibilityRadius(7.0),
        ))
        .id();

    let unlit_candle = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(100.0),
            CandleState::Unlit,
            BurnRate(1.0),
            VisibilityRadius(1.5),
        ))
        .id();

    let low_wax_candle = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(5.0),
            CandleState::Lit,
            BurnRate(1.0),
            VisibilityRadius(7.0),
        ))
        .id();

    // Act: Add system and run updates
    app.add_systems(Update, candle_burn_system);

    // Run updates
    for _ in 0..1000 {
        app.update();
    }

    // Assert: Lit candle depleted
    {
        let wax = app.world().get::<CandleWax>(lit_candle).unwrap();
        assert!(wax.0 < 100.0, "Lit candle should have burned");
    }

    // Assert: Unlit candle stays at 100.0
    {
        let wax = app.world().get::<CandleWax>(unlit_candle).unwrap();
        assert_eq!(wax.0, 100.0, "Unlit candle should not burn");
    }

    // Assert: Low wax candle has burned some (MinimalPlugins time too slow for full burnout in tests)
    {
        let wax = app.world().get::<CandleWax>(low_wax_candle).unwrap();
        assert!(wax.0 < 5.0, "Low wax candle should have burned some");
    }

    // Manually deplete low wax candle to test extinguish behavior
    {
        let mut wax = app.world_mut().get_mut::<CandleWax>(low_wax_candle).unwrap();
        wax.0 = 0.0;
    }

    // Run one update to trigger extinguish
    app.update();

    // Assert: Low wax candle extinguishes at 0.0
    {
        let state = app.world().get::<CandleState>(low_wax_candle).unwrap();
        assert_eq!(*state, CandleState::Extinguished, "Low wax candle should be extinguished");
    }
}
