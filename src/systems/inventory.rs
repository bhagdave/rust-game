use crate::components::inventory::{Inventory, Item, StackableItem};
use bevy::prelude::*;

#[derive(Event)]
pub struct ItemCollectedEvent {
    pub item: Entity,
    pub player: Entity,
}

#[derive(Event)]
pub struct ItemUsedEvent {
    pub item: Item,
    pub player: Entity,
}

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
                continue; // Inventory full
            }

            if let Ok((item, stackable)) = item_query.get(event.item) {
                // Handle stackable vs unique items
                if stackable.is_some() {
                    // TODO: Implement stacking; for now, push another instance
                    inventory.items.push(item.clone());
                } else {
                    inventory.items.push(item.clone());
                }

                // Despawn item from world
                commands.entity(event.item).despawn();
            }
        }
    }
}
