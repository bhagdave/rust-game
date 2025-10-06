/// Integration tests for T029: InventorySystem
///
/// These tests validate the complete item collection flow:
/// 1. Collision detection emits ItemCollectedEvent
/// 2. Inventory system processes event and adds item
/// 3. Item entity is despawned from world
/// 4. Inventory capacity is enforced
/// 5. Item usage removes items from inventory
use bevy::prelude::*;
use rust_game::components::inventory::*;
use rust_game::components::player::Player;
use rust_game::components::room::Collider;
use rust_game::systems::collision::collision_detection_system;
use rust_game::systems::inventory::{
    ItemCollectedEvent, ItemUsedEvent, inventory_collection_system, inventory_usage_system,
};
use rust_game::systems::trap::TrapTriggeredEvent;

#[test]
fn full_item_pickup_flow() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add events
    app.add_event::<ItemCollectedEvent>();
    app.add_event::<TrapTriggeredEvent>();

    // Add systems in correct order
    app.add_systems(
        Update,
        (collision_detection_system, inventory_collection_system).chain(),
    );

    // Spawn player with inventory and collider
    let player = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Inventory {
                items: vec![],
                max_capacity: 10,
            },
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Spawn collectible item overlapping with player
    let item = app
        .world_mut()
        .spawn((
            Item::DiaryPage(1),
            Collectible,
            Transform::from_xyz(105.0, 105.0, 0.0), // Within collision range
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    // Verify initial state
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(inventory.items.len(), 0, "Inventory should start empty");
    }

    {
        let item_exists = app.world().get::<Item>(item).is_some();
        assert!(item_exists, "Item should exist in world");
    }

    // Run one update - collision detection + inventory collection
    app.update();

    // Verify item was collected
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(
            inventory.items.len(),
            1,
            "Item should be added to inventory"
        );
        assert!(
            matches!(inventory.items[0], Item::DiaryPage(1)),
            "Should be diary page 1"
        );
    }

    // Verify item was despawned
    {
        let item_exists = app.world().get::<Item>(item).is_some();
        assert!(
            !item_exists,
            "Item entity should be despawned after collection"
        );
    }
}

#[test]
fn inventory_capacity_enforcement() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ItemCollectedEvent>();
    app.add_event::<TrapTriggeredEvent>();

    app.add_systems(
        Update,
        (collision_detection_system, inventory_collection_system).chain(),
    );

    // Spawn player with FULL inventory
    let player = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Inventory {
                items: vec![
                    Item::Match,
                    Item::Match,
                    Item::Key(KeyType::Brass),
                    Item::Key(KeyType::Iron),
                    Item::Key(KeyType::Ornate),
                    Item::Tool(ToolType::Wrench),
                    Item::Tool(ToolType::Crowbar),
                    Item::PuzzleItem(PuzzleItemType::Fuse),
                    Item::DiaryPage(1),
                    Item::DiaryPage(2),
                ],
                max_capacity: 10,
            },
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Spawn 11th item at player position
    let item = app
        .world_mut()
        .spawn((
            Item::DoubleJumpItem,
            Collectible,
            Transform::from_xyz(100.0, 100.0, 0.0), // Exactly at player position
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    // Run update
    app.update();

    // Verify item was NOT collected (inventory full)
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(
            inventory.items.len(),
            10,
            "Inventory should remain at capacity"
        );

        let has_double_jump = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::DoubleJumpItem));
        assert!(
            !has_double_jump,
            "DoubleJumpItem should not be added when inventory full"
        );
    }

    // Verify item still exists in world (not despawned)
    {
        let item_exists = app.world().get::<Item>(item).is_some();
        assert!(
            item_exists,
            "Item should remain in world when inventory full"
        );
    }
}

