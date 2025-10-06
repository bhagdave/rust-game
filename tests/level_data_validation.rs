use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Integration test: Level data validation
/// From tasks.md T038: Verify RON file parses and room data can be deserialized

// Data structures matching the RON file format
// These will be used by the level loading system in T039

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Floor {
    Ground,
    First,
    Second,
    Basement,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum KeyType {
    Brass,
    Iron,
    Ornate,
    Master,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ConnectionType {
    Door,
    Staircase,
    Ladder,
    Hidden,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub id: usize,
    pub floor: Floor,
    pub name: String,
    pub bounds: Bounds,
    pub tiles: Vec<Vec<u32>>,
    pub entities: Vec<EntitySpawn>,
    pub connections: Vec<RoomConnection>,
}

#[derive(Deserialize, Debug)]
pub struct Bounds {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

#[derive(Deserialize, Debug)]
pub struct EntitySpawn {
    pub entity_type: String,
    pub position: (f32, f32),
    #[serde(default)]
    pub target_room: Option<usize>,
    #[serde(default)]
    pub locked: Option<KeyType>,
    #[serde(default)]
    pub key_type: Option<KeyType>,
}

#[derive(Deserialize, Debug)]
pub struct RoomConnection {
    pub target_room: usize,
    pub connection_type: ConnectionType,
    pub position: (f32, f32),
    pub locked: Option<KeyType>,
}

#[test]
fn level_file_exists() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    assert!(
        level_path.exists(),
        "Level file should exist at assets/levels/ground_floor_entry.ron"
    );
}

#[test]
fn level_file_is_not_empty() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    assert!(!content.trim().is_empty(), "Level file should not be empty");

    // Should have reasonable minimum size (comments + data)
    assert!(
        content.len() > 200,
        "Level file should contain substantial data (> 200 bytes)"
    );
}

#[test]
fn level_data_parses_from_ron() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: Result<LevelData, _> = ron::from_str(&content);

    assert!(
        level_data.is_ok(),
        "Level data should parse from RON format: {:?}",
        level_data.err()
    );
}

#[test]
fn level_data_has_correct_structure() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify room metadata
    assert_eq!(level_data.id, 0, "Entry hall should have ID 0");
    assert_eq!(
        level_data.floor,
        Floor::Ground,
        "Entry hall should be on ground floor"
    );
    assert_eq!(
        level_data.name, "Entry Hall",
        "Room name should be 'Entry Hall'"
    );
}

#[test]
fn level_data_has_valid_bounds() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify bounds are valid
    assert_eq!(
        level_data.bounds.min,
        (0.0, 0.0),
        "Bounds min should be (0, 0)"
    );
    assert_eq!(
        level_data.bounds.max,
        (1920.0, 1080.0),
        "Bounds max should be (1920, 1080)"
    );

    // Verify bounds are positive and non-zero
    assert!(
        level_data.bounds.max.0 > level_data.bounds.min.0,
        "Max X should be greater than min X"
    );
    assert!(
        level_data.bounds.max.1 > level_data.bounds.min.1,
        "Max Y should be greater than min Y"
    );
}

#[test]
fn level_data_has_tile_grid() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify tile grid exists and has expected dimensions
    assert!(
        !level_data.tiles.is_empty(),
        "Tile grid should not be empty"
    );
    assert_eq!(level_data.tiles.len(), 15, "Tile grid should have 15 rows");

    // Verify all rows have same width
    let expected_width = 20;
    for (idx, row) in level_data.tiles.iter().enumerate() {
        assert_eq!(
            row.len(),
            expected_width,
            "Row {} should have {} columns",
            idx,
            expected_width
        );
    }
}

#[test]
fn level_data_tiles_are_valid_indices() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify tile indices are within valid range (0-1 for floor/wall)
    for (row_idx, row) in level_data.tiles.iter().enumerate() {
        for (col_idx, &tile) in row.iter().enumerate() {
            assert!(
                tile <= 1,
                "Tile at ({}, {}) should be 0 (floor) or 1 (wall), got {}",
                row_idx,
                col_idx,
                tile
            );
        }
    }
}

