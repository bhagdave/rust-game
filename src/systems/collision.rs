use crate::components::inventory::Collectible;
use crate::components::player::Player;
use crate::components::room::Collider;
use crate::components::trap::Trap;
use bevy::prelude::*;

/// System for AABB (Axis-Aligned Bounding Box) collision detection
///
/// Handles:
/// - Player vs trap collisions
/// - Player vs collectible item collisions
/// - Player vs door collisions (future work)
///
/// Currently logs collisions to console. Integration with event system
/// for TrapTriggeredEvent and ItemCollectedEvent will be added in T027 and T029.
///
/// From tasks.md T026: CollisionDetectionSystem
pub fn collision_detection_system(
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    trap_query: Query<(Entity, &Transform, &Collider), With<Trap>>,
    item_query: Query<(Entity, &Transform, &Collider), With<Collectible>>,
) {
    for (player_entity, player_transform, player_collider) in &player_query {
        let player_pos = player_transform.translation.truncate();

        // Check trap collisions
        for (trap_entity, trap_transform, trap_collider) in &trap_query {
            let trap_pos = trap_transform.translation.truncate();
            if aabb_intersects(player_pos, player_collider, trap_pos, trap_collider) {
                // TODO: Emit TrapTriggeredEvent (will be implemented in T027)
                // For now, log the collision for debugging
                debug!(
                    "Collision detected: Player {:?} hit trap {:?}",
                    player_entity, trap_entity
                );
            }
        }

        // Check item collisions
        for (item_entity, item_transform, item_collider) in &item_query {
            let item_pos = item_transform.translation.truncate();
            if aabb_intersects(player_pos, player_collider, item_pos, item_collider) {
                // TODO: Emit ItemCollectedEvent (will be implemented in T029)
                // For now, log the collision for debugging
                debug!(
                    "Collision detected: Player {:?} collected item {:?}",
                    player_entity, item_entity
                );
            }
        }
    }
}

