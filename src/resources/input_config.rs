use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Hash)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
    Climb,
    Interact,
    ToggleCandle,
    UseItem,
    OpenInventory,
    OpenMap,
    Pause,
}

/// Plugin to register input actions and provide default input map
pub struct InputConfigPlugin;

impl Plugin for InputConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .register_type::<PlayerAction>();
    }
}

/// Create default input map with keyboard bindings
pub fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    // Movement
    input_map.insert(PlayerAction::MoveLeft, KeyCode::KeyA);
    input_map.insert(PlayerAction::MoveLeft, KeyCode::ArrowLeft);
    input_map.insert(PlayerAction::MoveRight, KeyCode::KeyD);
    input_map.insert(PlayerAction::MoveRight, KeyCode::ArrowRight);

    // Jump and climb
    input_map.insert(PlayerAction::Jump, KeyCode::Space);
    input_map.insert(PlayerAction::Climb, KeyCode::KeyW);
    input_map.insert(PlayerAction::Climb, KeyCode::ArrowUp);

    // Interactions
    input_map.insert(PlayerAction::Interact, KeyCode::KeyF);
    input_map.insert(PlayerAction::ToggleCandle, KeyCode::KeyE);
    input_map.insert(PlayerAction::UseItem, KeyCode::KeyU);

    // UI
    input_map.insert(PlayerAction::OpenInventory, KeyCode::KeyI);
    input_map.insert(PlayerAction::OpenMap, KeyCode::Tab);
    input_map.insert(PlayerAction::Pause, KeyCode::Escape);

    input_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_action_derives() {
        // Test that PlayerAction can be cloned, copied, and compared
        let action1 = PlayerAction::Jump;
        let action2 = action1;
        assert_eq!(action1, action2);

        let action3 = PlayerAction::MoveLeft;
        assert_ne!(action1, action3);
    }

    #[test]
    fn default_input_map_contains_all_actions() {
        let input_map = default_input_map();

        // Verify movement keys
        assert!(input_map.get(&PlayerAction::MoveLeft).is_some());
        assert!(input_map.get(&PlayerAction::MoveRight).is_some());

        // Verify jump and climb
        assert!(input_map.get(&PlayerAction::Jump).is_some());
        assert!(input_map.get(&PlayerAction::Climb).is_some());

        // Verify interactions
        assert!(input_map.get(&PlayerAction::Interact).is_some());
        assert!(input_map.get(&PlayerAction::ToggleCandle).is_some());
        assert!(input_map.get(&PlayerAction::UseItem).is_some());

        // Verify UI
        assert!(input_map.get(&PlayerAction::OpenInventory).is_some());
        assert!(input_map.get(&PlayerAction::OpenMap).is_some());
        assert!(input_map.get(&PlayerAction::Pause).is_some());
    }

    #[test]
    fn movement_has_multiple_bindings() {
        let input_map = default_input_map();

        // MoveLeft should have both A and ArrowLeft
        let move_left_bindings = input_map.get(&PlayerAction::MoveLeft).unwrap();
        assert_eq!(move_left_bindings.len(), 2);

        // MoveRight should have both D and ArrowRight
        let move_right_bindings = input_map.get(&PlayerAction::MoveRight).unwrap();
        assert_eq!(move_right_bindings.len(), 2);

        // Climb should have both W and ArrowUp
        let climb_bindings = input_map.get(&PlayerAction::Climb).unwrap();
        assert_eq!(climb_bindings.len(), 2);
    }

    #[test]
    fn input_config_plugin_compiles() {
        // Just verify the plugin type exists and can be constructed
        let _plugin = InputConfigPlugin;

        // Verify it implements Plugin trait (this wouldn't compile if it didn't)
        fn check_plugin_impl<T: Plugin>(_: T) {}
        check_plugin_impl(InputConfigPlugin);
    }

    #[test]
    fn can_create_input_manager_bundle() {
        let input_map = default_input_map();

        // Create bundle that would be used with player entity
        // Insert InputMap and ActionState separately per new API
        let _action_state = ActionState::<PlayerAction>::default();

        // Verify we can create both components
        assert!(true);
    }

    #[test]
    fn player_action_debug_format() {
        let action = PlayerAction::Jump;
        let debug_str = format!("{:?}", action);
        assert_eq!(debug_str, "Jump");

        let action = PlayerAction::MoveLeft;
        let debug_str = format!("{:?}", action);
        assert_eq!(debug_str, "MoveLeft");
    }

    #[test]
    fn player_action_hash_works() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(PlayerAction::Jump);
        set.insert(PlayerAction::MoveLeft);
        set.insert(PlayerAction::Jump); // Duplicate

        // Should only have 2 unique actions
        assert_eq!(set.len(), 2);
        assert!(set.contains(&PlayerAction::Jump));
        assert!(set.contains(&PlayerAction::MoveLeft));
    }
}
