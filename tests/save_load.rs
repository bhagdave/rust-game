use bevy::prelude::*;
use rust_game::components::inventory::*;
use rust_game::components::lighting::*;
use rust_game::components::player::*;
use rust_game::resources::game_state::*;
use rust_game::resources::map_state::*;
use rust_game::systems::save_load::*;
use std::fs;
use std::time::Duration;

#[test]
fn auto_save_on_room_transition() {
    // Clean up any existing save files before test
    let save_path = get_save_path(0);
    let _ = fs::remove_file(&save_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add save/load systems and events
    app.add_event::<AutoSaveEvent>();
    app.add_event::<LoadGameEvent>();
    app.add_systems(Update, (auto_save_system, load_game_system));

    // Setup: Insert initial GameState (player in room 0)
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::from_secs(120), // 2 minutes played
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Insert MapState
    let mut map_state = MapState::default();
    map_state.mark_explored(0); // Room 0 already explored
    app.insert_resource(map_state);

    // Setup: Spawn player with inventory in room A (room 0)
    let player_entity = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Health::Alive,
            Inventory {
                items: vec![
                    Item::Match,
                    Item::Key(KeyType::Brass),
                    Item::Tool(ToolType::Wrench),
                    Item::DiaryPage(1),
                ],
                max_capacity: 10,
            },
        ))
        .id();

    // Spawn candle with current state
    let candle_entity = app
        .world_mut()
        .spawn((
            Candle,
            CandleWax(65.0), // 65% wax
            CandleState::Lit,
            BurnRate(1.0),
            VisibilityRadius(7.0),
        ))
        .id();

    // Assert: Player starts in room 0
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 0, "Player should start in room 0");
    }

    // Act: Trigger auto-save
    app.world_mut().send_event(AutoSaveEvent);
    app.update();

    // Assert: Save file exists
    assert!(save_path.exists(), "Save file should exist after auto-save");

    // Act: Modify state (simulate room transition)
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 1;
        game_state.player_spawn_point = Vec2::new(50.0, 50.0);
    }
    {
        let mut map_state = app.world_mut().resource_mut::<MapState>();
        map_state.mark_explored(1);
    }
    // Modify player inventory
    {
        let mut inventory = app.world_mut().get_mut::<Inventory>(player_entity).unwrap();
        inventory.items.push(Item::Key(KeyType::Iron));
    }

    // Verify state changed
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 1);
    }

    // Act: Load game from save file (should restore to room 0)
    app.world_mut().send_event(LoadGameEvent { slot: 0 });
    app.update();

    // Assert: Game state restored to room 0
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(
            game_state.current_room, 0,
            "Current room should be restored to 0"
        );
        assert_eq!(
            game_state.player_spawn_point,
            Vec2::new(100.0, 100.0),
            "Spawn point should be restored"
        );
        assert_eq!(
            game_state.completion_time.as_secs(),
            120,
            "Completion time should be preserved"
        );
    }

    // Assert: Player inventory restored
    {
        let inventory = app.world().get::<Inventory>(player_entity).unwrap();
        assert_eq!(
            inventory.items.len(),
            4,
            "Inventory should have 4 items after load"
        );
        assert!(matches!(inventory.items[0], Item::Match));
        assert!(matches!(inventory.items[1], Item::Key(KeyType::Brass)));
        assert!(matches!(inventory.items[2], Item::Tool(ToolType::Wrench)));
        assert!(matches!(inventory.items[3], Item::DiaryPage(1)));
    }

    // Assert: Map state restored
    {
        let map_state = app.world().resource::<MapState>();
        assert!(
            map_state.is_visited(0),
            "Room 0 should be marked as visited"
        );
        // Room 1 should NOT be visited (was added after save)
        assert!(
            !map_state.is_visited(1),
            "Room 1 should not be visited after load"
        );
    }

    // Assert: Candle state preserved
    {
        let wax = app.world().get::<CandleWax>(candle_entity).unwrap();
        assert_eq!(wax.0, 65.0, "Candle wax should be preserved");

        let state = app.world().get::<CandleState>(candle_entity).unwrap();
        assert_eq!(*state, CandleState::Lit, "Candle state should be preserved");
    }

    // Cleanup
    let _ = fs::remove_file(&save_path);
}

