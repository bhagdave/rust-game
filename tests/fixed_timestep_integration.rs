use bevy::prelude::*;
use rust_game::systems::fixed_timestep::{
    FixedTimestepPlugin, advance_fixed_timestep, get_fixed_timestep,
};

/// Helper to create test app
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// Test that fixed timestep plugin integrates with Bevy app
#[test]
fn fixed_timestep_plugin_registers_successfully() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    // Should not panic - plugin registered successfully
    app.update();
}

/// Test that fixed timestep is configured at 60Hz
#[test]
fn fixed_timestep_is_60hz() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    let timestep = get_fixed_timestep(&app);
    let expected_hz = 60.0;
    let expected_timestep = 1.0 / expected_hz;

    assert!(
        (timestep - expected_timestep).abs() < 0.0001,
        "Fixed timestep should be {} seconds ({}Hz), got {}",
        expected_timestep,
        expected_hz,
        timestep
    );
}

/// Test that fixed timestep plugin configuration is accessible
#[test]
fn fixed_timestep_resource_exists() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    // Verify Time<Fixed> resource was configured
    let timestep = get_fixed_timestep(&app);
    assert!(timestep > 0.0, "Fixed timestep should be configured");
}

/// Test that get_fixed_timestep works without plugin (returns default)
#[test]
fn get_fixed_timestep_returns_default_without_plugin() {
    let app = App::new();
    let timestep = get_fixed_timestep(&app);

    // Bevy's default fixed timestep is 64Hz
    let expected = 1.0 / 64.0;
    assert!(
        (timestep - expected).abs() < 0.0001,
        "Should return Bevy default timestep without plugin"
    );
}

/// Test that fixed timestep configuration persists across updates
#[test]
fn fixed_timestep_configuration_persists() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    let timestep_before = get_fixed_timestep(&app);

    // Run multiple updates
    for _ in 0..10 {
        app.update();
    }

    let timestep_after = get_fixed_timestep(&app);

    assert_eq!(
        timestep_before, timestep_after,
        "Fixed timestep configuration should persist across updates"
    );
}

/// Test that advance_fixed_timestep helper works
#[test]
fn advance_fixed_timestep_helper_works() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    // Helper should complete without panic
    advance_fixed_timestep(&mut app);
    advance_fixed_timestep(&mut app);
    advance_fixed_timestep(&mut app);

    // No assertion needed - just verify it doesn't crash
}

/// Test that FixedTimestepPlugin can be added to a real game app
#[test]
fn plugin_works_with_default_plugins() {
    // This test verifies the plugin works with MinimalPlugins
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, FixedTimestepPlugin));

    app.update();

    let timestep = get_fixed_timestep(&app);
    assert!((timestep - 1.0 / 60.0).abs() < 0.0001);
}

/// Test that the plugin properly configures Time<Fixed>
#[test]
fn plugin_configures_time_fixed() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    // Access Time<Fixed> resource
    let time_fixed = app.world().get_resource::<Time<Fixed>>();
    assert!(time_fixed.is_some(), "Time<Fixed> should be configured");

    let timestep = time_fixed.unwrap().timestep();
    assert!(
        (timestep.as_secs_f32() - 1.0 / 60.0).abs() < 0.0001,
        "Timestep should be 1/60 seconds"
    );
}

/// Test that fixed timestep value is correct
#[test]
fn fixed_timestep_value_is_correct() {
    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);

    let expected = 1.0 / 60.0; // 60Hz = ~0.01667 seconds
    let actual = get_fixed_timestep(&app);

    assert!(
        (actual - expected).abs() < 0.00001,
        "Expected {} seconds, got {}",
        expected,
        actual
    );
}

/// Test that the plugin doesn't interfere with normal Update schedule
#[test]
fn plugin_allows_normal_updates() {
    #[derive(Resource, Default)]
    struct Counter(u32);

    let mut app = create_test_app();
    app.add_plugins(FixedTimestepPlugin);
    app.insert_resource(Counter::default());

    // Add system to Update schedule (not FixedUpdate)
    app.add_systems(Update, |mut counter: ResMut<Counter>| {
        counter.0 += 1;
    });

    // Run some updates
    for _ in 0..5 {
        app.update();
    }

    // Update systems should still work
    let counter = app.world().resource::<Counter>();
    assert_eq!(counter.0, 5, "Update schedule should run every frame");
}
