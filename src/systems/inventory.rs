use crate::components::inventory::{Inventory, Item, StackableItem};
use bevy::prelude::*;

/// Event emitted when a player collects an item
///
/// This event is typically emitted by the collision detection system when
/// the player overlaps with a collectible entity.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use rust_game::systems::inventory::ItemCollectedEvent;
///
/// fn trigger_item_pickup(
///     mut events: EventWriter<ItemCollectedEvent>,
///     player_entity: Entity,
///     item_entity: Entity,
/// ) {
///     events.send(ItemCollectedEvent {
///         item: item_entity,
///         player: player_entity,
///     });
/// }
/// ```
#[derive(Event)]
pub struct ItemCollectedEvent {
    pub item: Entity,
    pub player: Entity,
}

/// Event emitted when a player uses an item from inventory
///
/// This event can be used to trigger item-specific behaviors
/// (e.g., using a match to light a candle).
#[derive(Event)]
pub struct ItemUsedEvent {
    pub item: Item,
    pub player: Entity,
}

/// System that handles item collection and inventory management
///
/// Listens for `ItemCollectedEvent` and:
/// - Checks inventory capacity
/// - Handles stackable items (matches) by incrementing count
/// - Adds unique items to separate slots
/// - Despawns collected items from the world
///
/// # System Dependencies
/// - **Upstream**: Requires `collision_detection_system` to emit `ItemCollectedEvent`
/// - **Components**: Reads `Item`, `StackableItem`, `Collectible`; Writes `Inventory`
/// - **Commands**: Despawns collected item entities
///
/// From tasks.md T029: InventorySystem
pub fn inventory_collection_system(
    mut events: EventReader<ItemCollectedEvent>,
    mut commands: Commands,
    mut inventory_query: Query<&mut Inventory>,
    item_query: Query<(&Item, Option<&StackableItem>)>,
) {
    for event in events.read() {
        if let Ok(mut inventory) = inventory_query.get_mut(event.player) {
            // Check capacity
            if inventory.items.len() >= inventory.max_capacity {
                // TODO: Emit InventoryFullEvent for UI notification
                continue; // Inventory full, cannot pick up
            }

            if let Ok((item, stackable)) = item_query.get(event.item) {
                // Handle stackable vs unique items
                if stackable.is_some() {
                    // For stackable items (like matches), increment count if already in inventory
                    // For now, we use simple duplication approach
                    // TODO: Implement proper stacking with HashMap<Item, count>
                    inventory.items.push(item.clone());
                } else {
                    // Unique items occupy separate slots
                    inventory.items.push(item.clone());
                }

                // Despawn item from world
                commands.entity(event.item).despawn();
            }
        }
    }
}

/// System that handles item usage from inventory
///
/// Listens for `ItemUsedEvent` and removes the consumed item from the player's inventory.
///
/// # System Dependencies
/// - **Upstream**: Input system or UI system emits `ItemUsedEvent`
/// - **Components**: Writes `Inventory`
///
/// From tasks.md T029: InventorySystem (item usage)
pub fn inventory_usage_system(
    mut events: EventReader<ItemUsedEvent>,
    mut inventory_query: Query<&mut Inventory>,
) {
    for event in events.read() {
        if let Ok(mut inventory) = inventory_query.get_mut(event.player) {
            // Find and remove the first matching item
            if let Some(pos) = inventory
                .items
                .iter()
                .position(|inv_item| match (&event.item, inv_item) {
                    (Item::Match, Item::Match) => true,
                    (Item::Key(k1), Item::Key(k2)) => k1 == k2,
                    (Item::Tool(t1), Item::Tool(t2)) => t1 == t2,
                    (Item::PuzzleItem(p1), Item::PuzzleItem(p2)) => p1 == p2,
                    (Item::DoubleJumpItem, Item::DoubleJumpItem) => true,
                    (Item::DiaryPage(n1), Item::DiaryPage(n2)) => n1 == n2,
                    _ => false,
                })
            {
                inventory.items.remove(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::inventory::{Collectible, KeyType};
    use crate::components::player::Player;

    #[test]
    fn inventory_collection_system_compiles() {
        // Verify system function signature is correct
        fn check_system<Params, S: IntoSystem<(), (), Params>>(s: S) -> S {
            s
        }

        check_system(inventory_collection_system);
    }

    #[test]
    fn inventory_usage_system_compiles() {
        fn check_system<Params, S: IntoSystem<(), (), Params>>(s: S) -> S {
            s
        }

        check_system(inventory_usage_system);
    }

    #[test]
    fn item_collected_event_can_be_sent() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<ItemCollectedEvent>();

        let player = app.world_mut().spawn(Player).id();
        let item = app.world_mut().spawn(Collectible).id();

        // Send event
        app.world_mut()
            .send_event(ItemCollectedEvent { item, player });

        // Event should be in queue
        let events = app.world().resource::<Events<ItemCollectedEvent>>();
        assert!(!events.is_empty());
    }

    #[test]
    fn item_usage_system_removes_item_from_inventory() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<ItemUsedEvent>();
        app.add_systems(Update, inventory_usage_system);

        // Spawn player with inventory
        let player = app
            .world_mut()
            .spawn((
                Player,
                Inventory {
                    items: vec![Item::Match, Item::Key(KeyType::Brass), Item::Match],
                    max_capacity: 10,
                },
            ))
            .id();

        // Send usage event for first Match
        app.world_mut().send_event(ItemUsedEvent {
            item: Item::Match,
            player,
        });

        // Run system
        app.update();

        // Verify match removed
        let inventory = app.world().get::<Inventory>(player).unwrap();
        assert_eq!(
            inventory.items.len(),
            2,
            "Should have 2 items after using 1 match"
        );
        // First match removed, second match and brass key remain
        assert!(matches!(inventory.items[0], Item::Key(KeyType::Brass)));
        assert!(matches!(inventory.items[1], Item::Match));
    }
}