#[test]
fn level_data_has_entities() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify entities exist
    assert!(
        !level_data.entities.is_empty(),
        "Level should have entities"
    );

    // Verify entity types are valid strings
    for entity in &level_data.entities {
        assert!(
            !entity.entity_type.is_empty(),
            "Entity type should not be empty"
        );
    }
}

#[test]
fn level_data_entity_positions_within_bounds() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify all entity positions are within room bounds
    for entity in &level_data.entities {
        assert!(
            entity.position.0 >= level_data.bounds.min.0
                && entity.position.0 <= level_data.bounds.max.0,
            "Entity {} X position {} should be within bounds [{}, {}]",
            entity.entity_type,
            entity.position.0,
            level_data.bounds.min.0,
            level_data.bounds.max.0
        );
        assert!(
            entity.position.1 >= level_data.bounds.min.1
                && entity.position.1 <= level_data.bounds.max.1,
            "Entity {} Y position {} should be within bounds [{}, {}]",
            entity.entity_type,
            entity.position.1,
            level_data.bounds.min.1,
            level_data.bounds.max.1
        );
    }
}

#[test]
fn level_data_has_connections() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify connections exist
    assert!(
        !level_data.connections.is_empty(),
        "Level should have connections to other rooms"
    );

    // Verify connection target rooms are valid
    for connection in &level_data.connections {
        assert!(
            connection.target_room > 0 || connection.target_room == level_data.id,
            "Connection target room should be a valid room ID"
        );
    }
}

#[test]
fn level_data_connection_positions_within_bounds() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Verify all connection positions are within or at room bounds
    for connection in &level_data.connections {
        // Connections can be at the edge (on walls), so use >= and <= instead of strict bounds
        assert!(
            connection.position.0 >= level_data.bounds.min.0
                && connection.position.0 <= level_data.bounds.max.0,
            "Connection X position {} should be within bounds [{}, {}]",
            connection.position.0,
            level_data.bounds.min.0,
            level_data.bounds.max.0
        );
        assert!(
            connection.position.1 >= level_data.bounds.min.1
                && connection.position.1 <= level_data.bounds.max.1,
            "Connection Y position {} should be within bounds [{}, {}]",
            connection.position.1,
            level_data.bounds.min.1,
            level_data.bounds.max.1
        );
    }
}

#[test]
fn level_data_has_expected_entity_types() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Collect entity types
    let entity_types: Vec<&str> = level_data
        .entities
        .iter()
        .map(|e| e.entity_type.as_str())
        .collect();

    // Verify expected entity types are present
    assert!(
        entity_types.contains(&"Match"),
        "Level should contain Match entities"
    );
    assert!(
        entity_types.contains(&"Key"),
        "Level should contain Key entity"
    );
    assert!(
        entity_types.contains(&"Door"),
        "Level should contain Door entity"
    );
    assert!(
        entity_types.contains(&"Candle"),
        "Level should contain Candle entity"
    );
    assert!(
        entity_types.contains(&"PlayerSpawn"),
        "Level should contain PlayerSpawn point"
    );
}

#[test]
fn level_data_door_has_required_fields() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Find door entity
    let door = level_data
        .entities
        .iter()
        .find(|e| e.entity_type == "Door")
        .expect("Level should have a Door entity");

    // Verify door has target room
    assert!(
        door.target_room.is_some(),
        "Door should have a target_room field"
    );

    // Verify door has lock status
    assert!(door.locked.is_some(), "Door should have a locked field");
    assert_eq!(
        door.locked.unwrap(),
        KeyType::Brass,
        "Door should be locked with Brass key"
    );
}

#[test]
fn level_data_key_has_key_type() {
    let level_path = Path::new("assets/levels/ground_floor_entry.ron");
    let content = fs::read_to_string(level_path).expect("Should be able to read level file");

    let level_data: LevelData = ron::from_str(&content).expect("Should parse RON data");

    // Find key entity
    let key = level_data
        .entities
        .iter()
        .find(|e| e.entity_type == "Key")
        .expect("Level should have a Key entity");

    // Verify key has key_type
    assert!(key.key_type.is_some(), "Key should have a key_type field");
    assert_eq!(
        key.key_type.unwrap(),
        KeyType::Brass,
        "Key should be a Brass key"
    );
}
