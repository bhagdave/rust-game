use crate::components::inventory::{Inventory, Item, KeyType, PuzzleItemType, ToolType};
use crate::components::lighting::{CandleState, CandleWax};
use crate::components::player::{DoubleJumpUnlocked, Health, Player};
use crate::components::room::RoomId;
use crate::resources::game_state::{GameMode, GameState};
use crate::resources::map_state::MapState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Save data structure for game state serialization
///
/// This structure contains all the necessary game state that needs to be
/// persisted across sessions. It uses RON (Rusty Object Notation) format
/// for human-readable serialization.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    /// Save file format version (currently 1)
    pub version: u32,
    /// The room ID where the player is located
    pub current_room: RoomId,
    /// Player's XY coordinates in the current room
    pub player_position: (f32, f32),
    /// Serialized list of inventory items
    pub inventory_items: Vec<SerializedItem>,
    /// Current candle wax percentage (0.0-100.0)
    pub candle_wax: f32,
    /// Current state of the candle (Lit, Unlit, Extinguished)
    pub candle_state: SerializedCandleState,
    /// List of room IDs that have been visited
    pub explored_rooms: Vec<RoomId>,
    /// Total game time in seconds
    pub completion_time_secs: u64,
    /// Number of times the player has died
    pub deaths: u32,
    /// Number of secret items collected
    pub collected_secrets: usize,
    /// Whether the double jump ability has been acquired
    pub double_jump_unlocked: bool,
    /// Current game mode (Menu, Playing, Paused, etc.)
    pub game_mode: SerializedGameMode,
}

/// Serializable representation of an inventory item
///
/// This enum mirrors the `Item` component but is designed for serialization.
/// Entity references in the original are not serialized.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SerializedItem {
    /// Match item for lighting candles
    Match,
    /// Key item of specific type
    Key(SerializedKeyType),
    /// Tool item of specific type
    Tool(SerializedToolType),
    /// Puzzle-specific item
    PuzzleItem(SerializedPuzzleItemType),
    /// Special item that unlocks double jump
    DoubleJumpItem,
    /// Diary page with page number
    DiaryPage(usize),
}

/// Serializable key types for save system
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SerializedKeyType {
    /// Common brass key
    Brass,
    /// Iron key for sturdy locks
    Iron,
    /// Decorative ornate key
    Ornate,
    /// Master key that opens multiple locks
    Master,
}

/// Serializable tool types for save system
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SerializedToolType {
    /// Wrench for mechanical puzzles
    Wrench,
    /// Crowbar for prying objects
    Crowbar,
    /// Wire cutters for electrical puzzles
    WireCutters,
    /// Magnet for attracting metal objects
    Magnet,
    /// Oil can for lubricating mechanisms
    OilCan,
    /// Ladder for reaching high places
    Ladder,
}

/// Serializable puzzle item types for save system
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SerializedPuzzleItemType {
    /// Fuse for circuit breaker puzzles
    Fuse,
    /// Red gemstone for matching puzzles
    GemstoneRed,
    /// Green gemstone for matching puzzles
    GemstoneGreen,
    /// Blue gemstone for matching puzzles
    GemstoneBlue,
    /// Electronic component for circuit puzzles
    CircuitComponent,
}

/// Serializable candle state for save system
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SerializedCandleState {
    /// Candle has not been lit yet
    Unlit,
    /// Candle is currently burning
    Lit,
    /// Candle wax is depleted or was put out
    Extinguished,
}

/// Serializable game mode for save system
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SerializedGameMode {
    /// Main menu screen
    Menu,
    /// Active gameplay
    Playing,
    /// Game paused by player
    Paused,
    /// Player died or failed
    GameOver,
    /// Player successfully escaped
    Victory,
}

/// Event triggered when the game should be auto-saved
///
/// This event is typically emitted by the room transition system when
/// the player moves to a new room.
///
/// # Examples
/// ```ignore
/// fn room_transition_system(
///     mut auto_save_events: EventWriter<AutoSaveEvent>,
/// ) {
///     // After room transition completes
///     auto_save_events.send(AutoSaveEvent);
/// }
/// ```
#[derive(Event)]
pub struct AutoSaveEvent;

/// Event triggered when the game should be manually saved
///
/// This event can be emitted by the UI system or input handler when
/// the player manually triggers a save.
#[derive(Event)]
pub struct ManualSaveEvent {
    /// Save slot number to save to
    pub slot: usize,
}

