// Bevy 0.16 WGSL Lighting Shader
// Implements dynamic circular lighting effect for 2D gameplay
// Used for candle-based visibility system in the house escape game

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Light position in world space
@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> light_position: vec2<f32>;

// Light radius (visibility range)
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var<uniform> light_radius: f32;

// Light color (RGB + intensity)
@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var<uniform> light_color: vec4<f32>;

/// Fragment shader for circular gradient lighting
///
/// Calculates lighting intensity based on distance from light source.
/// Creates a smooth circular gradient from bright center to dark edges.
///
/// # Algorithm
/// 1. Calculate distance from fragment to light position
/// 2. Normalize distance by light radius
/// 3. Apply smoothstep for smooth falloff
/// 4. Multiply by light color and intensity
///
/// # Returns
/// RGBA color with calculated lighting intensity
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from fragment to light source
    let distance = length(mesh.world_position.xy - light_position);

    // Normalize distance and invert for brightness (1.0 = bright, 0.0 = dark)
    // smoothstep creates smooth gradient transition
    let normalized_distance = distance / light_radius;
    let intensity = 1.0 - smoothstep(0.0, 1.0, normalized_distance);

    // Apply light color and intensity
    // RGB from light_color, alpha controls overall visibility
    return vec4<f32>(
        light_color.rgb * intensity,
        intensity * light_color.a
    );
}
