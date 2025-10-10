use bevy::prelude::*;
use rust_game::resources::asset_handles::AssetHandles;
use rust_game::resources::game_state::GameState;
use rust_game::resources::input_config::PlayerAction;
use rust_game::systems::demo_level::DemoPlugin;
use rust_game::systems::player_movement::player_movement_system;
use leafwing_input_manager::plugin::InputManagerPlugin;
use bevy_ecs_tilemap::TilemapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "House Escape".to_string(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AssetHandles>()
        .init_resource::<GameState>()
        .add_plugins(TilemapPlugin) // Add tilemap plugin for rendering tilemaps
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(DemoPlugin)
        .add_systems(Update, player_movement_system) // Add player movement
        .run();
}