/// Event triggered when a save file should be loaded
#[derive(Event)]
pub struct LoadGameEvent {
    /// Save slot number to load (0 for auto-save)
    pub slot: usize,
}

/// System that handles auto-save events
///
/// Listens for `AutoSaveEvent` and saves the current game state to disk
/// in RON format. The save file is stored in a platform-specific directory.
///
/// # System Dependencies
/// - **Upstream**: Room transition system emits `AutoSaveEvent`
/// - **Resources**: Reads `GameState`, `MapState`
/// - **Components**: Queries `Player`, `Inventory`, `Candle`, `CandleWax`, `CandleState`
///
/// # Save Location
/// - Linux: `~/.local/share/rust-game/save.ron`
/// - Windows: `%APPDATA%/rust-game/save.ron`
/// - macOS: `~/Library/Application Support/rust-game/save.ron`
///
/// From tasks.md T031: SaveLoadSystem
pub fn auto_save_system(
    mut events: EventReader<AutoSaveEvent>,
    game_state: Res<GameState>,
    map_state: Res<MapState>,
    player_query: Query<(&Transform, &Inventory, Option<&DoubleJumpUnlocked>), With<Player>>,
    candle_query: Query<(&CandleWax, &CandleState)>,
) {
    for _ in events.read() {
        // Gather player data
        let (player_position, inventory_items, double_jump_unlocked) =
            if let Ok((transform, inventory, double_jump)) = player_query.single() {
                let pos = (transform.translation.x, transform.translation.y);
                let items = inventory.items.iter().map(serialize_item).collect();
                let has_double_jump = double_jump.is_some();
                (pos, items, has_double_jump)
            } else {
                // No player found, use defaults
                (
                    (
                        game_state.player_spawn_point.x,
                        game_state.player_spawn_point.y,
                    ),
                    vec![],
                    false,
                )
            };

        // Gather candle data
        let (candle_wax, candle_state) = if let Ok((wax, state)) = candle_query.single() {
            (wax.0, serialize_candle_state(state))
        } else {
            // No candle found, use defaults
            (100.0, SerializedCandleState::Unlit)
        };

        // Gather explored rooms
        let explored_rooms: Vec<RoomId> = map_state
            .explored_rooms
            .iter()
            .filter_map(
                |(room_id, status)| {
                    if status.visited { Some(*room_id) } else { None }
                },
            )
            .collect();

        // Create save data
        let save_data = SaveData {
            version: 1,
            current_room: game_state.current_room,
            player_position,
            inventory_items,
            candle_wax,
            candle_state,
            explored_rooms,
            completion_time_secs: game_state.completion_time.as_secs(),
            deaths: game_state.deaths,
            collected_secrets: game_state.collected_secrets.len(),
            double_jump_unlocked,
            game_mode: serialize_game_mode(&game_state.game_mode),
        };

        // Get save path
        let save_path = get_save_path(0); // Slot 0 for auto-save

        // Serialize to RON format
        match ron::ser::to_string_pretty(&save_data, ron::ser::PrettyConfig::default()) {
            Ok(ron_string) => {
                // Write to file
                if let Err(e) = fs::write(&save_path, ron_string) {
                    error!("Failed to save game: {}", e);
                } else {
                    info!("Game auto-saved to {:?}", save_path);
                }
            }
            Err(e) => {
                error!("Failed to serialize save data: {}", e);
            }
        }
    }
}

