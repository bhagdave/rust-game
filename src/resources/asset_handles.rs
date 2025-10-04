use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use crate::components::inventory::KeyType;
use crate::components::trap::Trap;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub sprites: HashMap<SpriteType, Handle<Image>>,
    pub audio: HashMap<SoundType, Handle<AudioSource>>,
    pub fonts: HashMap<FontType, Handle<Font>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum SpriteType {
    Player,
    Candle,
    Match,
    Key(KeyType),
    Trap(TrapType),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TrapType {
    Spikes,
    FallingChandelier,
    CollapsingFloor,
    Pendulum,
    ArrowTrap,
}

impl From<Trap> for TrapType {
    fn from(trap: Trap) -> Self {
        match trap {
            Trap::Spikes => TrapType::Spikes,
            Trap::FallingChandelier => TrapType::FallingChandelier,
            Trap::CollapsingFloor => TrapType::CollapsingFloor,
            Trap::Pendulum => TrapType::Pendulum,
            Trap::ArrowTrap => TrapType::ArrowTrap,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum SoundType {
    MatchStrike,
    CandleExtinguish,
    DoorCreak,
    TrapTrigger,
    ItemPickup,
    PlayerDeath,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum FontType {
    UI,
    Title,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert_asset_handles_as_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Insert AssetHandles resource
        app.insert_resource(AssetHandles::default());

        // Verify resource exists
        let handles = app.world().get_resource::<AssetHandles>();
        assert!(handles.is_some());

        let handles = handles.unwrap();
        assert_eq!(handles.sprites.len(), 0);
        assert_eq!(handles.audio.len(), 0);
        assert_eq!(handles.fonts.len(), 0);
    }

    #[test]
    fn can_store_sprite_handles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        // Create mock handles
        let player_handle: Handle<Image> = Handle::default();
        let candle_handle: Handle<Image> = Handle::default();

        // Store handles
        {
            let mut handles = app.world_mut().resource_mut::<AssetHandles>();
            handles.sprites.insert(SpriteType::Player, player_handle.clone());
            handles.sprites.insert(SpriteType::Candle, candle_handle.clone());
        }

        // Verify retrieval
        let handles = app.world().resource::<AssetHandles>();
        assert_eq!(handles.sprites.len(), 2);
        assert!(handles.sprites.contains_key(&SpriteType::Player));
        assert!(handles.sprites.contains_key(&SpriteType::Candle));
    }

    #[test]
    fn can_store_audio_handles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let match_sound: Handle<AudioSource> = Handle::default();
        let death_sound: Handle<AudioSource> = Handle::default();

        {
            let mut handles = app.world_mut().resource_mut::<AssetHandles>();
            handles.audio.insert(SoundType::MatchStrike, match_sound.clone());
            handles.audio.insert(SoundType::PlayerDeath, death_sound.clone());
        }

        let handles = app.world().resource::<AssetHandles>();
        assert_eq!(handles.audio.len(), 2);
        assert!(handles.audio.contains_key(&SoundType::MatchStrike));
        assert!(handles.audio.contains_key(&SoundType::PlayerDeath));
    }

    #[test]
    fn can_store_font_handles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let ui_font: Handle<Font> = Handle::default();
        let title_font: Handle<Font> = Handle::default();

        {
            let mut handles = app.world_mut().resource_mut::<AssetHandles>();
            handles.fonts.insert(FontType::UI, ui_font.clone());
            handles.fonts.insert(FontType::Title, title_font.clone());
        }

        let handles = app.world().resource::<AssetHandles>();
        assert_eq!(handles.fonts.len(), 2);
        assert!(handles.fonts.contains_key(&FontType::UI));
        assert!(handles.fonts.contains_key(&FontType::Title));
    }

    #[test]
    fn sprite_type_with_key_variants() {
        // Test all key variants
        let brass_key = SpriteType::Key(KeyType::Brass);
        let iron_key = SpriteType::Key(KeyType::Iron);
        let ornate_key = SpriteType::Key(KeyType::Ornate);
        let master_key = SpriteType::Key(KeyType::Master);

        assert_ne!(brass_key, iron_key);
        assert_ne!(iron_key, ornate_key);
        assert_ne!(ornate_key, master_key);
        assert_eq!(brass_key, SpriteType::Key(KeyType::Brass));
    }

    #[test]
    fn sprite_type_with_trap_variants() {
        let spikes = SpriteType::Trap(TrapType::Spikes);
        let chandelier = SpriteType::Trap(TrapType::FallingChandelier);
        let floor = SpriteType::Trap(TrapType::CollapsingFloor);
        let pendulum = SpriteType::Trap(TrapType::Pendulum);
        let arrow = SpriteType::Trap(TrapType::ArrowTrap);

        assert_ne!(spikes, chandelier);
        assert_ne!(chandelier, floor);
        assert_ne!(floor, pendulum);
        assert_ne!(pendulum, arrow);
        assert_eq!(spikes, SpriteType::Trap(TrapType::Spikes));
    }

    #[test]
    fn trap_type_conversion() {
        let spikes_trap = Trap::Spikes;
        let spikes_type: TrapType = spikes_trap.into();
        assert_eq!(spikes_type, TrapType::Spikes);

        let chandelier_trap = Trap::FallingChandelier;
        let chandelier_type: TrapType = chandelier_trap.into();
        assert_eq!(chandelier_type, TrapType::FallingChandelier);
    }

    #[test]
    fn sound_type_variants() {
        let sounds = vec![
            SoundType::MatchStrike,
            SoundType::CandleExtinguish,
            SoundType::DoorCreak,
            SoundType::TrapTrigger,
            SoundType::ItemPickup,
            SoundType::PlayerDeath,
        ];

        assert_eq!(sounds.len(), 6);
        assert_ne!(sounds[0], sounds[1]);
        assert_eq!(SoundType::MatchStrike, SoundType::MatchStrike);
    }

    #[test]
    fn font_type_variants() {
        assert_eq!(FontType::UI, FontType::UI);
        assert_eq!(FontType::Title, FontType::Title);
        assert_ne!(FontType::UI, FontType::Title);
    }

    #[test]
    fn can_retrieve_specific_handles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        let player_handle: Handle<Image> = Handle::default();

        {
            let mut handles = app.world_mut().resource_mut::<AssetHandles>();
            handles.sprites.insert(SpriteType::Player, player_handle.clone());
        }

        let handles = app.world().resource::<AssetHandles>();
        let retrieved = handles.sprites.get(&SpriteType::Player);
        assert!(retrieved.is_some());
    }

    #[test]
    fn can_use_in_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(AssetHandles::default());

        fn test_system(handles: Res<AssetHandles>) {
            // Just verify we can access the resource
            assert_eq!(handles.sprites.len(), 0);
        }

        app.add_systems(Update, test_system);
        app.update();
    }

    #[test]
    fn hash_map_key_equality() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(SpriteType::Player);
        set.insert(SpriteType::Candle);
        set.insert(SpriteType::Player); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&SpriteType::Player));
        assert!(set.contains(&SpriteType::Candle));
    }
}
