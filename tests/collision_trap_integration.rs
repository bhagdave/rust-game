use bevy::prelude::*;
use rust_game::components::player::{Health, Player};
use rust_game::components::room::Collider;
use rust_game::components::trap::{Trap, TrapState, TrapTrigger};
use rust_game::systems::collision::collision_detection_system;
use rust_game::systems::trap::{PlayerDeathEvent, TrapTriggeredEvent, trap_activation_system};

#[test]
fn collision_detection_triggers_trap_activation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Register events
    app.add_event::<TrapTriggeredEvent>();
    app.add_event::<PlayerDeathEvent>();

    // Add both systems - collision detection followed by trap activation
    app.add_systems(
        Update,
        (collision_detection_system, trap_activation_system).chain(),
    );

    // Spawn player at origin
    let player = app
        .world_mut()
        .spawn((
            Player,
            Health::Alive,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
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
            Transform::from_xyz(10.0, 10.0, 0.0),
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

    // Run one update cycle - collision detection → trap activation
    app.update();

    // Verify player is dead
    assert_eq!(
        *app.world().get::<Health>(player).unwrap(),
        Health::Dead,
        "Player should be dead after trap collision"
    );

    // Verify trap is triggered
    assert_eq!(
        *app.world().get::<TrapState>(trap).unwrap(),
        TrapState::Triggered,
        "Trap should be triggered after collision"
    );

    // Verify PlayerDeathEvent was emitted
    let trap_events = app.world().resource::<Events<PlayerDeathEvent>>();
    let mut reader = trap_events.get_cursor();
    let death_events: Vec<_> = reader.read(trap_events).collect();

    assert_eq!(
        death_events.len(),
        1,
        "Should emit exactly one PlayerDeathEvent"
    );
    assert_eq!(
        death_events[0].player, player,
        "Death event should reference correct player"
    );
}

#[test]
fn no_collision_no_trap_activation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Register events
    app.add_event::<TrapTriggeredEvent>();
    app.add_event::<PlayerDeathEvent>();

    // Add both systems
    app.add_systems(
        Update,
        (collision_detection_system, trap_activation_system).chain(),
    );

    // Spawn player at origin
    let player = app
        .world_mut()
        .spawn((
            Player,
            Health::Alive,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Spawn trap far away from player
    let trap = app
        .world_mut()
        .spawn((
            Trap::Spikes,
            TrapState::Armed,
            TrapTrigger::PressurePlate,
            Transform::from_xyz(1000.0, 1000.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Run one update cycle
    app.update();

    // Verify player is still alive
    assert_eq!(
        *app.world().get::<Health>(player).unwrap(),
        Health::Alive,
        "Player should remain alive when no collision occurs"
    );

    // Verify trap is still armed
    assert_eq!(
        *app.world().get::<TrapState>(trap).unwrap(),
        TrapState::Armed,
        "Trap should remain armed when no collision occurs"
    );

    // Verify no PlayerDeathEvent was emitted
    let trap_events = app.world().resource::<Events<PlayerDeathEvent>>();
    let mut reader = trap_events.get_cursor();
    let death_events: Vec<_> = reader.read(trap_events).collect();

    assert_eq!(
        death_events.len(),
        0,
        "Should not emit PlayerDeathEvent when no collision"
    );
}

#[test]
fn multiple_trap_collisions_all_trigger() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Register events
    app.add_event::<TrapTriggeredEvent>();
    app.add_event::<PlayerDeathEvent>();

    // Add both systems
    app.add_systems(
        Update,
        (collision_detection_system, trap_activation_system).chain(),
    );

    // Spawn player at origin
    let player = app
        .world_mut()
        .spawn((
            Player,
            Health::Alive,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Spawn three traps all overlapping with player
    let trap1 = app
        .world_mut()
        .spawn((
            Trap::Spikes,
            TrapState::Armed,
            TrapTrigger::PressurePlate,
            Transform::from_xyz(5.0, 5.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    let trap2 = app
        .world_mut()
        .spawn((
            Trap::ArrowTrap,
            TrapState::Armed,
            TrapTrigger::Proximity(5.0),
            Transform::from_xyz(-5.0, -5.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    let trap3 = app
        .world_mut()
        .spawn((
            Trap::CollapsingFloor,
            TrapState::Armed,
            TrapTrigger::Timed(2.0),
            Transform::from_xyz(0.0, 8.0, 0.0),
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Run one update cycle
    app.update();

    // Verify player is dead (from any trap)
    assert_eq!(
        *app.world().get::<Health>(player).unwrap(),
        Health::Dead,
        "Player should be dead after hitting multiple traps"
    );

    // Verify all traps are triggered
    assert_eq!(
        *app.world().get::<TrapState>(trap1).unwrap(),
        TrapState::Triggered,
        "Trap 1 should be triggered"
    );
    assert_eq!(
        *app.world().get::<TrapState>(trap2).unwrap(),
        TrapState::Triggered,
        "Trap 2 should be triggered"
    );
    assert_eq!(
        *app.world().get::<TrapState>(trap3).unwrap(),
        TrapState::Triggered,
        "Trap 3 should be triggered"
    );
}