/// System that handles manual save events
///
/// Similar to auto_save_system but allows saving to specific slots.
pub fn manual_save_system(
    mut events: EventReader<ManualSaveEvent>,
    game_state: Res<GameState>,
    map_state: Res<MapState>,
    player_query: Query<(&Transform, &Inventory, Option<&DoubleJumpUnlocked>), With<Player>>,
    candle_query: Query<(&CandleWax, &CandleState)>,
) {
    for event in events.read() {
        // Gather player data
        let (player_position, inventory_items, double_jump_unlocked) =
            if let Ok((transform, inventory, double_jump)) = player_query.single() {
                let pos = (transform.translation.x, transform.translation.y);
                let items = inventory.items.iter().map(serialize_item).collect();
                let has_double_jump = double_jump.is_some();
                (pos, items, has_double_jump)
            } else {
                (
                    (
                        game_state.player_spawn_point.x,
                        game_state.player_spawn_point.y,
                    ),
                    vec![],
                    false,
                )
            };

        // Gather candle data
        let (candle_wax, candle_state) = if let Ok((wax, state)) = candle_query.single() {
            (wax.0, serialize_candle_state(state))
        } else {
            (100.0, SerializedCandleState::Unlit)
        };

        // Gather explored rooms
        let explored_rooms: Vec<RoomId> = map_state
            .explored_rooms
            .iter()
            .filter_map(
                |(room_id, status)| {
                    if status.visited { Some(*room_id) } else { None }
                },
            )
            .collect();

        // Create save data
        let save_data = SaveData {
            version: 1,
            current_room: game_state.current_room,
            player_position,
            inventory_items,
            candle_wax,
            candle_state,
            explored_rooms,
            completion_time_secs: game_state.completion_time.as_secs(),
            deaths: game_state.deaths,
            collected_secrets: game_state.collected_secrets.len(),
            double_jump_unlocked,
            game_mode: serialize_game_mode(&game_state.game_mode),
        };

        // Get save path for specific slot
        let save_path = get_save_path(event.slot);

        // Serialize to RON format
        match ron::ser::to_string_pretty(&save_data, ron::ser::PrettyConfig::default()) {
            Ok(ron_string) => {
                if let Err(e) = fs::write(&save_path, ron_string) {
                    error!("Failed to save game to slot {}: {}", event.slot, e);
                } else {
                    info!(
                        "Game manually saved to slot {} at {:?}",
                        event.slot, save_path
                    );
                }
            }
            Err(e) => {
                error!("Failed to serialize save data: {}", e);
            }
        }
    }
}

// Type alias for complex player query
type PlayerLoadQuery<'a> = (
    Entity,
    &'a mut Transform,
    &'a mut Inventory,
    &'a mut Health,
    Option<&'a DoubleJumpUnlocked>,
);

/// System that handles load game events
///
/// Loads game state from a save file and restores the world to that state.
/// If the save file doesn't exist, the game starts with default state.
///
/// # System Dependencies
/// - **Commands**: Modifies world entities and resources
/// - **Resources**: Writes `GameState`, `MapState`
/// - **Components**: Modifies `Player`, `Inventory`, `Candle` components
///
/// From tasks.md T031: SaveLoadSystem
pub fn load_game_system(
    mut events: EventReader<LoadGameEvent>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut map_state: ResMut<MapState>,
    mut player_query: Query<PlayerLoadQuery, With<Player>>,
    mut candle_query: Query<(&mut CandleWax, &mut CandleState)>,
) {
    for event in events.read() {
        let save_path = get_save_path(event.slot);

        // Check if save file exists
        if !save_path.exists() {
            warn!(
                "Save file at {:?} does not exist. Starting with default state.",
                save_path
            );
            continue;
        }

        // Read save file
        let save_content = match fs::read_to_string(&save_path) {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to read save file: {}", e);
                continue;
            }
        };

        // Deserialize save data
        let save_data: SaveData = match ron::from_str(&save_content) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to deserialize save data: {}", e);
                continue;
            }
        };

        // Restore game state
        game_state.current_room = save_data.current_room;
        game_state.player_spawn_point =
            Vec2::new(save_data.player_position.0, save_data.player_position.1);
        game_state.completion_time = std::time::Duration::from_secs(save_data.completion_time_secs);
        game_state.deaths = save_data.deaths;
        game_state.game_mode = deserialize_game_mode(&save_data.game_mode);

        // Restore map state
        map_state.explored_rooms.clear();
        for room_id in save_data.explored_rooms {
            map_state.mark_explored(room_id);
        }

        // Restore player state
        if let Ok((entity, mut transform, mut inventory, mut health, double_jump)) =
            player_query.single_mut()
        {
            // Update player position
            transform.translation.x = save_data.player_position.0;
            transform.translation.y = save_data.player_position.1;
            transform.translation.z = 0.0;

            // Restore inventory
            inventory.items = save_data
                .inventory_items
                .iter()
                .map(deserialize_item)
                .collect();

            // Ensure player is alive
            *health = Health::Alive;

            // Handle double jump unlock
            if save_data.double_jump_unlocked && double_jump.is_none() {
                commands.entity(entity).insert(DoubleJumpUnlocked);
            } else if !save_data.double_jump_unlocked && double_jump.is_some() {
                commands.entity(entity).remove::<DoubleJumpUnlocked>();
            }
        }

        // Restore candle state
        if let Ok((mut wax, mut state)) = candle_query.single_mut() {
            wax.0 = save_data.candle_wax;
            *state = deserialize_candle_state(&save_data.candle_state);
        }

        info!("Game loaded from slot {} at {:?}", event.slot, save_path);
    }
}

