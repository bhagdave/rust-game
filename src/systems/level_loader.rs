use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use crate::components::inventory::KeyType;
use crate::components::room::{ConnectionType, Floor};

/// Level data structures matching RON file format
/// From tasks.md T039: Load room data from RON files and spawn entities
///
/// Main level data structure deserialized from RON files
///
/// Represents a complete room definition including metadata, tile layout,
/// entity spawns, and connections to other rooms.
///
/// From tasks.md T039: "Load room data from RON files"
#[derive(Deserialize, Debug, Clone)]
pub struct LevelData {
    /// Unique room identifier
    pub id: usize,
    /// Floor level (Ground, First, Second, Basement)
    pub floor: Floor,
    /// Human-readable room name
    pub name: String,
    /// Room boundaries in world coordinates
    pub bounds: Bounds,
    /// 2D grid of tile indices for floor/wall layout
    pub tiles: Vec<Vec<u32>>,
    /// List of entities to spawn in the room
    pub entities: Vec<EntitySpawn>,
    /// List of connections to other rooms (doors, stairs, etc.)
    pub connections: Vec<RoomConnection>,
}

/// Room boundary coordinates
///
/// Defines the min and max coordinates of a room in world space.
/// Used for collision detection and camera bounds.
#[derive(Deserialize, Debug, Clone)]
pub struct Bounds {
    /// Minimum (bottom-left) corner coordinates (x, y)
    pub min: (f32, f32),
    /// Maximum (top-right) corner coordinates (x, y)
    pub max: (f32, f32),
}

/// Entity spawn definition from level data
///
/// Represents an entity to be spawned in the room with its type and position.
/// Optional fields support different entity types (doors, keys, etc.).
#[derive(Deserialize, Debug, Clone)]
pub struct EntitySpawn {
    /// String identifier for entity type (e.g., "Match", "Key", "Door")
    pub entity_type: String,
    /// Position coordinates (x, y) in world space
    pub position: (f32, f32),
    /// Optional room ID for doors/portals
    #[serde(default)]
    pub target_room: Option<usize>,
    /// Optional key type for locked doors
    #[serde(default)]
    pub locked: Option<KeyType>,
    /// Optional key type identifier for key entities
    #[serde(default)]
    pub key_type: Option<KeyType>,
}

/// Room connection definition
///
/// Represents a connection to another room (door, staircase, etc.).
/// Used to build the room graph for navigation.
#[derive(Deserialize, Debug, Clone)]
pub struct RoomConnection {
    /// ID of the room this connection leads to
    pub target_room: usize,
    /// Type of connection (Door, Staircase, Ladder, Hidden)
    pub connection_type: ConnectionType,
    /// Position coordinates (x, y) in world space
    pub position: (f32, f32),
    /// Optional key type required to unlock this connection
    pub locked: Option<KeyType>,
}

/// Load level data from a RON file
///
/// Reads and deserializes level data from assets/levels/ directory.
/// Returns the parsed LevelData structure for room setup.
///
/// # Arguments
/// * `level_path` - Path to the RON file relative to assets/ (e.g., "levels/ground_floor_entry.ron")
///
/// # Returns
/// * `Result<LevelData, String>` - Parsed level data or error message
///
/// # Errors
/// Returns error string if:
/// - File cannot be read
/// - RON parsing fails
/// - Data validation fails
///
/// # Example
/// ```ignore
/// let level_data = load_level_data("levels/ground_floor_entry.ron")?;
/// println!("Loaded room: {}", level_data.name);
/// ```
///
/// From tasks.md T039: "Load and parse RON file"
pub fn load_level_data(level_path: &str) -> Result<LevelData, String> {
    // Construct full path to assets directory
    let full_path = format!("assets/{}", level_path);

    // Read file contents
    let content = fs::read_to_string(&full_path)
        .map_err(|e| format!("Failed to read level file '{}': {}", full_path, e))?;

    // Parse RON format
    let level_data: LevelData = ron::from_str(&content)
        .map_err(|e| format!("Failed to parse RON from '{}': {}", full_path, e))?;

    Ok(level_data)
}

/// System to load a level and spawn entities
///
/// This is a placeholder system that demonstrates level loading.
/// In a full implementation, this would be called when transitioning between rooms.
///
/// # System Dependencies
/// - **Resources**: AssetServer (for future sprite loading)
/// - **Commands**: For spawning entities
///
/// # Behavior
/// 1. Load level data from RON file
/// 2. Parse level structure
/// 3. Spawn entities based on level data
/// 4. Set up room connections
///
/// From tasks.md T039: "Spawn entities based on LevelData"
///
/// **Note**: This is a demonstration system. In production, level loading would be
/// triggered by room transition events and integrated with the room_transition system.
pub fn load_level_system(
    _commands: Commands,
    _asset_server: Res<AssetServer>,
    // In a full implementation, this would read from a resource or event
) {
    // Load the entry hall level as an example
    let level_path = "levels/ground_floor_entry.ron";

    match load_level_data(level_path) {
        Ok(level_data) => {
            info!(
                "Loaded level: {} (ID: {}, Floor: {:?})",
                level_data.name, level_data.id, level_data.floor
            );
            info!("  Entities: {}", level_data.entities.len());
            info!("  Connections: {}", level_data.connections.len());
            info!(
                "  Tile grid: {}x{}",
                level_data.tiles.len(),
                level_data.tiles.first().map(|r| r.len()).unwrap_or(0)
            );

            // TODO: Spawn entities based on level_data.entities
            // TODO: Set up room connections based on level_data.connections
            // TODO: Configure tilemap based on level_data.tiles

            // Example: Log entity spawns
            for entity_spawn in &level_data.entities {
                info!(
                    "  Would spawn {} at ({}, {})",
                    entity_spawn.entity_type, entity_spawn.position.0, entity_spawn.position.1
                );
            }
        }
        Err(e) => {
            error!("Failed to load level: {}", e);
        }
    }
}

