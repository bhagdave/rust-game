use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_capacity: usize,
}

#[derive(Component, Clone)]
pub enum Item {
    Match,
    Key(KeyType),
    Tool(ToolType),
    PuzzleItem(PuzzleItemType),
    DoubleJumpItem,
    DiaryPage(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum KeyType {
    Brass,
    Iron,
    Ornate,
    Master,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolType {
    Wrench,
    Crowbar,
    WireCutters,
    Magnet,
    OilCan,
    Ladder,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PuzzleItemType {
    Fuse,
    Gemstone(Color),
    CircuitComponent,
}

#[derive(Component)]
pub struct StackableItem(pub u32); // stack count

#[derive(Component)]
pub struct Collectible; // marker for pickup

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_inventory() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create inventory with items
        let inventory = Inventory {
            items: vec![
                Item::Match,
                Item::Key(KeyType::Brass),
                Item::Tool(ToolType::Wrench),
                Item::PuzzleItem(PuzzleItemType::Fuse),
                Item::DoubleJumpItem,
                Item::DiaryPage(1),
            ],
            max_capacity: 10,
        };

        // Spawn entity with inventory
        let entity = app.world_mut().spawn(inventory).id();

        // Verify inventory component exists
        let inv = app.world().get::<Inventory>(entity);
        assert!(inv.is_some());

        let inv = inv.unwrap();
        assert_eq!(inv.items.len(), 6);
        assert_eq!(inv.max_capacity, 10);
    }

    #[test]
    fn can_create_collectible_items() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create collectible item
        let entity = app
            .world_mut()
            .spawn((Item::Match, StackableItem(5), Collectible))
            .id();

        // Verify components
        let item = app.world().get::<Item>(entity);
        assert!(item.is_some());

        let stackable = app.world().get::<StackableItem>(entity);
        assert!(stackable.is_some());
        assert_eq!(stackable.unwrap().0, 5);

        let collectible = app.world().get::<Collectible>(entity);
        assert!(collectible.is_some());
    }

    #[test]
    fn item_variants_compile() {
        // Test all item variants
        let _match = Item::Match;
        let _brass_key = Item::Key(KeyType::Brass);
        let _iron_key = Item::Key(KeyType::Iron);
        let _ornate_key = Item::Key(KeyType::Ornate);
        let _master_key = Item::Key(KeyType::Master);

        let _wrench = Item::Tool(ToolType::Wrench);
        let _crowbar = Item::Tool(ToolType::Crowbar);
        let _wire_cutters = Item::Tool(ToolType::WireCutters);
        let _magnet = Item::Tool(ToolType::Magnet);
        let _oil_can = Item::Tool(ToolType::OilCan);
        let _ladder = Item::Tool(ToolType::Ladder);

        let _fuse = Item::PuzzleItem(PuzzleItemType::Fuse);
        let _gemstone = Item::PuzzleItem(PuzzleItemType::Gemstone(Color::srgb(1.0, 0.0, 0.0)));
        let _circuit = Item::PuzzleItem(PuzzleItemType::CircuitComponent);

        let _double_jump = Item::DoubleJumpItem;
        let _diary = Item::DiaryPage(0);
    }

    #[test]
    fn key_types_equality() {
        assert_eq!(KeyType::Brass, KeyType::Brass);
        assert_ne!(KeyType::Brass, KeyType::Iron);
        assert_ne!(KeyType::Iron, KeyType::Ornate);
        assert_ne!(KeyType::Ornate, KeyType::Master);
    }

    #[test]
    fn inventory_capacity() {
        let mut inventory = Inventory {
            items: vec![],
            max_capacity: 3,
        };

        // Add items
        inventory.items.push(Item::Match);
        inventory.items.push(Item::Key(KeyType::Brass));
        inventory.items.push(Item::Tool(ToolType::Wrench));

        assert_eq!(inventory.items.len(), 3);
        assert_eq!(inventory.items.len(), inventory.max_capacity);
    }
}
