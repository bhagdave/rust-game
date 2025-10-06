use std::path::Path;

/// Integration test: Sprite asset validation
/// From tasks.md T040: Verify placeholder sprites exist and have correct dimensions

#[test]
fn player_sprite_exists() {
    let sprite_path = Path::new("assets/sprites/player.png");
    assert!(
        sprite_path.exists(),
        "Player sprite should exist at assets/sprites/player.png"
    );
}

#[test]
fn candle_sprite_exists() {
    let sprite_path = Path::new("assets/sprites/candle.png");
    assert!(
        sprite_path.exists(),
        "Candle sprite should exist at assets/sprites/candle.png"
    );
}

#[test]
fn match_sprite_exists() {
    let sprite_path = Path::new("assets/sprites/match.png");
    assert!(
        sprite_path.exists(),
        "Match sprite should exist at assets/sprites/match.png"
    );
}

#[test]
fn key_sprite_exists() {
    let sprite_path = Path::new("assets/sprites/key.png");
    assert!(
        sprite_path.exists(),
        "Key sprite should exist at assets/sprites/key.png"
    );
}

#[test]
fn tileset_sprite_exists() {
    let sprite_path = Path::new("assets/sprites/tileset.png");
    assert!(
        sprite_path.exists(),
        "Tileset sprite should exist at assets/sprites/tileset.png"
    );
}

#[test]
fn all_required_sprites_exist() {
    let required_sprites = vec![
        "assets/sprites/player.png",
        "assets/sprites/candle.png",
        "assets/sprites/match.png",
        "assets/sprites/key.png",
        "assets/sprites/tileset.png",
    ];

    for sprite in required_sprites {
        assert!(
            Path::new(sprite).exists(),
            "Required sprite {} should exist",
            sprite
        );
    }
}

#[cfg(feature = "image-validation")]
mod dimension_tests {
    use image::GenericImageView;
    use std::path::Path;

    #[test]
    fn player_sprite_has_correct_dimensions() {
        let img = image::open("assets/sprites/player.png").expect("Should open player sprite");
        let (width, height) = img.dimensions();
        assert_eq!(width, 32, "Player sprite should be 32 pixels wide");
        assert_eq!(height, 32, "Player sprite should be 32 pixels tall");
    }

    #[test]
    fn candle_sprite_has_correct_dimensions() {
        let img = image::open("assets/sprites/candle.png").expect("Should open candle sprite");
        let (width, height) = img.dimensions();
        assert_eq!(width, 16, "Candle sprite should be 16 pixels wide");
        assert_eq!(height, 16, "Candle sprite should be 16 pixels tall");
    }

    #[test]
    fn match_sprite_has_correct_dimensions() {
        let img = image::open("assets/sprites/match.png").expect("Should open match sprite");
        let (width, height) = img.dimensions();
        assert_eq!(width, 8, "Match sprite should be 8 pixels wide");
        assert_eq!(height, 8, "Match sprite should be 8 pixels tall");
    }

    #[test]
    fn key_sprite_has_correct_dimensions() {
        let img = image::open("assets/sprites/key.png").expect("Should open key sprite");
        let (width, height) = img.dimensions();
        assert_eq!(width, 12, "Key sprite should be 12 pixels wide");
        assert_eq!(height, 12, "Key sprite should be 12 pixels tall");
    }

    #[test]
    fn tileset_sprite_has_correct_dimensions() {
        let img = image::open("assets/sprites/tileset.png").expect("Should open tileset sprite");
        let (width, height) = img.dimensions();
        // Tileset is 32x16 (2 tiles of 16x16 each, arranged horizontally)
        assert_eq!(width, 32, "Tileset should be 32 pixels wide");
        assert_eq!(height, 16, "Tileset should be 16 pixels tall");
    }

    #[test]
    fn all_sprites_are_valid_png_files() {
        let sprites = vec![
            "assets/sprites/player.png",
            "assets/sprites/candle.png",
            "assets/sprites/match.png",
            "assets/sprites/key.png",
            "assets/sprites/tileset.png",
        ];

        for sprite_path in sprites {
            let img = image::open(sprite_path);
            assert!(img.is_ok(), "{} should be a valid PNG file", sprite_path);
        }
    }
}

#[test]
fn sprites_directory_exists() {
    let sprites_dir = Path::new("assets/sprites");
    assert!(
        sprites_dir.exists(),
        "Sprites directory should exist at assets/sprites"
    );
    assert!(sprites_dir.is_dir(), "assets/sprites should be a directory");
}

#[test]
fn sprites_have_reasonable_file_sizes() {
    use std::fs;

    let sprites = vec![
        ("assets/sprites/player.png", 50, 500), // Expected range in bytes
        ("assets/sprites/candle.png", 50, 500),
        ("assets/sprites/match.png", 50, 300),
        ("assets/sprites/key.png", 50, 300),
        ("assets/sprites/tileset.png", 50, 500),
    ];

    for (sprite_path, min_size, max_size) in sprites {
        let metadata =
            fs::metadata(sprite_path).expect(&format!("Should get metadata for {}", sprite_path));
        let size = metadata.len();

        assert!(
            size >= min_size && size <= max_size,
            "{} should have reasonable file size (got {} bytes, expected {}-{} bytes)",
            sprite_path,
            size,
            min_size,
            max_size
        );
    }
}

#[test]
fn sprites_are_not_empty() {
    use std::fs;

    let sprites = vec![
        "assets/sprites/player.png",
        "assets/sprites/candle.png",
        "assets/sprites/match.png",
        "assets/sprites/key.png",
        "assets/sprites/tileset.png",
    ];

    for sprite_path in sprites {
        let metadata =
            fs::metadata(sprite_path).expect(&format!("Should get metadata for {}", sprite_path));
        let size = metadata.len();

        assert!(size > 0, "{} should not be empty", sprite_path);
    }
}
