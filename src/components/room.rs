use bevy::prelude::*;
use crate::components::inventory::KeyType;

pub type RoomId = usize;

#[derive(Component)]
pub struct Room {
    pub id: RoomId,
    pub floor: Floor,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Floor {
    Ground,
    First,
    Second,
    Basement,
}

#[derive(Component)]
pub struct RoomBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Component)]
pub struct RoomConnections(pub Vec<RoomConnection>);

#[derive(Clone)]
pub struct RoomConnection {
    pub target_room: RoomId,
    pub connection_type: ConnectionType,
    pub position: Vec2,
    pub locked: Option<KeyType>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionType {
    Door,
    Staircase,
    Ladder,
    Hidden,
}

#[derive(Component)]
pub struct Explored(pub bool);

#[derive(Component)]
pub struct Collider {
    pub min: Vec2,
    pub max: Vec2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_room_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn room entity with components
        let entity = app
            .world_mut()
            .spawn((
                Room {
                    id: 0,
                    floor: Floor::Ground,
                    name: "Entry Hall".to_string(),
                },
                RoomBounds {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(1920.0, 1080.0),
                },
                Explored(false),
            ))
            .id();

        // Verify room components
        let room = app.world().get::<Room>(entity);
        assert!(room.is_some());
        let room = room.unwrap();
        assert_eq!(room.id, 0);
        assert_eq!(room.floor, Floor::Ground);
        assert_eq!(room.name, "Entry Hall");

        let bounds = app.world().get::<RoomBounds>(entity);
        assert!(bounds.is_some());
        let bounds = bounds.unwrap();
        assert_eq!(bounds.min, Vec2::new(0.0, 0.0));
        assert_eq!(bounds.max, Vec2::new(1920.0, 1080.0));

        let explored = app.world().get::<Explored>(entity);
        assert!(explored.is_some());
        assert_eq!(explored.unwrap().0, false);
    }

    #[test]
    fn room_connections_can_be_created() {
        let connection = RoomConnection {
            target_room: 1,
            connection_type: ConnectionType::Door,
            position: Vec2::new(1800.0, 500.0),
            locked: Some(KeyType::Brass),
        };

        assert_eq!(connection.target_room, 1);
        assert_eq!(connection.connection_type, ConnectionType::Door);
        assert_eq!(connection.locked, Some(KeyType::Brass));

        // Test cloning
        let cloned = connection.clone();
        assert_eq!(cloned.target_room, connection.target_room);
    }

    #[test]
    fn floor_enum_comparisons() {
        assert_eq!(Floor::Ground, Floor::Ground);
        assert_ne!(Floor::Ground, Floor::First);
        assert_ne!(Floor::First, Floor::Second);
        assert_ne!(Floor::Second, Floor::Basement);
    }

    #[test]
    fn connection_type_comparisons() {
        assert_eq!(ConnectionType::Door, ConnectionType::Door);
        assert_ne!(ConnectionType::Door, ConnectionType::Staircase);
        assert_ne!(ConnectionType::Ladder, ConnectionType::Hidden);
    }

    #[test]
    fn collider_aabb_bounds() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let entity = app
            .world_mut()
            .spawn(Collider {
                min: Vec2::new(-16.0, -16.0),
                max: Vec2::new(16.0, 16.0),
            })
            .id();

        let collider = app.world().get::<Collider>(entity);
        assert!(collider.is_some());
        let collider = collider.unwrap();
        assert_eq!(collider.min, Vec2::new(-16.0, -16.0));
        assert_eq!(collider.max, Vec2::new(16.0, 16.0));
    }

    #[test]
    fn room_connections_component() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let connections = vec![
            RoomConnection {
                target_room: 1,
                connection_type: ConnectionType::Door,
                position: Vec2::new(1800.0, 500.0),
                locked: Some(KeyType::Brass),
            },
            RoomConnection {
                target_room: 2,
                connection_type: ConnectionType::Staircase,
                position: Vec2::new(960.0, 100.0),
                locked: None,
            },
        ];

        let entity = app
            .world_mut()
            .spawn(RoomConnections(connections.clone()))
            .id();

        let room_connections = app.world().get::<RoomConnections>(entity);
        assert!(room_connections.is_some());
        assert_eq!(room_connections.unwrap().0.len(), 2);
    }
}
