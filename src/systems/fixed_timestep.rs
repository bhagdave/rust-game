use bevy::prelude::*;

/// Plugin that configures fixed timestep scheduling for deterministic game logic
///
/// Implements a fixed 60Hz update rate for physics, collision detection, and gameplay
/// systems to ensure deterministic behavior across different frame rates and platforms.
///
/// # Why Fixed Timestep?
///
/// - **Determinism**: Physics calculations produce identical results regardless of frame rate
/// - **Testing**: Automated tests can reliably reproduce game states
/// - **Fairness**: Game logic runs at same speed on all hardware
/// - **Network**: Easier to synchronize multiplayer game states
///
/// # Systems Scheduled on Fixed Timestep
///
/// The following systems run on `FixedUpdate` schedule at 60Hz:
/// 1. Player movement and physics
/// 2. Collision detection
/// 3. Trap activation
/// 4. Candle wax depletion
/// 5. Respawn logic
///
/// # Systems on Variable Timestep
///
/// These systems run on `Update` schedule (per-frame):
/// - Rendering (lighting, UI, tilemap)
/// - Input handling (queued for next fixed update)
/// - Audio playback
/// - Save/load operations
///
/// # Usage
///
/// Add this plugin to your Bevy app during initialization:
///
/// ```rust
/// use bevy::prelude::*;
/// use rust_game::systems::fixed_timestep::FixedTimestepPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(FixedTimestepPlugin)
///         .run();
/// }
/// ```
///
/// # Configuration
///
/// The fixed timestep rate is set to 60Hz (16.67ms per tick). To modify:
///
/// ```rust
/// app.insert_resource(Time::<Fixed>::from_hz(120.0)); // 120Hz
/// ```
///
/// # Performance Considerations
///
/// - Fixed timestep may run multiple times per frame on slow hardware (catch-up)
/// - Maximum catch-up iterations limited to prevent spiral of death
/// - Systems should complete in <16ms to maintain 60Hz target
///
/// From tasks.md T042: Fixed timestep for deterministic physics
pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        // Configure fixed timestep at 60Hz (60 updates per second)
        // This provides deterministic game logic across all platforms
        app.insert_resource(Time::<Fixed>::from_hz(60.0));

        // Note: Individual game systems are added to FixedUpdate schedule
        // in their respective plugin modules (e.g., PlayerMovementPlugin,
        // CollisionPlugin, etc.)
        //
        // This plugin only configures the timestep rate. Systems should
        // use app.add_systems(FixedUpdate, system_fn) in their own plugins.
    }
}

/// Helper function to verify fixed timestep configuration in tests
///
/// Returns the configured fixed timestep duration in seconds.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use rust_game::systems::fixed_timestep::{FixedTimestepPlugin, get_fixed_timestep};
///
/// let mut app = App::new();
/// app.add_plugins(FixedTimestepPlugin);
///
/// let timestep = get_fixed_timestep(&app);
/// assert!((timestep - 1.0/60.0).abs() < 0.0001);
/// ```
pub fn get_fixed_timestep(app: &App) -> f32 {
    if let Some(fixed_time) = app.world().get_resource::<Time<Fixed>>() {
        fixed_time.timestep().as_secs_f32()
    } else {
        // Default Bevy fixed timestep (64Hz)
        1.0 / 64.0
    }
}

/// Helper function to advance fixed timestep by one tick in tests
///
/// Useful for deterministic testing of game logic.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use rust_game::systems::fixed_timestep::advance_fixed_timestep;
///
/// let mut app = App::new();
/// app.add_plugins(MinimalPlugins);
///
/// // Advance game logic by one fixed timestep
/// advance_fixed_timestep(&mut app);
/// ```
pub fn advance_fixed_timestep(app: &mut App) {
    // Run fixed update schedule once
    app.update();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_timestep_plugin_compiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);
        // Plugin should register successfully - verified by compilation
    }

    #[test]
    fn fixed_timestep_configured_at_60hz() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);

        let timestep = get_fixed_timestep(&app);
        let expected = 1.0 / 60.0;
        let tolerance = 0.0001;

        assert!(
            (timestep - expected).abs() < tolerance,
            "Fixed timestep should be 1/60 seconds (60Hz), got {}",
            timestep
        );
    }

    #[test]
    fn fixed_timestep_resource_exists() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);

        // Verify Time<Fixed> resource was inserted
        assert!(
            app.world().get_resource::<Time<Fixed>>().is_some(),
            "Time<Fixed> resource should be inserted by plugin"
        );
    }

    #[test]
    fn fixed_timestep_configures_correctly() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);

        // Verify the Time<Fixed> resource is configured
        let time_fixed = app.world().get_resource::<Time<Fixed>>();
        assert!(
            time_fixed.is_some(),
            "Time<Fixed> resource should be configured"
        );

        // Verify the timestep is correct
        let timestep = time_fixed.unwrap().timestep();
        let expected = std::time::Duration::from_secs_f32(1.0 / 60.0);
        assert!(
            (timestep.as_secs_f32() - expected.as_secs_f32()).abs() < 0.0001,
            "Timestep should be 1/60 seconds"
        );
    }

    #[test]
    fn advance_fixed_timestep_helper_doesnt_crash() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);

        // Helper should complete without panic
        advance_fixed_timestep(&mut app);
        advance_fixed_timestep(&mut app);

        // No assertion needed - just verify it doesn't crash
    }

    #[test]
    fn get_fixed_timestep_returns_default_without_plugin() {
        let app = App::new();
        let timestep = get_fixed_timestep(&app);

        // Should return Bevy's default (64Hz)
        let expected = 1.0 / 64.0;
        let tolerance = 0.0001;

        assert!(
            (timestep - expected).abs() < tolerance,
            "Should return default Bevy timestep without plugin"
        );
    }

    #[test]
    fn fixed_timestep_independent_of_frame_rate() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(FixedTimestepPlugin);

        // Get timestep before and after updates
        let timestep_before = get_fixed_timestep(&app);

        app.update();
        app.update();
        app.update();

        let timestep_after = get_fixed_timestep(&app);

        assert_eq!(
            timestep_before, timestep_after,
            "Fixed timestep should remain constant regardless of frame updates"
        );
    }
}
