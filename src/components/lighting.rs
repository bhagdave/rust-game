use bevy::prelude::*;

/// Marker component for candle entities.
///
/// Used to identify candle entities in queries. Candles provide light
/// and consume wax over time when lit.
#[derive(Component)]
pub struct Candle;

/// Component storing the remaining wax in a candle.
///
/// Value ranges from 0.0 (empty) to 100.0 (full). When wax reaches 0.0,
/// the candle automatically extinguishes.
#[derive(Component)]
pub struct CandleWax(pub f32);

/// Component tracking the current state of a candle.
///
/// State transitions:
/// - `Unlit` -> `Lit` (when player uses a match)
/// - `Lit` -> `Extinguished` (when wax reaches 0.0 or environmental effect)
/// - `Extinguished` -> Cannot be relit
#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum CandleState {
    /// Candle has not been lit yet (can be lit with a match)
    Unlit,
    /// Candle is currently burning and providing light
    Lit,
    /// Candle wax is depleted or was put out (cannot be relit)
    Extinguished,
}

/// Component defining how far the player can see around a light source.
///
/// Measured in tiles. Typical values:
/// - 7.0 tiles for a lit candle
/// - 1.5 tiles for minimal ambient visibility
#[derive(Component)]
pub struct VisibilityRadius(pub f32);

/// Component defining how quickly candle wax depletes.
///
/// Measured in wax percentage per second. A burn rate of 1.0 means
/// the candle will fully deplete in 100 seconds.
#[derive(Component)]
pub struct BurnRate(pub f32);

/// Component defining visual properties of a light source.
///
/// Used for rendering light effects. Typical candle values:
/// - color: Warm yellow-orange (1.0, 0.9, 0.6)
/// - intensity: 1.0 (full brightness)
#[derive(Component)]
pub struct LightSource {
    /// Color of the emitted light
    pub color: Color,
    /// Brightness multiplier (0.0 to 1.0+)
    pub intensity: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_candle_components() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn candle entity with all components
        let entity = app
            .world_mut()
            .spawn((
                Candle,
                CandleWax(100.0),
                CandleState::Lit,
                VisibilityRadius(7.0),
                BurnRate(1.0),
                LightSource {
                    color: Color::srgb(1.0, 0.9, 0.6),
                    intensity: 1.0,
                },
            ))
            .id();

        // Verify components can be queried
        let candle = app.world().get::<Candle>(entity);
        assert!(candle.is_some());

        let wax = app.world().get::<CandleWax>(entity);
        assert!(wax.is_some());
        assert_eq!(wax.unwrap().0, 100.0);

        let state = app.world().get::<CandleState>(entity);
        assert!(state.is_some());
        assert_eq!(*state.unwrap(), CandleState::Lit);

        let radius = app.world().get::<VisibilityRadius>(entity);
        assert!(radius.is_some());
        assert_eq!(radius.unwrap().0, 7.0);

        let burn_rate = app.world().get::<BurnRate>(entity);
        assert!(burn_rate.is_some());
        assert_eq!(burn_rate.unwrap().0, 1.0);

        let light_source = app.world().get::<LightSource>(entity);
        assert!(light_source.is_some());
        assert_eq!(light_source.unwrap().intensity, 1.0);
    }

    #[test]
    fn candle_state_transitions() {
        assert_eq!(CandleState::Unlit, CandleState::Unlit);
        assert_ne!(CandleState::Unlit, CandleState::Lit);
        assert_ne!(CandleState::Lit, CandleState::Extinguished);
    }

    #[test]
    fn candle_wax_bounds() {
        let full_wax = CandleWax(100.0);
        let empty_wax = CandleWax(0.0);
        let partial_wax = CandleWax(50.5);

        assert_eq!(full_wax.0, 100.0);
        assert_eq!(empty_wax.0, 0.0);
        assert_eq!(partial_wax.0, 50.5);
    }
}
