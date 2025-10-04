use bevy::prelude::*;

#[derive(Component)]
pub enum Trap {
    Spikes,
    FallingChandelier,
    CollapsingFloor,
    Pendulum,
    ArrowTrap,
}

#[derive(Component)]
pub enum TrapTrigger {
    PressurePlate,
    Proximity(f32), // radius
    Timed(f32),     // duration
}

#[derive(Component, Debug, PartialEq)]
pub enum TrapState {
    Armed,
    Triggered,
    Resetting,
}

#[derive(Component)]
pub struct InstantDeath; // marker for instant kill

#[derive(Component)]
pub enum EnvironmentalHazard {
    DraftyWindow,
    WaterPuddle,
    BrokenFloor,
    FanBlade,
    SteamVent,
}

#[derive(Component)]
pub enum HazardEffect {
    ExtinguishCandle,
    SlowMovement,
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