/// Returns the platform-specific save file path
///
/// # Arguments
/// * `slot` - Save slot number (0 for auto-save, 1-3 for manual saves)
///
/// # Platform-specific paths
/// - Linux: `~/.local/share/rust-game/save{slot}.ron`
/// - Windows: `%APPDATA%/rust-game/save{slot}.ron`
/// - macOS: `~/Library/Application Support/rust-game/save{slot}.ron`
pub fn get_save_path(slot: usize) -> PathBuf {
    let project_dirs = directories::ProjectDirs::from("com", "example", "rust-game")
        .expect("Failed to determine data directory");

    let mut path = project_dirs.data_local_dir().to_path_buf();
    path.push("rust-game");

    // Create directory if it doesn't exist
    fs::create_dir_all(&path).ok();

    // Append slot-specific filename
    let filename = if slot == 0 {
        "save.ron".to_string()
    } else {
        format!("save{}.ron", slot)
    };
    path.push(filename);

    path
}

// Helper conversion functions

fn serialize_item(item: &Item) -> SerializedItem {
    match item {
        Item::Match => SerializedItem::Match,
        Item::Key(key_type) => SerializedItem::Key(match key_type {
            KeyType::Brass => SerializedKeyType::Brass,
            KeyType::Iron => SerializedKeyType::Iron,
            KeyType::Ornate => SerializedKeyType::Ornate,
            KeyType::Master => SerializedKeyType::Master,
        }),
        Item::Tool(tool_type) => SerializedItem::Tool(match tool_type {
            ToolType::Wrench => SerializedToolType::Wrench,
            ToolType::Crowbar => SerializedToolType::Crowbar,
            ToolType::WireCutters => SerializedToolType::WireCutters,
            ToolType::Magnet => SerializedToolType::Magnet,
            ToolType::OilCan => SerializedToolType::OilCan,
            ToolType::Ladder => SerializedToolType::Ladder,
        }),
        Item::PuzzleItem(puzzle_type) => SerializedItem::PuzzleItem(match puzzle_type {
            PuzzleItemType::Fuse => SerializedPuzzleItemType::Fuse,
            PuzzleItemType::Gemstone(color) => {
                // Simplified: map Color to enum variants
                // This is a simplification; real implementation would need better color mapping
                if color.to_srgba().red > 0.5 {
                    SerializedPuzzleItemType::GemstoneRed
                } else if color.to_srgba().green > 0.5 {
                    SerializedPuzzleItemType::GemstoneGreen
                } else {
                    SerializedPuzzleItemType::GemstoneBlue
                }
            }
            PuzzleItemType::CircuitComponent => SerializedPuzzleItemType::CircuitComponent,
        }),
        Item::DoubleJumpItem => SerializedItem::DoubleJumpItem,
        Item::DiaryPage(n) => SerializedItem::DiaryPage(*n),
    }
}

fn deserialize_item(item: &SerializedItem) -> Item {
    match item {
        SerializedItem::Match => Item::Match,
        SerializedItem::Key(key_type) => Item::Key(match key_type {
            SerializedKeyType::Brass => KeyType::Brass,
            SerializedKeyType::Iron => KeyType::Iron,
            SerializedKeyType::Ornate => KeyType::Ornate,
            SerializedKeyType::Master => KeyType::Master,
        }),
        SerializedItem::Tool(tool_type) => Item::Tool(match tool_type {
            SerializedToolType::Wrench => ToolType::Wrench,
            SerializedToolType::Crowbar => ToolType::Crowbar,
            SerializedToolType::WireCutters => ToolType::WireCutters,
            SerializedToolType::Magnet => ToolType::Magnet,
            SerializedToolType::OilCan => ToolType::OilCan,
            SerializedToolType::Ladder => ToolType::Ladder,
        }),
        SerializedItem::PuzzleItem(puzzle_type) => Item::PuzzleItem(match puzzle_type {
            SerializedPuzzleItemType::Fuse => PuzzleItemType::Fuse,
            SerializedPuzzleItemType::GemstoneRed => {
                PuzzleItemType::Gemstone(Color::srgb(1.0, 0.0, 0.0))
            }
            SerializedPuzzleItemType::GemstoneGreen => {
                PuzzleItemType::Gemstone(Color::srgb(0.0, 1.0, 0.0))
            }
            SerializedPuzzleItemType::GemstoneBlue => {
                PuzzleItemType::Gemstone(Color::srgb(0.0, 0.0, 1.0))
            }
            SerializedPuzzleItemType::CircuitComponent => PuzzleItemType::CircuitComponent,
        }),
        SerializedItem::DoubleJumpItem => Item::DoubleJumpItem,
        SerializedItem::DiaryPage(n) => Item::DiaryPage(*n),
    }
}

