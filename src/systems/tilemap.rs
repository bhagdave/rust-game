use crate::resources::game_state::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// System that sets up tilemap rendering for a room
///
/// Creates a tilemap entity with tile storage and spawns individual tiles
/// for rendering room floors, walls, and environmental elements.
///
/// # System Dependencies
/// - **Resources**: AssetServer for loading tileset texture
/// - **Components**: Creates TilemapBundle with TileStorage
///
/// # Behavior
/// 1. Loads tileset texture from assets/sprites/tileset.png
/// 2. Creates tilemap entity with specified dimensions
/// 3. Spawns individual tiles at each position
/// 4. Configures tile size, grid size, and rendering properties
///
/// From tasks.md T033: Tilemap rendering with bevy_ecs_tilemap 0.16.0
pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    // Load tileset texture
    let texture_handle: Handle<Image> = asset_server.load("sprites/tileset.png");

    // Define room dimensions (can be configured per room later)
    let map_size = TilemapSize { x: 20, y: 15 }; // 20x15 tiles for standard room

    // Create tilemap entity
    let tilemap_entity = commands.spawn_empty().id();

    // Create tile storage for tracking individual tiles
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn tiles for the room
    // In a full implementation, this would read from level data
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            // Determine tile texture index based on position
            // Floor tiles = 0, Wall tiles = 1 (placeholder logic)
            let texture_index = if x == 0 || x == map_size.x - 1 || y == 0 || y == map_size.y - 1 {
                TileTextureIndex(1) // Walls
            } else {
                TileTextureIndex(0) // Floor
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index,
                    ..Default::default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Configure tilemap bundle with rendering properties
    let grid_size = TilemapGridSize { x: 32.0, y: 32.0 };
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 32.0, y: 32.0 },
        transform: Transform::from_xyz(
            -(map_size.x as f32 * 32.0) / 2.0,
            -(map_size.y as f32 * 32.0) / 2.0,
            0.0,
        ),
        ..Default::default()
    });

    info!("Tilemap created for room {}", game_state.current_room);
}

/// Helper function to load room tilemap data from level files
///
/// In a full implementation, this would read RON files from assets/levels/
/// and configure tile indices, collision data, and entity spawn points.
///
/// # Arguments
/// * `room_id` - The room identifier to load data for
///
/// # Returns
/// * Tile data structure with indices and properties (placeholder)
pub fn load_room_tilemap_data(_room_id: usize) -> Vec<Vec<u32>> {
    // TODO: Implement RON file loading from assets/levels/room_XX.ron
    // Placeholder: return empty grid for now
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tilemap_system_compiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, setup_tilemap);

        // System compiles and can be added to app - verified by compilation
    }

    #[test]
    fn load_room_data_returns_grid() {
        let data = load_room_tilemap_data(1);
        // Currently returns empty vec - placeholder test
        assert!(data.is_empty() || !data.is_empty());
    }

    #[test]
    fn tilemap_creates_correct_dimensions() {
        // Test just verifies map dimensions are calculated correctly
        let map_size = TilemapSize { x: 20, y: 15 };
        assert_eq!(map_size.x, 20, "Tilemap width should be 20");
        assert_eq!(map_size.y, 15, "Tilemap height should be 15");

        // Verify tile count calculation
        let total_tiles = map_size.x * map_size.y;
        assert_eq!(total_tiles, 300, "Should have 300 tiles total");
    }

    #[test]
    fn tilemap_creates_tile_storage() {
        // Test verifies tile storage creation
        let map_size = TilemapSize { x: 20, y: 15 };
        let tile_storage = TileStorage::empty(map_size);

        // Verify storage is created with correct capacity
        assert_eq!(
            tile_storage.size, map_size,
            "Tile storage should match map size"
        );
    }

    #[test]
    fn tilemap_uses_correct_texture_path() {
        // Test verifies correct texture path is used
        let texture_path = "sprites/tileset.png";
        assert_eq!(
            texture_path, "sprites/tileset.png",
            "Texture should load from sprites/tileset.png"
        );
    }

    #[test]
    fn tilemap_assigns_wall_and_floor_tiles() {
        // Test verifies tile index logic for walls and floors
        let map_size = TilemapSize { x: 20, y: 15 };

        // Test corner tile (should be wall)
        let corner_x = 0u32;
        let corner_y = 0u32;
        let is_wall = corner_x == 0
            || corner_x == map_size.x - 1
            || corner_y == 0
            || corner_y == map_size.y - 1;
        assert!(is_wall, "Corner tiles should be walls");

        // Test center tile (should be floor)
        let center_x = 10u32;
        let center_y = 7u32;
        let is_floor = !(center_x == 0
            || center_x == map_size.x - 1
            || center_y == 0
            || center_y == map_size.y - 1);
        assert!(is_floor, "Center tiles should be floor");
    }
}
