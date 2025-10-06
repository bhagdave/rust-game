use bevy::math::Vec2;
use bevy::prelude::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

// Note: We can't directly benchmark the shader (GPU code), but we can benchmark
// the CPU-side lighting calculations and material updates

/// Simulates the lighting calculation that would happen in the shader
///
/// This is the core algorithm from lighting.wgsl fragment shader:
/// 1. Calculate distance from point to light
/// 2. Normalize by radius
/// 3. Apply smoothstep for falloff
/// 4. Calculate intensity
fn calculate_lighting_at_point(point: Vec2, light_pos: Vec2, light_radius: f32) -> f32 {
    let distance = point.distance(light_pos);
    let normalized_distance = distance / light_radius;

    // Smoothstep implementation: 3t² - 2t³ where t is clamped to [0,1]
    let t = normalized_distance.clamp(0.0, 1.0);
    let smoothed = 3.0 * t * t - 2.0 * t * t * t;

    // Intensity is inverse of smoothed distance (1.0 = bright, 0.0 = dark)
    1.0 - smoothed
}

/// Benchmark single point lighting calculation
fn bench_single_point(c: &mut Criterion) {
    c.bench_function("lighting single point", |b| {
        let light_pos = Vec2::new(960.0, 540.0); // Screen center
        let point = Vec2::new(1200.0, 700.0);
        let radius = 300.0;

        b.iter(|| {
            black_box(calculate_lighting_at_point(
                black_box(point),
                black_box(light_pos),
                black_box(radius),
            ))
        });
    });
}

/// Benchmark lighting calculations for a grid of points (simulates fragment shader workload)
fn bench_lighting_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("lighting grid");

    // Test different grid sizes representing different screen resolutions
    for size in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let light_pos = Vec2::new(960.0, 540.0);
            let radius = 300.0;

            b.iter(|| {
                let mut total_intensity = 0.0;
                for x in 0..size {
                    for y in 0..size {
                        let point = Vec2::new(x as f32, y as f32);
                        total_intensity += calculate_lighting_at_point(
                            black_box(point),
                            black_box(light_pos),
                            black_box(radius),
                        );
                    }
                }
                black_box(total_intensity)
            });
        });
    }

    group.finish();
}

/// Benchmark multiple light sources (worst case scenario)
fn bench_multiple_lights(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple lights");

    for num_lights in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_lights),
            num_lights,
            |b, &num_lights| {
                // Create multiple light sources
                let lights: Vec<(Vec2, f32)> = (0..num_lights)
                    .map(|i| {
                        let angle = (i as f32 / num_lights as f32) * std::f32::consts::TAU;
                        let pos =
                            Vec2::new(960.0 + 400.0 * angle.cos(), 540.0 + 400.0 * angle.sin());
                        (pos, 200.0) // position and radius
                    })
                    .collect();

                let point = Vec2::new(960.0, 540.0); // Center point

                b.iter(|| {
                    let mut total_intensity = 0.0;
                    for (light_pos, radius) in &lights {
                        total_intensity += calculate_lighting_at_point(
                            black_box(point),
                            black_box(*light_pos),
                            black_box(*radius),
                        );
                    }
                    black_box(total_intensity)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark wax-based intensity calculation (from update_lighting_system)
fn bench_wax_intensity(c: &mut Criterion) {
    c.bench_function("wax intensity calculation", |b| {
        let wax = 50.0f32; // Half depleted candle

        b.iter(|| {
            let intensity = black_box((wax / 100.0).clamp(0.3, 1.0));
            black_box(intensity)
        });
    });
}

/// Benchmark realistic lighting update scenario
/// Simulates the CPU work done per frame in update_lighting_system
fn bench_lighting_update_per_frame(c: &mut Criterion) {
    c.bench_function("lighting update per frame", |b| {
        // Simulate candle data
        let candle_pos = Vec2::new(960.0, 540.0);
        let wax = 75.0f32;
        let radius = 300.0;

        b.iter(|| {
            // This simulates the work done in update_lighting_system
            let intensity = (wax / 100.0).clamp(0.3, 1.0);
            let _color = (1.0 * intensity, 0.9 * intensity, 0.7 * intensity, intensity);

            // Simulated lighting calculation for a sample point
            let sample_point = Vec2::new(1200.0, 700.0);
            let _light_value = calculate_lighting_at_point(
                black_box(sample_point),
                black_box(candle_pos),
                black_box(radius),
            );

            black_box(intensity)
        });
    });
}

/// Benchmark distance calculation methods
fn bench_distance_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("distance methods");

    let p1 = Vec2::new(100.0, 200.0);
    let p2 = Vec2::new(500.0, 600.0);

    group.bench_function("Vec2::distance", |b| {
        b.iter(|| black_box(p1.distance(black_box(p2))));
    });

    group.bench_function("Vec2::distance_squared", |b| {
        b.iter(|| black_box(p1.distance_squared(black_box(p2))));
    });

    group.bench_function("manual distance", |b| {
        b.iter(|| {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            black_box((dx * dx + dy * dy).sqrt())
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_single_point,
    bench_lighting_grid,
    bench_multiple_lights,
    bench_wax_intensity,
    bench_lighting_update_per_frame,
    bench_distance_methods,
);
criterion_main!(benches);
