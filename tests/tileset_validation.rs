/// Test suite for tileset validation
/// From tasks.md T003: Validate tileset.png structure
///
/// Tests:
/// - Tileset file exists
/// - Is valid PNG format
/// - Has correct dimensions (64x32 for 2 tiles)
/// - Contains floor and wall tiles
use std::path::Path;

#[test]
fn tileset_file_exists() {
    let tileset_path = Path::new("assets/sprites/tileset.png");
    assert!(
        tileset_path.exists(),
        "Tileset should exist at assets/sprites/tileset.png"
    );
}

#[test]
fn tileset_is_a_file() {
    let tileset_path = Path::new("assets/sprites/tileset.png");
    assert!(
        tileset_path.is_file(),
        "Tileset path should be a file, not a directory"
    );
}

#[test]
fn tileset_file_size_is_reasonable() {
    let metadata =
        std::fs::metadata("assets/sprites/tileset.png").expect("Should read tileset metadata");

    let file_size = metadata.len();

    // 64x32 RGBA PNG should be less than 10KB
    assert!(
        file_size < 10_000,
        "Tileset file size should be reasonable (< 10KB), got {} bytes",
        file_size
    );

    // But not empty
    assert!(
        file_size > 100,
        "Tileset should not be empty or corrupt, got {} bytes",
        file_size
    );
}

#[test]
fn tileset_is_not_empty() {
    let metadata =
        std::fs::metadata("assets/sprites/tileset.png").expect("Should read tileset metadata");

    let size = metadata.len();
    assert!(size > 0, "Tileset should not be empty, got {} bytes", size);
}

#[cfg(feature = "image-validation")]
mod dimension_tests {
    use image::GenericImageView;

    #[test]
    fn tileset_is_valid_png() {
        let result = image::open("assets/sprites/tileset.png");

        assert!(
            result.is_ok(),
            "Tileset should be a valid image file: {:?}",
            result.err()
        );
    }

    #[test]
    fn tileset_has_correct_dimensions() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        let (width, height) = img.dimensions();

        assert_eq!(
            width, 64,
            "Tileset width should be 64 pixels (2 tiles × 32px)"
        );
        assert_eq!(height, 32, "Tileset height should be 32 pixels");
    }

    #[test]
    fn tileset_contains_two_tiles() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        let (width, height) = img.dimensions();
        let tile_size = 32;

        let num_tiles_x = width / tile_size;
        let num_tiles_y = height / tile_size;

        assert_eq!(num_tiles_x, 2, "Tileset should contain 2 horizontal tiles");
        assert_eq!(num_tiles_y, 1, "Tileset should contain 1 vertical tile");
    }

    #[test]
    fn tileset_has_rgba_format() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        // Check that image has color type with alpha
        let color_type = img.color();
        assert!(
            matches!(
                color_type,
                image::ColorType::Rgba8 | image::ColorType::Rgba16
            ),
            "Tileset should be in RGBA format, got {:?}",
            color_type
        );
    }

    #[test]
    fn tileset_floor_tile_exists() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        // Sample pixels from tile 0 (floor tile, left half)
        // Check center of tile 0 (16, 16)
        let pixel = img.get_pixel(16, 16);

        // Floor tile should have some color (not fully transparent)
        let alpha = pixel[3];
        assert!(
            alpha > 0,
            "Floor tile (tile 0) should not be fully transparent"
        );
    }

    #[test]
    fn tileset_wall_tile_exists() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        // Sample pixels from tile 1 (wall tile, right half)
        // Check center of tile 1 (48, 16) = (32 + 16, 16)
        let pixel = img.get_pixel(48, 16);

        // Wall tile should have some color (not fully transparent)
        let alpha = pixel[3];
        assert!(
            alpha > 0,
            "Wall tile (tile 1) should not be fully transparent"
        );
    }

    #[test]
    fn tileset_floor_and_wall_are_different() {
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        // Sample center pixels from each tile
        let floor_pixel = img.get_pixel(16, 16);
        let wall_pixel = img.get_pixel(48, 16);

        // Tiles should have different colors
        let floor_color = (floor_pixel[0], floor_pixel[1], floor_pixel[2]);
        let wall_color = (wall_pixel[0], wall_pixel[1], wall_pixel[2]);

        assert_ne!(
            floor_color, wall_color,
            "Floor and wall tiles should have different colors"
        );
    }

    #[test]
    fn tileset_can_be_used_by_bevy() {
        // Test that the tileset dimensions work with the expected tile size
        let img = image::open("assets/sprites/tileset.png").expect("Should load tileset");

        let (width, height) = img.dimensions();
        let expected_tile_size = 32u32;

        // Width should be evenly divisible by tile size
        assert_eq!(
            width % expected_tile_size,
            0,
            "Tileset width should be divisible by tile size (32)"
        );

        // Height should be evenly divisible by tile size
        assert_eq!(
            height % expected_tile_size,
            0,
            "Tileset height should be divisible by tile size (32)"
        );

        // Should have at least 2 tiles (floor and wall)
        let num_tiles = (width / expected_tile_size) * (height / expected_tile_size);
        assert!(
            num_tiles >= 2,
            "Tileset should have at least 2 tiles (floor + wall), got {}",
            num_tiles
        );
    }
}