#[test]
fn manual_save_preserves_all_state() {
    // Clean up save file
    let save_path = get_save_path(20);
    let _ = fs::remove_file(&save_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add save/load systems
    app.add_event::<ManualSaveEvent>();
    app.add_event::<LoadGameEvent>();
    app.add_systems(Update, (manual_save_system, load_game_system));

    // Setup: Complex game state
    app.insert_resource(GameState {
        current_room: 3,
        player_spawn_point: Vec2::new(300.0, 200.0),
        completion_time: Duration::from_secs(600), // 10 minutes
        collected_secrets: {
            let mut set = std::collections::HashSet::new();
            set.insert(Entity::from_raw(1));
            set.insert(Entity::from_raw(2));
            set
        },
        game_mode: GameMode::Playing,
        deaths: 5,
    });

    // Map with multiple explored rooms
    let mut map_state = MapState::default();
    map_state.mark_explored(0);
    map_state.mark_explored(1);
    map_state.mark_explored(2);
    map_state.mark_explored(3);
    app.insert_resource(map_state);

    // Player with full inventory
    let player_entity = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(300.0, 200.0, 0.0),
            Health::Alive,
            Inventory {
                items: vec![
                    Item::Match,
                    Item::Match,
                    Item::Match,
                    Item::Key(KeyType::Brass),
                    Item::Key(KeyType::Iron),
                    Item::Tool(ToolType::Crowbar),
                    Item::PuzzleItem(PuzzleItemType::Fuse),
                    Item::DiaryPage(1),
                    Item::DiaryPage(2),
                    Item::DoubleJumpItem,
                ],
                max_capacity: 10,
            },
            DoubleJumpUnlocked, // Powerup acquired
        ))
        .id();

    // Spawn candle
    app.world_mut().spawn((
        Candle,
        CandleWax(45.0),
        CandleState::Lit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Act: Trigger manual save to slot 20
    app.world_mut().send_event(ManualSaveEvent { slot: 20 });
    app.update();

    // Assert: Save file exists
    assert!(
        save_path.exists(),
        "Save file should exist after manual save"
    );

    // Act: Modify state significantly
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 0;
        game_state.deaths = 10;
    }
    {
        let mut inventory = app.world_mut().get_mut::<Inventory>(player_entity).unwrap();
        inventory.items.clear();
    }

    // Act: Load from slot 20
    app.world_mut().send_event(LoadGameEvent { slot: 20 });
    app.update();

    // Assert: All state restored
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 3, "Current room should be 3");
        assert_eq!(
            game_state.player_spawn_point,
            Vec2::new(300.0, 200.0),
            "Spawn point should be (300, 200)"
        );
        assert_eq!(
            game_state.completion_time.as_secs(),
            600,
            "Completion time should be 600 seconds"
        );
        assert_eq!(game_state.deaths, 5, "Death count should be 5");
        assert_eq!(
            game_state.collected_secrets.len(),
            2,
            "Should have 2 collected secrets (note: entity refs not persisted)"
        );
    }

    // Assert: All 10 inventory items preserved
    {
        let inventory = app.world().get::<Inventory>(player_entity).unwrap();
        assert_eq!(inventory.items.len(), 10, "Should have 10 inventory items");
    }

    // Assert: Double jump unlocked
    {
        let has_double_jump = app
            .world()
            .get::<DoubleJumpUnlocked>(player_entity)
            .is_some();
        assert!(has_double_jump, "Double jump should be unlocked");
    }

    // Assert: All 4 explored rooms
    {
        let map_state = app.world().resource::<MapState>();
        assert!(map_state.is_visited(0), "Room 0 should be explored");
        assert!(map_state.is_visited(1), "Room 1 should be explored");
        assert!(map_state.is_visited(2), "Room 2 should be explored");
        assert!(map_state.is_visited(3), "Room 3 should be explored");
    }

    // Cleanup
    let _ = fs::remove_file(&save_path);
}