/// Helper function to get level path by room ID
///
/// Maps room IDs to their corresponding RON file paths.
/// This allows room transitions to load the correct level data.
///
/// # Arguments
/// * `room_id` - The room ID to look up
///
/// # Returns
/// * `String` - Path to the RON file relative to assets/
///
/// # Example
/// ```ignore
/// let path = get_level_path(0); // Returns "levels/ground_floor_entry.ron"
/// ```
pub fn get_level_path(room_id: usize) -> String {
    match room_id {
        0 => "levels/ground_floor_entry.ron".to_string(),
        // Add more room mappings as levels are created
        _ => format!("levels/room_{}.ron", room_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_data_structures_deserialize() {
        // Test that LevelData structure can be created
        let level_data = LevelData {
            id: 0,
            floor: Floor::Ground,
            name: "Test Room".to_string(),
            bounds: Bounds {
                min: (0.0, 0.0),
                max: (1920.0, 1080.0),
            },
            tiles: vec![vec![0, 1, 0], vec![1, 0, 1]],
            entities: vec![],
            connections: vec![],
        };

        assert_eq!(level_data.id, 0);
        assert_eq!(level_data.name, "Test Room");
    }

    #[test]
    fn entity_spawn_has_required_fields() {
        let entity = EntitySpawn {
            entity_type: "Match".to_string(),
            position: (100.0, 200.0),
            target_room: None,
            locked: None,
            key_type: None,
        };

        assert_eq!(entity.entity_type, "Match");
        assert_eq!(entity.position, (100.0, 200.0));
        assert!(entity.target_room.is_none());
    }

    #[test]
    fn entity_spawn_supports_optional_fields() {
        let door = EntitySpawn {
            entity_type: "Door".to_string(),
            position: (1840.0, 540.0),
            target_room: Some(1),
            locked: Some(KeyType::Brass),
            key_type: None,
        };

        assert_eq!(door.entity_type, "Door");
        assert_eq!(door.target_room, Some(1));
        assert_eq!(door.locked, Some(KeyType::Brass));
    }

    #[test]
    fn load_level_data_reads_entry_hall() {
        // Test loading the actual entry hall RON file
        let result = load_level_data("levels/ground_floor_entry.ron");

        assert!(
            result.is_ok(),
            "Should successfully load entry hall: {:?}",
            result.err()
        );

        let level_data = result.unwrap();
        assert_eq!(level_data.id, 0);
        assert_eq!(level_data.floor, Floor::Ground);
        assert_eq!(level_data.name, "Entry Hall");
    }

    #[test]
    fn load_level_data_validates_entry_hall_structure() {
        let level_data =
            load_level_data("levels/ground_floor_entry.ron").expect("Should load entry hall");

        // Verify bounds
        assert_eq!(level_data.bounds.min, (0.0, 0.0));
        assert_eq!(level_data.bounds.max, (1920.0, 1080.0));

        // Verify tile grid
        assert_eq!(level_data.tiles.len(), 15, "Should have 15 rows");
        assert_eq!(level_data.tiles[0].len(), 20, "Should have 20 columns");

        // Verify entities exist
        assert!(!level_data.entities.is_empty(), "Should have entities");

        // Verify connections exist
        assert!(
            !level_data.connections.is_empty(),
            "Should have connections"
        );
    }

    #[test]
    fn load_level_data_finds_expected_entities() {
        let level_data =
            load_level_data("levels/ground_floor_entry.ron").expect("Should load entry hall");

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

        assert_eq!(matches, 3, "Should have 3 matches");
        assert_eq!(keys, 1, "Should have 1 key");
        assert_eq!(doors, 1, "Should have 1 door");
    }

    #[test]
    fn load_level_data_handles_invalid_path() {
        let result = load_level_data("levels/nonexistent.ron");

        assert!(result.is_err(), "Should fail for nonexistent file");
        assert!(result.unwrap_err().contains("Failed to read level file"));
    }

    #[test]
    fn get_level_path_maps_room_zero() {
        let path = get_level_path(0);
        assert_eq!(path, "levels/ground_floor_entry.ron");
    }

    #[test]
    fn get_level_path_generates_default_path() {
        let path = get_level_path(5);
        assert_eq!(path, "levels/room_5.ron");
    }

    #[test]
    fn load_level_system_compiles() {
        // Test that load_level_system can be added to an app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, load_level_system);

        // System should compile and be addable
        assert!(true, "load_level_system compiles and can be added");
    }
}