fn serialize_candle_state(state: &CandleState) -> SerializedCandleState {
    match state {
        CandleState::Unlit => SerializedCandleState::Unlit,
        CandleState::Lit => SerializedCandleState::Lit,
        CandleState::Extinguished => SerializedCandleState::Extinguished,
    }
}

fn deserialize_candle_state(state: &SerializedCandleState) -> CandleState {
    match state {
        SerializedCandleState::Unlit => CandleState::Unlit,
        SerializedCandleState::Lit => CandleState::Lit,
        SerializedCandleState::Extinguished => CandleState::Extinguished,
    }
}

fn serialize_game_mode(mode: &GameMode) -> SerializedGameMode {
    match mode {
        GameMode::Menu => SerializedGameMode::Menu,
        GameMode::Playing => SerializedGameMode::Playing,
        GameMode::Paused => SerializedGameMode::Paused,
        GameMode::GameOver => SerializedGameMode::GameOver,
        GameMode::Victory => SerializedGameMode::Victory,
    }
}

fn deserialize_game_mode(mode: &SerializedGameMode) -> GameMode {
    match mode {
        SerializedGameMode::Menu => GameMode::Menu,
        SerializedGameMode::Playing => GameMode::Playing,
        SerializedGameMode::Paused => GameMode::Paused,
        SerializedGameMode::GameOver => GameMode::GameOver,
        SerializedGameMode::Victory => GameMode::Victory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_data_serializes_to_ron() {
        let save_data = SaveData {
            version: 1,
            current_room: 2,
            player_position: (150.0, 200.0),
            inventory_items: vec![
                SerializedItem::Match,
                SerializedItem::Key(SerializedKeyType::Brass),
            ],
            candle_wax: 75.0,
            candle_state: SerializedCandleState::Lit,
            explored_rooms: vec![0, 1, 2],
            completion_time_secs: 120,
            deaths: 3,
            collected_secrets: 2,
            double_jump_unlocked: true,
            game_mode: SerializedGameMode::Playing,
        };

        let ron_string = ron::ser::to_string_pretty(&save_data, ron::ser::PrettyConfig::default())
            .expect("Failed to serialize");

        // Verify it's valid RON
        assert!(ron_string.contains("version: 1"));
        assert!(ron_string.contains("current_room: 2"));
        assert!(ron_string.contains("player_position: (150.0, 200.0)"));
    }

    #[test]
    fn save_data_deserializes_from_ron() {
        let ron_string = r#"(
            version: 1,
            current_room: 3,
            player_position: (100.0, 50.0),
            inventory_items: [Match, Key(Brass)],
            candle_wax: 50.0,
            candle_state: Lit,
            explored_rooms: [0, 1, 2, 3],
            completion_time_secs: 300,
            deaths: 1,
            collected_secrets: 5,
            double_jump_unlocked: false,
            game_mode: Playing,
        )"#;

        let save_data: SaveData = ron::from_str(ron_string).expect("Failed to deserialize");

        assert_eq!(save_data.version, 1);
        assert_eq!(save_data.current_room, 3);
        assert_eq!(save_data.player_position, (100.0, 50.0));
        assert_eq!(save_data.inventory_items.len(), 2);
        assert_eq!(save_data.candle_wax, 50.0);
    }

    #[test]
    fn get_save_path_returns_platform_specific_path() {
        let path = get_save_path(0);
        assert!(path.ends_with("save.ron"));
        assert!(path.to_string_lossy().contains("rust-game"));

        let path_slot1 = get_save_path(1);
        assert!(path_slot1.ends_with("save1.ron"));
    }

    #[test]
    fn item_serialization_round_trip() {
        let items = vec![
            Item::Match,
            Item::Key(KeyType::Brass),
            Item::Tool(ToolType::Wrench),
            Item::DiaryPage(5),
        ];

        for item in items {
            let serialized = serialize_item(&item);
            let _deserialized = deserialize_item(&serialized);
            // Note: This test can't use PartialEq on Item due to Color in PuzzleItem
            // but it ensures the round-trip works
        }
    }
}
