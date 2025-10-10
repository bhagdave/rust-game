use bevy::prelude::*;
use rust_game::resources::asset_handles::AssetHandles;
use rust_game::resources::game_state::GameState;
use rust_game::resources::input_config::InputConfigPlugin;
use rust_game::systems::demo_level::DemoPlugin;
use rust_game::systems::player_movement::player_movement_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "House Escape".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AssetHandles>()
        .init_resource::<GameState>()
        .add_plugins(InputConfigPlugin) // Add input handling
        .add_plugins(DemoPlugin)
        .add_systems(Update, player_movement_system) // Add player movement
        .run();
}
