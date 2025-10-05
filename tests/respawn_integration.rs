use bevy::prelude::*;
use rust_game::components::inventory::*;
use rust_game::components::player::{Health, Player};
use rust_game::components::room::Collider;
use rust_game::components::trap::{Trap, TrapState, TrapTrigger};
use rust_game::resources::game_state::{GameMode, GameState};
use rust_game::systems::collision::collision_detection_system;
use rust_game::systems::inventory::ItemCollectedEvent;
use rust_game::systems::respawn::{DeathTimer, RESPAWN_DELAY, respawn_system};
use rust_game::systems::trap::{PlayerDeathEvent, TrapTriggeredEvent, trap_activation_system};
use std::time::Duration;

#[test]
fn complete_death_and_respawn_cycle() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Register events (including ItemCollectedEvent for collision system)
    app.add_event::<TrapTriggeredEvent>();
    app.add_event::<PlayerDeathEvent>();
    app.add_event::<ItemCollectedEvent>();

    // Add all systems in order: collision → trap activation → respawn
    app.add_systems(
        Update,
        (
            collision_detection_system,
            trap_activation_system,
            respawn_system,
        )
            .chain(),
    );

    // Setup GameState
    let spawn_point = Vec2::new(100.0, 100.0);
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: spawn_point,
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Spawn player with inventory at spawn point
    let player = app
        .world_mut()
        .spawn((
            Player,
            Health::Alive,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
            Inventory {
                items: vec![Item::Match, Item::Key(KeyType::Brass)],
                max_capacity: 10,
            },
        ))
        .id();

    // Spawn trap overlapping with player
    let trap = app
        .world_mut()
        .spawn((
            Trap::Spikes,
            TrapState::Armed,
            TrapTrigger::PressurePlate,
            Transform::from_xyz(105.0, 105.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Verify initial state
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Alive);
    assert_eq!(
        *app.world().get::<TrapState>(trap).unwrap(),
        TrapState::Armed
    );
    assert_eq!(app.world().get::<Inventory>(player).unwrap().items.len(), 2);

    // Update 1: Collision detected, trap triggers, player dies
    app.update();

    // Verify player is dead and trap is triggered
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Dead);
    assert_eq!(
        *app.world().get::<TrapState>(trap).unwrap(),
        TrapState::Triggered
    );

    // Verify death timer was added
    assert!(app.world().get::<DeathTimer>(player).is_some());

    // Verify inventory is preserved
    assert_eq!(
        app.world().get::<Inventory>(player).unwrap().items.len(),
        2,
        "Inventory should be preserved during death"
    );

    // Manually tick the death timer to completion
    {
        let mut query = app.world_mut().query::<&mut DeathTimer>();
        if let Ok(mut timer) = query.get_mut(app.world_mut(), player) {
            timer.0.tick(Duration::from_secs_f32(RESPAWN_DELAY + 0.1));
        }
    }

    // Update 2: Timer expired, player respawns
    app.update();

    // Verify player respawned
    assert_eq!(
        *app.world().get::<Health>(player).unwrap(),
        Health::Alive,
        "Player should be alive after respawn"
    );

    // Verify position reset to spawn point
    let transform = app.world().get::<Transform>(player).unwrap();
    assert_eq!(
        transform.translation.truncate(),
        spawn_point,
        "Player should respawn at spawn point"
    );

    // Verify death timer removed
    assert!(
        app.world().get::<DeathTimer>(player).is_none(),
        "DeathTimer should be removed after respawn"
    );

    // Verify inventory still preserved
    assert_eq!(
        app.world().get::<Inventory>(player).unwrap().items.len(),
        2,
        "Inventory should still be preserved after respawn"
    );
}

