use crate::components::room::RoomId;
use bevy::prelude::*;
use std::collections::HashMap;

/// Resource for tracking explored rooms and their layout data
#[derive(Resource, Default)]
pub struct MapState {
    pub explored_rooms: HashMap<RoomId, ExploredStatus>,
}

/// Status of a room's exploration
pub struct ExploredStatus {
    pub visited: bool,
    pub layout_data: Option<Vec<Vec<TileType>>>, // 2D grid
}

/// Types of tiles that can be displayed on the map
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Trap,
    Item,
}

impl MapState {
    /// Mark a room as explored
    pub fn mark_explored(&mut self, room_id: RoomId) {
        self.explored_rooms
            .entry(room_id)
            .or_insert(ExploredStatus {
                visited: true,
                layout_data: None,
            })
            .visited = true;
    }

    /// Check if a room has been visited
    pub fn is_visited(&self, room_id: RoomId) -> bool {
        self.explored_rooms
            .get(&room_id)
            .is_some_and(|status| status.visited)
    }

    /// Set the layout data for a room
    pub fn set_layout(&mut self, room_id: RoomId, layout: Vec<Vec<TileType>>) {
        self.explored_rooms
            .entry(room_id)
            .or_insert(ExploredStatus {
                visited: true,
                layout_data: None,
            })
            .layout_data = Some(layout);
    }

    /// Get the layout data for a room
    pub fn get_layout(&self, room_id: RoomId) -> Option<&Vec<Vec<TileType>>> {
        self.explored_rooms
            .get(&room_id)
            .and_then(|status| status.layout_data.as_ref())
    }

    /// Get the count of explored rooms
    pub fn explored_count(&self) -> usize {
        self.explored_rooms
            .values()
            .filter(|status| status.visited)
            .count()
    }

    /// Clear all exploration data
    pub fn clear(&mut self) {
        self.explored_rooms.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert_map_state_as_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Insert MapState as resource
        app.insert_resource(MapState::default());

        // Verify it can be accessed
        let map_state = app.world().get_resource::<MapState>();
        assert!(map_state.is_some());
    }

    #[test]
    fn can_mark_room_explored() {
        let mut map_state = MapState::default();

        // Initially, room should not be explored
        assert!(!map_state.is_visited(0));

        // Mark room as explored
        map_state.mark_explored(0);

        // Now room should be explored
        assert!(map_state.is_visited(0));
    }

    #[test]
    fn can_track_multiple_rooms() {
        let mut map_state = MapState::default();

        // Mark several rooms as explored
        map_state.mark_explored(0);
        map_state.mark_explored(1);
        map_state.mark_explored(5);

        // Verify all are marked
        assert!(map_state.is_visited(0));
        assert!(map_state.is_visited(1));
        assert!(map_state.is_visited(5));

        // Verify unexplored room returns false
        assert!(!map_state.is_visited(2));
    }

    #[test]
    fn can_set_and_get_layout_data() {
        let mut map_state = MapState::default();

        // Create a simple 2x2 layout
        let layout = vec![
            vec![TileType::Floor, TileType::Wall],
            vec![TileType::Floor, TileType::Door],
        ];

        // Set layout for room 0
        map_state.set_layout(0, layout.clone());

        // Retrieve layout
        let retrieved = map_state.get_layout(0);
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0][0], TileType::Floor);
        assert_eq!(retrieved[0][1], TileType::Wall);
        assert_eq!(retrieved[1][0], TileType::Floor);
        assert_eq!(retrieved[1][1], TileType::Door);
    }

    #[test]
    fn get_layout_returns_none_for_unexplored_room() {
        let map_state = MapState::default();

        // Room 0 has never been explored
        let layout = map_state.get_layout(0);
        assert!(layout.is_none());
    }

    #[test]
    fn explored_count_tracks_visited_rooms() {
        let mut map_state = MapState::default();

        assert_eq!(map_state.explored_count(), 0);

        map_state.mark_explored(0);
        assert_eq!(map_state.explored_count(), 1);

        map_state.mark_explored(1);
        assert_eq!(map_state.explored_count(), 2);

        map_state.mark_explored(2);
        assert_eq!(map_state.explored_count(), 3);

        // Marking the same room again shouldn't increase count
        map_state.mark_explored(1);
        assert_eq!(map_state.explored_count(), 3);
    }

    #[test]
    fn clear_removes_all_exploration_data() {
        let mut map_state = MapState::default();

        // Add some exploration data
        map_state.mark_explored(0);
        map_state.mark_explored(1);
        assert_eq!(map_state.explored_count(), 2);

        // Clear it
        map_state.clear();
        assert_eq!(map_state.explored_count(), 0);
        assert!(!map_state.is_visited(0));
        assert!(!map_state.is_visited(1));
    }

    #[test]
    fn can_access_map_state_in_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(MapState::default());

        // Define a system that accesses MapState
        fn test_system(mut map_state: ResMut<MapState>) {
            map_state.mark_explored(0);
        }

        app.add_systems(Update, test_system);
        app.update();

        // Verify the system ran and modified the resource
        let map_state = app.world().get_resource::<MapState>().unwrap();
        assert!(map_state.is_visited(0));
    }

    #[test]
    fn tile_type_equality() {
        assert_eq!(TileType::Floor, TileType::Floor);
        assert_eq!(TileType::Wall, TileType::Wall);
        assert_ne!(TileType::Floor, TileType::Wall);
        assert_ne!(TileType::Door, TileType::Trap);
    }

    #[test]
    fn tile_type_debug_format() {
        let floor = TileType::Floor;
        let debug_str = format!("{:?}", floor);
        assert_eq!(debug_str, "Floor");

        let wall = TileType::Wall;
        let debug_str = format!("{:?}", wall);
        assert_eq!(debug_str, "Wall");
    }

    #[test]
    fn tile_type_can_be_cloned() {
        let original = TileType::Door;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn setting_layout_marks_room_as_visited() {
        let mut map_state = MapState::default();

        let layout = vec![vec![TileType::Floor]];

        // Room not visited yet
        assert!(!map_state.is_visited(0));

        // Set layout
        map_state.set_layout(0, layout);

        // Now room should be marked as visited
        assert!(map_state.is_visited(0));
    }
}
