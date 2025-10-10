use bevy::prelude::*;
use rust_game::resources::asset_handles::AssetHandles;
use rust_game::resources::game_state::GameState;
use rust_game::systems::demo_level::DemoPlugin;

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
        .add_plugins(DemoPlugin)
        .run();
}