#[test]
fn respawn_with_death_event_only() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PlayerDeathEvent>();
    app.add_systems(Update, respawn_system);

    let spawn_point = Vec2::new(50.0, 50.0);
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: spawn_point,
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Spawn player at different location
    let player = app
        .world_mut()
        .spawn((Player, Health::Dead, Transform::from_xyz(300.0, 300.0, 0.0)))
        .id();

    // Verify initial state
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Dead);
    let initial_pos = app.world().get::<Transform>(player).unwrap().translation;
    assert_eq!(initial_pos.truncate(), Vec2::new(300.0, 300.0));

    // Send death event
    app.world_mut().send_event(PlayerDeathEvent { player });

    // Update to add timer
    app.update();

    // Verify timer added
    assert!(app.world().get::<DeathTimer>(player).is_some());

    // Manually expire timer
    {
        let mut query = app.world_mut().query::<&mut DeathTimer>();
        if let Ok(mut timer) = query.get_mut(app.world_mut(), player) {
            timer.0.tick(Duration::from_secs_f32(RESPAWN_DELAY + 0.1));
        }
    }

    // Update to respawn
    app.update();

    // Verify respawn
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Alive);
    assert_eq!(
        app.world()
            .get::<Transform>(player)
            .unwrap()
            .translation
            .truncate(),
        spawn_point
    );
    assert!(app.world().get::<DeathTimer>(player).is_none());
}

#[test]
fn multiple_death_respawn_cycles() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PlayerDeathEvent>();
    app.add_systems(Update, respawn_system);

    let spawn_point = Vec2::new(0.0, 0.0);
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: spawn_point,
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    let player = app
        .world_mut()
        .spawn((Player, Health::Alive, Transform::default()))
        .id();

    // Death-respawn cycle 1
    app.world_mut().send_event(PlayerDeathEvent { player });
    app.update();
    {
        let mut query = app.world_mut().query::<&mut DeathTimer>();
        if let Ok(mut timer) = query.get_mut(app.world_mut(), player) {
            timer.0.tick(Duration::from_secs_f32(RESPAWN_DELAY + 0.1));
        }
    }
    app.update();
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Alive);

    // Death-respawn cycle 2
    app.world_mut().send_event(PlayerDeathEvent { player });
    app.update();
    {
        let mut query = app.world_mut().query::<&mut DeathTimer>();
        if let Ok(mut timer) = query.get_mut(app.world_mut(), player) {
            timer.0.tick(Duration::from_secs_f32(RESPAWN_DELAY + 0.1));
        }
    }
    app.update();
    assert_eq!(*app.world().get::<Health>(player).unwrap(), Health::Alive);

    // Player should still exist and be respawnable
    assert!(app.world().get::<Player>(player).is_some());
}

#[test]
fn respawn_only_affects_players_with_expired_timers() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PlayerDeathEvent>();
    app.add_systems(Update, respawn_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(0.0, 0.0),
        completion_time: Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Player 1: Timer will expire
    let player1 = app
        .world_mut()
        .spawn((
            Player,
            Health::Dead,
            Transform::from_xyz(100.0, 100.0, 0.0),
            DeathTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        ))
        .id();

    // Player 2: Timer won't expire
    let player2 = app
        .world_mut()
        .spawn((
            Player,
            Health::Dead,
            Transform::from_xyz(200.0, 200.0, 0.0),
            DeathTimer(Timer::from_seconds(10.0, TimerMode::Once)),
        ))
        .id();

    // Tick only player1's timer
    {
        let mut query = app.world_mut().query::<&mut DeathTimer>();
        if let Ok(mut timer) = query.get_mut(app.world_mut(), player1) {
            timer.0.tick(Duration::from_secs_f32(1.0));
        }
    }

    // Update to process
    app.update();

    // Player 1 should respawn
    assert_eq!(*app.world().get::<Health>(player1).unwrap(), Health::Alive);
    assert!(app.world().get::<DeathTimer>(player1).is_none());

    // Player 2 should still be dead
    assert_eq!(*app.world().get::<Health>(player2).unwrap(), Health::Dead);
    assert!(app.world().get::<DeathTimer>(player2).is_some());
}
