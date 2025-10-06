use std::fs;
use std::path::Path;

/// Integration test: Shader file validation
/// From tasks.md T034: Verify lighting shader exists and has valid content

#[test]
fn lighting_shader_file_exists() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    assert!(
        shader_path.exists(),
        "Lighting shader file should exist at assets/shaders/lighting.wgsl"
    );
}

#[test]
fn lighting_shader_has_required_imports() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Verify required Bevy imports
    assert!(
        shader_content.contains("#import bevy_sprite::mesh2d_vertex_output::VertexOutput"),
        "Shader should import VertexOutput from bevy_sprite"
    );
}

#[test]
fn lighting_shader_has_uniform_bindings() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Verify light_position uniform
    assert!(
        shader_content.contains("var<uniform> light_position: vec2<f32>"),
        "Shader should define light_position uniform"
    );

    // Verify light_radius uniform
    assert!(
        shader_content.contains("var<uniform> light_radius: f32"),
        "Shader should define light_radius uniform"
    );

    // Verify light_color uniform
    assert!(
        shader_content.contains("var<uniform> light_color: vec4<f32>"),
        "Shader should define light_color uniform"
    );
}

#[test]
fn lighting_shader_has_fragment_function() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Verify fragment shader function exists
    assert!(
        shader_content.contains("@fragment"),
        "Shader should have @fragment attribute"
    );

    assert!(
        shader_content.contains("fn fragment"),
        "Shader should define fragment function"
    );

    // Verify correct return type
    assert!(
        shader_content.contains("-> @location(0) vec4<f32>"),
        "Fragment function should return vec4<f32> at location 0"
    );
}

#[test]
fn lighting_shader_uses_smoothstep() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Verify smoothstep is used for gradient falloff
    assert!(
        shader_content.contains("smoothstep"),
        "Shader should use smoothstep for smooth lighting gradient"
    );
}

#[test]
fn lighting_shader_calculates_distance() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Verify distance calculation
    assert!(
        shader_content.contains("length"),
        "Shader should calculate distance using length function"
    );

    assert!(
        shader_content.contains("mesh.world_position"),
        "Shader should use mesh world position for distance calculation"
    );
}

#[test]
fn lighting_shader_has_proper_syntax() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    // Check for common WGSL syntax elements
    assert!(
        shader_content.contains("@group"),
        "Shader should use @group attribute for bindings"
    );

    assert!(
        shader_content.contains("@binding"),
        "Shader should use @binding attribute for uniforms"
    );

    // Verify no obvious syntax errors (opening/closing braces match)
    let open_braces = shader_content.matches('{').count();
    let close_braces = shader_content.matches('}').count();
    assert_eq!(
        open_braces, close_braces,
        "Opening and closing braces should match"
    );
}

#[test]
fn lighting_shader_file_is_not_empty() {
    let shader_path = Path::new("assets/shaders/lighting.wgsl");
    let shader_content =
        fs::read_to_string(shader_path).expect("Should be able to read lighting shader file");

    assert!(
        !shader_content.trim().is_empty(),
        "Shader file should not be empty"
    );

    // Should have reasonable minimum size (comments + code)
    assert!(
        shader_content.len() > 100,
        "Shader file should contain substantial code (> 100 bytes)"
    );
}