/// AABB collision detection helper function
///
/// Tests if two axis-aligned bounding boxes intersect.
///
/// # Arguments
/// * `pos_a` - Center position of first collider
/// * `collider_a` - First collider (min/max offsets from center)
/// * `pos_b` - Center position of second collider
/// * `collider_b` - Second collider (min/max offsets from center)
///
/// # Returns
/// `true` if the bounding boxes overlap, `false` otherwise
///
/// # Algorithm
/// AABB intersection test: Two boxes intersect if they overlap on both axes.
/// For each axis, check if max_a > min_b AND min_a < max_b.
fn aabb_intersects(pos_a: Vec2, collider_a: &Collider, pos_b: Vec2, collider_b: &Collider) -> bool {
    let a_min = pos_a + collider_a.min;
    let a_max = pos_a + collider_a.max;
    let b_min = pos_b + collider_b.min;
    let b_max = pos_b + collider_b.max;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::Player;

    #[test]
    fn collision_detection_system_compiles() {
        // Verify the system function signature is correct
        fn check_system<Params, S: IntoSystem<(), (), Params>>(s: S) -> S {
            s
        }

        check_system(collision_detection_system);
    }

    #[test]
    fn aabb_intersects_detects_overlap() {
        // Two boxes overlapping
        let pos_a = Vec2::new(0.0, 0.0);
        let collider_a = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        let pos_b = Vec2::new(5.0, 5.0);
        let collider_b = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        assert!(
            aabb_intersects(pos_a, &collider_a, pos_b, &collider_b),
            "Overlapping boxes should collide"
        );
    }

    #[test]
    fn aabb_intersects_detects_no_overlap() {
        // Two boxes not overlapping
        let pos_a = Vec2::new(0.0, 0.0);
        let collider_a = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        let pos_b = Vec2::new(50.0, 50.0);
        let collider_b = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        assert!(
            !aabb_intersects(pos_a, &collider_a, pos_b, &collider_b),
            "Non-overlapping boxes should not collide"
        );
    }

    #[test]
    fn aabb_intersects_edge_touching() {
        // Two boxes touching at edge (should not count as collision)
        let pos_a = Vec2::new(0.0, 0.0);
        let collider_a = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        let pos_b = Vec2::new(20.0, 0.0);
        let collider_b = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        assert!(
            !aabb_intersects(pos_a, &collider_a, pos_b, &collider_b),
            "Edge-touching boxes should not collide"
        );
    }

    #[test]
    fn aabb_intersects_corner_overlap() {
        // Two boxes overlapping at corner
        let pos_a = Vec2::new(0.0, 0.0);
        let collider_a = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        let pos_b = Vec2::new(15.0, 15.0);
        let collider_b = Collider {
            min: Vec2::new(-10.0, -10.0),
            max: Vec2::new(10.0, 10.0),
        };

        assert!(
            aabb_intersects(pos_a, &collider_a, pos_b, &collider_b),
            "Corner-overlapping boxes should collide"
        );
    }

    #[test]
    fn aabb_intersects_one_inside_other() {
        // Small box completely inside large box
        let pos_a = Vec2::new(0.0, 0.0);
        let collider_a = Collider {
            min: Vec2::new(-50.0, -50.0),
            max: Vec2::new(50.0, 50.0),
        };

        let pos_b = Vec2::new(5.0, 5.0);
        let collider_b = Collider {
            min: Vec2::new(-5.0, -5.0),
            max: Vec2::new(5.0, 5.0),
        };

        assert!(
            aabb_intersects(pos_a, &collider_a, pos_b, &collider_b),
            "Box inside another should collide"
        );
    }

    #[test]
    fn collision_system_with_player_and_trap() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add collision detection system
        app.add_systems(Update, collision_detection_system);

        // Spawn player with collider
        let _player = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Spawn trap with collider (overlapping with player)
        let _trap = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                Transform::from_xyz(10.0, 10.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Run one update - collision should be detected and logged
        app.update();

        // No assertion here - this test verifies the system runs without panicking
        // Once event system is added in T027, we can assert events are emitted
    }

    #[test]
    fn collision_system_with_player_and_item() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.add_systems(Update, collision_detection_system);

        // Spawn player with collider
        let _player = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(100.0, 100.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Spawn collectible item with collider (overlapping with player)
        let _item = app
            .world_mut()
            .spawn((
                Collectible,
                Transform::from_xyz(105.0, 105.0, 0.0),
                Collider {
                    min: Vec2::new(-8.0, -8.0),
                    max: Vec2::new(8.0, 8.0),
                },
            ))
            .id();

        // Run one update - collision should be detected and logged
        app.update();

        // No assertion here - this test verifies the system runs without panicking
        // Once event system is added in T029, we can assert events are emitted
    }

    #[test]
    fn collision_system_no_collision() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.add_systems(Update, collision_detection_system);

        // Spawn player with collider
        let _player = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Spawn trap with collider (far away from player)
        let _trap = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                Transform::from_xyz(1000.0, 1000.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Run one update - no collision should occur
        app.update();

        // No assertion here - this test verifies the system runs without panicking
    }

    #[test]
    fn collision_system_multiple_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.add_systems(Update, collision_detection_system);

        // Spawn player
        let _player = app
            .world_mut()
            .spawn((
                Player,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Spawn multiple traps - some colliding, some not
        let _trap1 = app
            .world_mut()
            .spawn((
                Trap::Spikes,
                Transform::from_xyz(10.0, 10.0, 0.0), // Colliding
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        let _trap2 = app
            .world_mut()
            .spawn((
                Trap::Pendulum,
                Transform::from_xyz(500.0, 500.0, 0.0), // Not colliding
                Collider {
                    min: Vec2::new(-16.0, -16.0),
                    max: Vec2::new(16.0, 16.0),
                },
            ))
            .id();

        // Spawn multiple items - some colliding, some not
        let _item1 = app
            .world_mut()
            .spawn((
                Collectible,
                Transform::from_xyz(5.0, 5.0, 0.0), // Colliding
                Collider {
                    min: Vec2::new(-8.0, -8.0),
                    max: Vec2::new(8.0, 8.0),
                },
            ))
            .id();

        let _item2 = app
            .world_mut()
            .spawn((
                Collectible,
                Transform::from_xyz(800.0, 800.0, 0.0), // Not colliding
                Collider {
                    min: Vec2::new(-8.0, -8.0),
                    max: Vec2::new(8.0, 8.0),
                },
            ))
            .id();

        // Run one update - multiple collisions should be detected
        app.update();

        // No assertion here - this test verifies the system runs without panicking
    }
}