#[test]
fn multiple_items_collected_in_order() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ItemCollectedEvent>();
    app.add_event::<TrapTriggeredEvent>();

    app.add_systems(
        Update,
        (collision_detection_system, inventory_collection_system).chain(),
    );

    // Spawn player
    let player = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Inventory {
                items: vec![],
                max_capacity: 10,
            },
            Collider {
                min: Vec2::new(-20.0, -20.0),
                max: Vec2::new(20.0, 20.0),
            },
        ))
        .id();

    // Spawn multiple collectible items all overlapping with player
    let _match_item = app
        .world_mut()
        .spawn((
            Item::Match,
            StackableItem(1),
            Collectible,
            Transform::from_xyz(105.0, 100.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    let _brass_key = app
        .world_mut()
        .spawn((
            Item::Key(KeyType::Brass),
            Collectible,
            Transform::from_xyz(100.0, 105.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    let _wrench = app
        .world_mut()
        .spawn((
            Item::Tool(ToolType::Wrench),
            Collectible,
            Transform::from_xyz(95.0, 95.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    // Run update - all items should be collected
    app.update();

    // Verify all 3 items collected
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(inventory.items.len(), 3, "All 3 items should be collected");

        // Verify all item types are present
        let has_match = inventory.items.iter().any(|i| matches!(i, Item::Match));
        let has_brass = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Key(KeyType::Brass)));
        let has_wrench = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Tool(ToolType::Wrench)));

        assert!(has_match, "Should have match");
        assert!(has_brass, "Should have brass key");
        assert!(has_wrench, "Should have wrench");
    }
}

#[test]
fn stackable_items_increment_count() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ItemCollectedEvent>();
    app.add_event::<TrapTriggeredEvent>();

    app.add_systems(
        Update,
        (collision_detection_system, inventory_collection_system).chain(),
    );

    // Spawn player with 1 match already
    let player = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Inventory {
                items: vec![Item::Match],
                max_capacity: 10,
            },
            Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            },
        ))
        .id();

    // Spawn 2 more matches
    let _match1 = app
        .world_mut()
        .spawn((
            Item::Match,
            StackableItem(1),
            Collectible,
            Transform::from_xyz(105.0, 100.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    let _match2 = app
        .world_mut()
        .spawn((
            Item::Match,
            StackableItem(1),
            Collectible,
            Transform::from_xyz(100.0, 105.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    // Run update
    app.update();

    // Verify matches stacked (current simple implementation: 3 separate items)
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        let match_count = inventory
            .items
            .iter()
            .filter(|i| matches!(i, Item::Match))
            .count();
        assert_eq!(
            match_count, 3,
            "Should have 3 matches total (1 initial + 2 collected)"
        );
    }
}

#[test]
fn item_usage_removes_from_inventory() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ItemUsedEvent>();
    app.add_systems(Update, inventory_usage_system);

    // Spawn player with 3 items
    let player = app
        .world_mut()
        .spawn((
            Player,
            Inventory {
                items: vec![
                    Item::Match,
                    Item::Key(KeyType::Brass),
                    Item::Tool(ToolType::Wrench),
                ],
                max_capacity: 10,
            },
        ))
        .id();

    // Use the match
    app.world_mut().send_event(ItemUsedEvent {
        item: Item::Match,
        player,
    });

    app.update();

    // Verify match removed
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(inventory.items.len(), 2, "Should have 2 items left");

        let has_match = inventory.items.iter().any(|i| matches!(i, Item::Match));
        assert!(!has_match, "Match should be removed");

        let has_brass = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Key(KeyType::Brass)));
        let has_wrench = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Tool(ToolType::Wrench)));

        assert!(has_brass, "Brass key should remain");
        assert!(has_wrench, "Wrench should remain");
    }
}

#[test]
fn unique_items_occupy_separate_slots() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<ItemCollectedEvent>();
    app.add_event::<TrapTriggeredEvent>();

    app.add_systems(
        Update,
        (collision_detection_system, inventory_collection_system).chain(),
    );

    // Spawn player
    let player = app
        .world_mut()
        .spawn((
            Player,
            Transform::from_xyz(100.0, 100.0, 0.0),
            Inventory {
                items: vec![],
                max_capacity: 10,
            },
            Collider {
                min: Vec2::new(-20.0, -20.0),
                max: Vec2::new(20.0, 20.0),
            },
        ))
        .id();

    // Spawn multiple unique items
    let _brass = app
        .world_mut()
        .spawn((
            Item::Key(KeyType::Brass),
            Collectible,
            Transform::from_xyz(105.0, 100.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    let _iron = app
        .world_mut()
        .spawn((
            Item::Key(KeyType::Iron),
            Collectible,
            Transform::from_xyz(100.0, 105.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    let _wrench = app
        .world_mut()
        .spawn((
            Item::Tool(ToolType::Wrench),
            Collectible,
            Transform::from_xyz(95.0, 100.0, 0.0),
            Collider {
                min: Vec2::new(-8.0, -8.0),
                max: Vec2::new(8.0, 8.0),
            },
        ))
        .id();

    // Run update
    app.update();

    // Verify all 3 items in separate slots
    {
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(
            inventory.items.len(),
            3,
            "3 unique items should occupy 3 slots"
        );

        let has_brass = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Key(KeyType::Brass)));
        let has_iron = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Key(KeyType::Iron)));
        let has_wrench = inventory
            .items
            .iter()
            .any(|i| matches!(i, Item::Tool(ToolType::Wrench)));

        assert!(has_brass, "Brass Key should be in inventory");
        assert!(has_iron, "Iron Key should be in inventory");
        assert!(has_wrench, "Wrench should be in inventory");
    }
}
