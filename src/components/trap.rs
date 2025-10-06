use bevy::prelude::*;

/// Component defining the type of trap.
///
/// Each trap type has unique visual representation and activation behavior.
/// Traps are triggered by their associated `TrapTrigger` component.
#[derive(Component)]
pub enum Trap {
    /// Floor spikes that emerge when triggered
    Spikes,
    /// Chandelier that falls from ceiling
    FallingChandelier,
    /// Floor section that collapses beneath the player
    CollapsingFloor,
    /// Swinging blade pendulum
    Pendulum,
    /// Wall-mounted arrow launcher
    ArrowTrap,
}

/// Component defining how a trap is activated.
///
/// Determines the conditions under which a trap transitions from
/// `Armed` to `Triggered` state.
#[derive(Component)]
pub enum TrapTrigger {
    /// Activates when player steps on pressure plate
    PressurePlate,
    /// Activates when player enters radius (in pixels)
    Proximity(f32),
    /// Activates after duration (in seconds)
    Timed(f32),
}

/// Component tracking the current state of a trap.
///
/// State transitions:
/// - `Armed` -> `Triggered` (when trigger condition met)
/// - `Triggered` -> `Resetting` (after trap executes)
/// - `Resetting` -> `Armed` (when reset complete)
#[derive(Component, Debug, PartialEq)]
pub enum TrapState {
    /// Trap is ready to be triggered
    Armed,
    /// Trap has been activated and is executing
    Triggered,
    /// Trap is resetting to armed state
    Resetting,
}

/// Marker component indicating a trap causes instant death on contact.
///
/// When present, collision with this trap immediately sets player health to `Dead`.
#[derive(Component)]
pub struct InstantDeath;

/// Component defining environmental hazards in the game world.
///
/// Unlike traps, hazards are static elements that affect gameplay
/// through their associated `HazardEffect` component.
#[derive(Component)]
pub enum EnvironmentalHazard {
    /// Open window that creates drafts
    DraftyWindow,
    /// Puddle of water on floor
    WaterPuddle,
    /// Damaged floor section
    BrokenFloor,
    /// Spinning fan blade
    FanBlade,
    /// Steam vent that releases periodic bursts
    SteamVent,
}

/// Component defining the gameplay effect of an environmental hazard.
///
/// Applied to the player when they interact with the associated
/// `EnvironmentalHazard` component.
#[derive(Component)]
pub enum HazardEffect {
    /// Extinguishes the player's candle
    ExtinguishCandle,
    /// Reduces player movement speed
    SlowMovement,
    /// Causes damage when falling through
    FallDamage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_trap_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn trap with all components
        let entity = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                TrapTrigger::PressurePlate,
                TrapState::Armed,
                InstantDeath,
            ))
            .id();

        // Verify trap components
        let trap = app.world().get::<Trap>(entity);
        assert!(trap.is_some());

        let trigger = app.world().get::<TrapTrigger>(entity);
        assert!(trigger.is_some());

        let state = app.world().get::<TrapState>(entity);
        assert!(state.is_some());
        assert_eq!(*state.unwrap(), TrapState::Armed);

        let instant_death = app.world().get::<InstantDeath>(entity);
        assert!(instant_death.is_some());
    }

    #[test]
    fn trap_types_definable() {
        // Test all trap variants
        let _spikes = Trap::Spikes;
        let _chandelier = Trap::FallingChandelier;
        let _floor = Trap::CollapsingFloor;
        let _pendulum = Trap::Pendulum;
        let _arrow = Trap::ArrowTrap;
    }

    #[test]
    fn trap_trigger_types() {
        let _pressure = TrapTrigger::PressurePlate;
        let _proximity = TrapTrigger::Proximity(5.0);
        let _timed = TrapTrigger::Timed(3.0);
    }

    #[test]
    fn trap_state_transitions() {
        assert_eq!(TrapState::Armed, TrapState::Armed);
        assert_ne!(TrapState::Armed, TrapState::Triggered);
        assert_ne!(TrapState::Triggered, TrapState::Resetting);
    }

    #[test]
    fn environmental_hazards_definable() {
        // Test all hazard variants
        let _window = EnvironmentalHazard::DraftyWindow;
        let _puddle = EnvironmentalHazard::WaterPuddle;
        let _floor = EnvironmentalHazard::BrokenFloor;
        let _fan = EnvironmentalHazard::FanBlade;
        let _vent = EnvironmentalHazard::SteamVent;
    }

    #[test]
    fn hazard_effects_definable() {
        let _extinguish = HazardEffect::ExtinguishCandle;
        let _slow = HazardEffect::SlowMovement;
        let _fall = HazardEffect::FallDamage;
    }

    #[test]
    fn can_create_hazard_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let entity = app
            .world_mut()
            .spawn((
                EnvironmentalHazard::DraftyWindow,
                HazardEffect::ExtinguishCandle,
            ))
            .id();

        let hazard = app.world().get::<EnvironmentalHazard>(entity);
        assert!(hazard.is_some());

        let effect = app.world().get::<HazardEffect>(entity);
        assert!(effect.is_some());
    }
}
