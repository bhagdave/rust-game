use bevy::prelude::*;

/// Marker component for entities spawned by the demo level.
///
/// This component is attached to all entities created during the demo level
/// to enable easy cleanup and identification. When the demo level ends or
/// the player transitions to another game mode, all entities with this marker
/// can be despawned in a single query.
///
/// # Example
/// ```ignore
/// commands.spawn((
///     SpriteBundle { /* ... */ },
///     DemoMarker,
///     // ... other components
/// ));
/// ```
///
/// # Cleanup
/// ```ignore
/// fn cleanup_demo(mut commands: Commands, demo_entities: Query<Entity, With<DemoMarker>>) {
///     for entity in demo_entities.iter() {
///         commands.entity(entity).despawn_recursive();
///     }
/// }
/// ```
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct DemoMarker;

/// Component for objects the player can interact with in the demo level.
///
/// Stores metadata about interactive objects like doors, items, and pickups
/// that the player can engage with during the demo.
///
/// # Example
/// ```ignore
/// commands.spawn((
///     SpriteBundle { /* ... */ },
///     InteractableDemo {
///         object_id: "door_01".to_string(),
///         interaction_prompt: "Press E to open".to_string(),
///     },
///     DemoMarker,
/// ));
/// ```
#[derive(Component, Debug, Clone)]
pub struct InteractableDemo {
    /// Unique identifier for this interactive object
    pub object_id: String,
    /// UI text displayed when player is near (e.g., "Press E to open")
    pub interaction_prompt: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_marker_is_component() {
        // Test that DemoMarker derives Component
        let marker = DemoMarker;

        // Should implement Copy
        let marker2 = marker;
        let _marker3 = marker2;

        // Should implement Default
        let _default_marker = DemoMarker;
    }

    #[test]
    fn demo_marker_can_be_added_to_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let entity = app.world_mut().spawn(DemoMarker).id();

        // Verify entity has DemoMarker component
        let has_marker = app.world().get::<DemoMarker>(entity).is_some();
        assert!(has_marker, "Entity should have DemoMarker component");
    }

    #[test]
    fn interactable_demo_has_required_fields() {
        let interactable = InteractableDemo {
            object_id: "test_door".to_string(),
            interaction_prompt: "Press E".to_string(),
        };

        assert_eq!(interactable.object_id, "test_door");
        assert_eq!(interactable.interaction_prompt, "Press E");
    }

    #[test]
    fn interactable_demo_can_be_cloned() {
        let interactable = InteractableDemo {
            object_id: "door_01".to_string(),
            interaction_prompt: "Press E to open".to_string(),
        };

        let cloned = interactable.clone();
        assert_eq!(cloned.object_id, interactable.object_id);
        assert_eq!(cloned.interaction_prompt, interactable.interaction_prompt);
    }

    #[test]
    fn interactable_demo_can_be_added_to_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let interactable = InteractableDemo {
            object_id: "key_brass".to_string(),
            interaction_prompt: "Press E to collect".to_string(),
        };

        let entity = app.world_mut().spawn(interactable.clone()).id();

        // Verify entity has InteractableDemo component
        let has_component = app.world().get::<InteractableDemo>(entity).is_some();
        assert!(
            has_component,
            "Entity should have InteractableDemo component"
        );

        // Verify the component data
        let component = app.world().get::<InteractableDemo>(entity).unwrap();
        assert_eq!(component.object_id, "key_brass");
        assert_eq!(component.interaction_prompt, "Press E to collect");
    }

    #[test]
    fn demo_marker_query_filters_correctly() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn entities with and without DemoMarker
        let e1 = app.world_mut().spawn(DemoMarker).id();
        let e2 = app.world_mut().spawn(DemoMarker).id();
        let _e3 = app.world_mut().spawn(()).id(); // Entity without marker

        // Verify first two entities have DemoMarker
        assert!(app.world().get::<DemoMarker>(e1).is_some());
        assert!(app.world().get::<DemoMarker>(e2).is_some());

        // Count entities with DemoMarker using world_mut
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<DemoMarker>>();
        let count = query.iter(world).count();

        assert_eq!(count, 2, "Should find exactly 2 entities with DemoMarker");
    }

    #[test]
    fn can_query_both_components_together() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let interactable = InteractableDemo {
            object_id: "door_demo".to_string(),
            interaction_prompt: "Press E".to_string(),
        };

        // Spawn entity with both components
        let entity = app
            .world_mut()
            .spawn((DemoMarker, interactable.clone()))
            .id();

        // Verify entity has both components
        assert!(app.world().get::<DemoMarker>(entity).is_some());
        assert!(app.world().get::<InteractableDemo>(entity).is_some());

        // Count entities with both components using world_mut
        let world = app.world_mut();
        let mut query =
            world.query_filtered::<Entity, (With<DemoMarker>, With<InteractableDemo>)>();
        let count = query.iter(world).count();

        assert_eq!(
            count, 1,
            "Should find exactly 1 entity with both components"
        );
    }
}
