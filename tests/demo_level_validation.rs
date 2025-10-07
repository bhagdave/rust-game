/// Test suite for demo level validation
/// From tasks.md T002: Validate demo.ron structure and content
///
/// Tests:
/// - Demo level RON file loads successfully
/// - Structure matches LevelData format
/// - Contains required entities (player spawn, doors, items)
/// - Tile grid is correctly sized
/// - Entity positions are within bounds
use rust_game::components::room::Floor;
use rust_game::systems::level_loader::{EntitySpawn, load_level_data};

#[test]
fn demo_level_loads_successfully() {
    // Test that demo.ron can be loaded
    let result = load_level_data("levels/demo.ron");

    assert!(
        result.is_ok(),
        "Demo level should load successfully: {:?}",
        result.err()
    );
}

#[test]
fn demo_level_has_correct_metadata() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    assert_eq!(level_data.id, 100, "Demo level should have ID 100");
    assert_eq!(
        level_data.floor,
        Floor::Ground,
        "Demo level should be on ground floor"
    );
    assert_eq!(
        level_data.name, "Demo Level",
        "Demo level should have correct name"
    );
}

#[test]
fn demo_level_has_correct_bounds() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    assert_eq!(
        level_data.bounds.min,
        (0.0, 0.0),
        "Minimum bounds should be (0, 0)"
    );
    assert_eq!(
        level_data.bounds.max,
        (1920.0, 1080.0),
        "Maximum bounds should be (1920, 1080)"
    );
}

#[test]
fn demo_level_has_correct_tile_grid_size() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    assert_eq!(level_data.tiles.len(), 15, "Should have 15 rows");

    // Verify all rows have same length
    for (i, row) in level_data.tiles.iter().enumerate() {
        assert_eq!(row.len(), 20, "Row {} should have 20 columns", i);
    }
}

#[test]
fn demo_level_has_player_spawn() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    let player_spawns: Vec<&EntitySpawn> = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "PlayerSpawn")
        .collect();

    assert_eq!(player_spawns.len(), 1, "Should have exactly 1 player spawn");

    let spawn = player_spawns[0];
    assert_eq!(
        spawn.position,
        (960.0, 540.0),
        "Player should spawn at center"
    );
}

#[test]
fn demo_level_has_required_entities() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    // Count entity types
    let matches = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Match")
        .count();
    let keys = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Key")
        .count();
    let doors = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Door")
        .count();

    assert_eq!(matches, 2, "Should have 2 matches (as per T002 spec)");
    assert_eq!(keys, 2, "Should have 2 keys (as per T002 spec)");
    assert_eq!(doors, 2, "Should have 2 doors (as per T002 spec)");
}

#[test]
fn demo_level_doors_have_varied_lock_states() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    let doors: Vec<&EntitySpawn> = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Door")
        .collect();

    let locked_doors = doors.iter().filter(|d| d.locked.is_some()).count();
    let unlocked_doors = doors.iter().filter(|d| d.locked.is_none()).count();

    assert_eq!(locked_doors, 1, "Should have 1 locked door");
    assert_eq!(unlocked_doors, 1, "Should have 1 unlocked door");
}

#[test]
fn demo_level_keys_have_types() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    let keys: Vec<&EntitySpawn> = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Key")
        .collect();

    for key in keys {
        assert!(
            key.key_type.is_some(),
            "Key at ({}, {}) should have a key_type",
            key.position.0,
            key.position.1
        );
    }
}

#[test]
fn demo_level_all_entity_positions_within_bounds() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    let (min_x, min_y) = level_data.bounds.min;
    let (max_x, max_y) = level_data.bounds.max;

    for entity in &level_data.entities {
        let (x, y) = entity.position;
        assert!(
            x >= min_x && x <= max_x,
            "Entity {} at ({}, {}) has X position outside bounds ({}, {})",
            entity.entity_type,
            x,
            y,
            min_x,
            max_x
        );
        assert!(
            y >= min_y && y <= max_y,
            "Entity {} at ({}, {}) has Y position outside bounds ({}, {})",
            entity.entity_type,
            x,
            y,
            min_y,
            max_y
        );
    }
}

#[test]
fn demo_level_has_connections() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    assert_eq!(
        level_data.connections.len(),
        2,
        "Should have 2 connections (one locked, one unlocked)"
    );
}

#[test]
fn demo_level_connections_match_doors() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    // Verify connections align with door entities
    let doors: Vec<&EntitySpawn> = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Door")
        .collect();

    assert_eq!(
        doors.len(),
        level_data.connections.len(),
        "Number of doors should match number of connections"
    );
}

#[test]
fn demo_level_has_candle() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    let candles = level_data
        .entities
        .iter()
        .filter(|e| e.entity_type == "Candle")
        .count();

    assert_eq!(candles, 1, "Should have exactly 1 candle");
}

#[test]
fn demo_level_tile_grid_has_walls() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    // Check top row has walls (all 1s)
    let top_row = &level_data.tiles[0];
    assert!(
        top_row.iter().all(|&tile| tile == 1),
        "Top row should be all walls (tile index 1)"
    );

    // Check bottom row has walls
    let bottom_row = &level_data.tiles[level_data.tiles.len() - 1];
    assert!(
        bottom_row.iter().all(|&tile| tile == 1),
        "Bottom row should be all walls (tile index 1)"
    );

    // Check left and right columns have walls
    for row in &level_data.tiles {
        assert_eq!(row[0], 1, "Left column should be walls");
        assert_eq!(row[row.len() - 1], 1, "Right column should be walls");
    }
}

#[test]
fn demo_level_tile_grid_has_floor_space() {
    let level_data = load_level_data("levels/demo.ron").expect("Should load demo level");

    // Count floor tiles (tile index 0)
    let mut floor_count = 0;
    for row in &level_data.tiles {
        floor_count += row.iter().filter(|&&tile| tile == 0).count();
    }

    assert!(
        floor_count > 0,
        "Demo level should have some floor tiles (tile index 0)"
    );

    // Most tiles should be floor (not walls)
    let total_tiles = level_data.tiles.len() * level_data.tiles[0].len();
    assert!(
        floor_count as f32 / total_tiles as f32 > 0.5,
        "More than 50% of tiles should be floor space"
    );
}