#[test]
fn save_file_format_is_ron() {
    // This test verifies the save file uses RON format for human readability
    let save_path = get_save_path(11);
    let _ = fs::remove_file(&save_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ManualSaveEvent>();
    app.add_systems(Update, manual_save_system);

    app.insert_resource(GameState {
        current_room: 1,
        player_spawn_point: Vec2::new(150.0, 150.0),
        completion_time: Duration::from_secs(60),
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });
    app.insert_resource(MapState::default());

    // Spawn player and candle
    app.world_mut().spawn((
        Player,
        Transform::from_xyz(150.0, 150.0, 0.0),
        Health::Alive,
        Inventory {
            items: vec![Item::Match],
            max_capacity: 10,
        },
    ));
    app.world_mut().spawn((
        Candle,
        CandleWax(100.0),
        CandleState::Unlit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Act: Save game
    app.world_mut().send_event(ManualSaveEvent { slot: 11 });
    app.update();

    // Assert: Save file exists and is RON format
    assert!(save_path.exists(), "Save file should exist");

    let content = fs::read_to_string(&save_path).expect("Failed to read save file");

    // Verify RON syntax
    assert!(content.contains("version: 1"), "Should have version field");
    assert!(
        content.contains("current_room: 1"),
        "Should have current_room field"
    );
    assert!(
        content.contains("player_position: (150.0, 150.0)"),
        "Should have player_position tuple"
    );
    assert!(content.contains("("), "Should contain RON parentheses");
    assert!(content.contains(":"), "Should contain RON field separators");

    // Cleanup
    let _ = fs::remove_file(&save_path);
}

#[test]
fn save_to_platform_specific_directory() {
    // Verifies save file goes to correct platform directory
    // Linux: ~/.local/share/rust-game/
    // Windows: %APPDATA%/rust-game/
    // macOS: ~/Library/Application Support/rust-game/

    let save_path = get_save_path(12);
    let _ = fs::remove_file(&save_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ManualSaveEvent>();
    app.add_systems(Update, manual_save_system);

    app.insert_resource(GameState::default());
    app.insert_resource(MapState::default());

    // Spawn minimal player and candle
    app.world_mut().spawn((
        Player,
        Transform::default(),
        Health::Alive,
        Inventory {
            items: vec![],
            max_capacity: 10,
        },
    ));
    app.world_mut().spawn((
        Candle,
        CandleWax(100.0),
        CandleState::Unlit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Act: Trigger save
    app.world_mut().send_event(ManualSaveEvent { slot: 12 });
    app.update();

    // Assert: Save file exists at expected platform-specific path
    assert!(
        save_path.exists(),
        "Save file should exist at {:?}",
        save_path
    );
    assert!(
        save_path.to_string_lossy().contains("rust-game"),
        "Path should contain 'rust-game' directory"
    );

    // Verify parent directory was created
    let parent_dir = save_path.parent().expect("Should have parent directory");
    assert!(
        parent_dir.exists(),
        "Parent directory should exist at {:?}",
        parent_dir
    );

    // Cleanup
    let _ = fs::remove_file(&save_path);
}

#[test]
fn load_nonexistent_save_returns_default_state() {
    // Verifies game starts fresh if no save file exists

    let save_path = get_save_path(99); // Use unlikely slot number
    let _ = fs::remove_file(&save_path); // Ensure it doesn't exist

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<LoadGameEvent>();
    app.add_systems(Update, load_game_system);

    // Setup default state
    app.insert_resource(GameState::default());
    app.insert_resource(MapState::default());

    app.world_mut().spawn((
        Player,
        Transform::default(),
        Health::Alive,
        Inventory {
            items: vec![],
            max_capacity: 10,
        },
    ));
    app.world_mut().spawn((
        Candle,
        CandleWax(100.0),
        CandleState::Unlit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Verify save file doesn't exist
    assert!(!save_path.exists(), "Save file should not exist");

    // Act: Try to load from nonexistent save
    app.world_mut().send_event(LoadGameEvent { slot: 99 });
    app.update();

    // Assert: Game state remains default (no crash, no changes)
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(
            game_state.current_room, 0,
            "Should remain in default room 0"
        );
        assert_eq!(
            game_state.deaths, 0,
            "Deaths should remain at default value"
        );
    }
}

#[test]
fn multiple_save_slots_supported() {
    // Verifies game can maintain multiple save files

    let slot1_path = get_save_path(1);
    let slot2_path = get_save_path(2);
    let slot3_path = get_save_path(3);

    // Cleanup
    let _ = fs::remove_file(&slot1_path);
    let _ = fs::remove_file(&slot2_path);
    let _ = fs::remove_file(&slot3_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ManualSaveEvent>();
    app.add_event::<LoadGameEvent>();
    app.add_systems(Update, (manual_save_system, load_game_system));

    app.insert_resource(MapState::default());

    // Spawn player and candle
    let _player = app
        .world_mut()
        .spawn((
            Player,
            Transform::default(),
            Health::Alive,
            Inventory {
                items: vec![],
                max_capacity: 10,
            },
        ))
        .id();
    app.world_mut().spawn((
        Candle,
        CandleWax(100.0),
        CandleState::Unlit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Act: Create save in slot 1 (room 0, 0 deaths)
    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: Duration::from_secs(0),
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });
    app.world_mut().send_event(ManualSaveEvent { slot: 1 });
    app.update();

    // Act: Create save in slot 2 (room 1, 5 deaths)
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 1;
        game_state.deaths = 5;
    }
    app.world_mut().send_event(ManualSaveEvent { slot: 2 });
    app.update();

    // Act: Create save in slot 3 (room 2, 10 deaths)
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.current_room = 2;
        game_state.deaths = 10;
    }
    app.world_mut().send_event(ManualSaveEvent { slot: 3 });
    app.update();

    // Assert: All three save files exist
    assert!(slot1_path.exists(), "Slot 1 save should exist");
    assert!(slot2_path.exists(), "Slot 2 save should exist");
    assert!(slot3_path.exists(), "Slot 3 save should exist");

    // Act: Load slot 2 specifically
    app.world_mut().send_event(LoadGameEvent { slot: 2 });
    app.update();

    // Assert: Slot 2 state loaded correctly
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(game_state.current_room, 1, "Should load room 1 from slot 2");
        assert_eq!(game_state.deaths, 5, "Should load 5 deaths from slot 2");
    }

    // Cleanup
    let _ = fs::remove_file(&slot1_path);
    let _ = fs::remove_file(&slot2_path);
    let _ = fs::remove_file(&slot3_path);
}

#[test]
fn save_version_compatibility() {
    // Verifies save file includes version for future compatibility

    let save_path = get_save_path(10);
    let _ = fs::remove_file(&save_path);

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ManualSaveEvent>();
    app.add_systems(Update, manual_save_system);

    app.insert_resource(GameState::default());
    app.insert_resource(MapState::default());

    // Spawn minimal entities
    app.world_mut().spawn((
        Player,
        Transform::default(),
        Health::Alive,
        Inventory {
            items: vec![],
            max_capacity: 10,
        },
    ));
    app.world_mut().spawn((
        Candle,
        CandleWax(100.0),
        CandleState::Unlit,
        BurnRate(1.0),
        VisibilityRadius(7.0),
    ));

    // Act: Save game
    app.world_mut().send_event(ManualSaveEvent { slot: 10 });
    app.update();

    // Assert: Parse save file and check version
    assert!(save_path.exists(), "Save file should exist");

    let content = fs::read_to_string(&save_path).expect("Failed to read save file");

    // Parse as RON
    let save_data: SaveData = ron::from_str(&content).expect("Failed to deserialize save file");

    // Assert: Version is 1
    assert_eq!(save_data.version, 1, "Save file version should be 1");

    // Verify version field in raw content too
    assert!(
        content.contains("version: 1"),
        "Save file should explicitly contain version field"
    );

    // Cleanup
    let _ = fs::remove_file(&save_path);
}
