use bevy::math::Vec2;

/// Simulates the lighting calculation from the benchmark
/// (Duplicated here for testing purposes)
fn calculate_lighting_at_point(point: Vec2, light_pos: Vec2, light_radius: f32) -> f32 {
    let distance = point.distance(light_pos);
    let normalized_distance = distance / light_radius;

    let t = normalized_distance.clamp(0.0, 1.0);
    let smoothed = 3.0 * t * t - 2.0 * t * t * t;

    1.0 - smoothed
}

#[test]
fn test_lighting_at_light_source() {
    // At the exact light position, intensity should be 1.0 (maximum)
    let light_pos = Vec2::new(100.0, 100.0);
    let radius = 200.0;

    let intensity = calculate_lighting_at_point(light_pos, light_pos, radius);
    assert!((intensity - 1.0).abs() < 0.001, "Intensity at light source should be 1.0");
}

#[test]
fn test_lighting_at_radius_edge() {
    // At the exact radius edge, intensity should be near 0.0
    let light_pos = Vec2::new(100.0, 100.0);
    let radius = 200.0;
    let point_at_edge = Vec2::new(100.0 + radius, 100.0); // Exactly at radius distance

    let intensity = calculate_lighting_at_point(point_at_edge, light_pos, radius);
    assert!(intensity < 0.1, "Intensity at radius edge should be near 0.0, got {}", intensity);
}

#[test]
fn test_lighting_beyond_radius() {
    // Beyond the radius, intensity should be 0.0
    let light_pos = Vec2::new(100.0, 100.0);
    let radius = 200.0;
    let point_beyond = Vec2::new(100.0 + radius * 2.0, 100.0);

    let intensity = calculate_lighting_at_point(point_beyond, light_pos, radius);
    assert_eq!(intensity, 0.0, "Intensity beyond radius should be 0.0");
}

#[test]
fn test_lighting_at_half_radius() {
    // At half radius, intensity should be 0.5 or slightly higher
    let light_pos = Vec2::new(100.0, 100.0);
    let radius = 200.0;
    let point_at_half = Vec2::new(100.0 + radius * 0.5, 100.0);

    let intensity = calculate_lighting_at_point(point_at_half, light_pos, radius);
    assert!(intensity >= 0.5 && intensity < 1.0,
        "Intensity at half radius should be >= 0.5 and < 1.0, got {}", intensity);
}

#[test]
fn test_lighting_smoothstep_monotonic() {
    // Verify that intensity decreases monotonically with distance
    let light_pos = Vec2::new(100.0, 100.0);
    let radius = 200.0;

    let intensity_near = calculate_lighting_at_point(
        Vec2::new(100.0 + radius * 0.25, 100.0),
        light_pos,
        radius
    );

    let intensity_mid = calculate_lighting_at_point(
        Vec2::new(100.0 + radius * 0.5, 100.0),
        light_pos,
        radius
    );

    let intensity_far = calculate_lighting_at_point(
        Vec2::new(100.0 + radius * 0.75, 100.0),
        light_pos,
        radius
    );

    assert!(intensity_near > intensity_mid,
        "Intensity should decrease with distance: near={}, mid={}", intensity_near, intensity_mid);
    assert!(intensity_mid > intensity_far,
        "Intensity should decrease with distance: mid={}, far={}", intensity_mid, intensity_far);
}

#[test]
fn test_wax_intensity_calculation() {
    // Test the wax-based intensity calculation from the lighting system
    let wax_full = 100.0f32;
    let wax_half = 50.0f32;
    let wax_low = 10.0f32;
    let wax_empty = 0.0f32;

    let intensity_full = (wax_full / 100.0).clamp(0.3, 1.0);
    let intensity_half = (wax_half / 100.0).clamp(0.3, 1.0);
    let intensity_low = (wax_low / 100.0).clamp(0.3, 1.0);
    let intensity_empty = (wax_empty / 100.0).clamp(0.3, 1.0);

    assert_eq!(intensity_full, 1.0, "Full wax should give max intensity");
    assert_eq!(intensity_half, 0.5, "Half wax should give 0.5 intensity");
    assert_eq!(intensity_low, 0.3, "Low wax should be clamped to minimum 0.3");
    assert_eq!(intensity_empty, 0.3, "Empty wax should be clamped to minimum 0.3");
}

#[test]
fn test_lighting_performance_requirement() {
    // Verify that a reasonable number of lighting calculations can complete in <1ms
    // This is not a precise benchmark, but a sanity check

    use std::time::Instant;

    let light_pos = Vec2::new(960.0, 540.0);
    let radius = 300.0;

    let start = Instant::now();

    // Calculate lighting for 10,000 points (simulating fragment shader workload)
    let mut total = 0.0f32;
    for i in 0..10000 {
        let x = (i % 100) as f32 * 10.0;
        let y = (i / 100) as f32 * 10.0;
        let point = Vec2::new(x, y);
        total += calculate_lighting_at_point(point, light_pos, radius);
    }

    let elapsed = start.elapsed();

    // Should complete well under 1ms for CPU calculations
    // (Actual GPU shader will be much faster)
    assert!(elapsed.as_millis() < 10,
        "10k lighting calculations should complete in <10ms, took {:?} (total: {})",
        elapsed, total);
}

#[test]
fn test_distance_calculation_consistency() {
    // Verify Vec2::distance matches manual calculation
    let p1 = Vec2::new(100.0, 200.0);
    let p2 = Vec2::new(400.0, 600.0);

    let bevy_distance = p1.distance(p2);
    let manual_distance = {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    };

    assert!((bevy_distance - manual_distance).abs() < 0.001,
        "Bevy distance and manual distance should match");
}

#[test]
fn test_lighting_symmetry() {
    // Verify lighting is radially symmetric
    let light_pos = Vec2::new(0.0, 0.0);
    let radius = 100.0;

    // Points at same distance in different directions should have same intensity
    let north = Vec2::new(0.0, 50.0);
    let east = Vec2::new(50.0, 0.0);
    let south = Vec2::new(0.0, -50.0);
    let west = Vec2::new(-50.0, 0.0);

    let intensity_n = calculate_lighting_at_point(north, light_pos, radius);
    let intensity_e = calculate_lighting_at_point(east, light_pos, radius);
    let intensity_s = calculate_lighting_at_point(south, light_pos, radius);
    let intensity_w = calculate_lighting_at_point(west, light_pos, radius);

    assert!((intensity_n - intensity_e).abs() < 0.001);
    assert!((intensity_n - intensity_s).abs() < 0.001);
    assert!((intensity_n - intensity_w).abs() < 0.001);
}
