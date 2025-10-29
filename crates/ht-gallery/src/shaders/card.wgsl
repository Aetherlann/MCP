// Gallery card shader - beautiful rounded cards with elevation shadows
//
// This shader renders media cards with:
// - Rounded corners (smooth SDF-based)
// - Elevation shadows (soft, realistic)
// - Smooth color gradients
// - Border effects
// - Glass-like aesthetics

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) border_radius: f32,
    @location(5) elevation: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) border_color: vec4<f32>,
    @location(3) border_radius: f32,
    @location(4) elevation: f32,
    @location(5) local_pos: vec2<f32>,  // Position within card (0-1)
}

struct Uniforms {
    elevation: f32,
    scale: f32,
    opacity: f32,
    padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var card_texture: texture_2d<f32>;

@group(0) @binding(2)
var card_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Apply scale from uniforms
    let scaled_pos = input.position * uniforms.scale;

    // Transform to NDC (assuming orthographic projection)
    output.position = vec4<f32>(scaled_pos, 0.0, 1.0);

    // Pass through
    output.tex_coords = input.tex_coords;
    output.color = input.color;
    output.border_color = input.border_color;
    output.border_radius = input.border_radius;
    output.elevation = input.elevation;
    output.local_pos = input.tex_coords; // Use tex coords as local position

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample texture
    var tex_color = textureSample(card_texture, card_sampler, input.tex_coords);

    // Get card dimensions from derivatives
    let dx = length(vec2<f32>(dpdx(input.local_pos.x), dpdy(input.local_pos.x)));
    let dy = length(vec2<f32>(dpdx(input.local_pos.y), dpdy(input.local_pos.y)));

    // Calculate distance from edges for rounded corners
    let border_width = 1.0; // 1px border
    let radius = input.border_radius;

    // Distance to nearest edge
    let edge_dist = sdf_rounded_box(
        input.local_pos,
        vec2<f32>(0.5, 0.5),  // Center
        vec2<f32>(0.5, 0.5),  // Half size
        radius / 100.0         // Normalize radius
    );

    // Smooth border
    let border_alpha = smoothstep(-border_width, 0.0, edge_dist);

    // Shadow calculation based on elevation
    let shadow = calculate_shadow(input.local_pos, input.elevation);

    // Blend background color, texture, and border
    var final_color: vec4<f32>;

    if (tex_color.a > 0.01) {
        // Has texture - use it
        final_color = tex_color;
    } else {
        // No texture - use solid color
        final_color = input.color;
    }

    // Apply border
    let border_mix = smoothstep(0.0, border_width, -edge_dist);
    final_color = mix(final_color, input.border_color, border_mix);

    // Apply shadow (darken based on elevation)
    final_color.rgb = final_color.rgb * (1.0 - shadow * 0.3);

    // Apply rounded corner clipping
    final_color.a = final_color.a * smoothstep(-1.0, 0.0, edge_dist);

    // Apply global opacity
    final_color.a = final_color.a * uniforms.opacity;

    return final_color;
}

// Signed distance function for rounded box
fn sdf_rounded_box(p: vec2<f32>, center: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let offset = abs(p - center) - half_size + vec2<f32>(radius, radius);
    let outside = length(max(offset, vec2<f32>(0.0, 0.0)));
    let inside = min(max(offset.x, offset.y), 0.0);
    return outside + inside - radius;
}

// Calculate soft shadow based on elevation
fn calculate_shadow(pos: vec2<f32>, elevation: f32) -> f32 {
    if (elevation < 0.1) {
        return 0.0;
    }

    // Shadow is stronger at edges and top
    let edge_factor = smoothstep(0.1, 0.0, min(
        min(pos.x, 1.0 - pos.x),
        min(pos.y, 1.0 - pos.y)
    ));

    // Stronger shadow at bottom (simulating light from top)
    let vertical_factor = smoothstep(0.5, 1.0, pos.y);

    // Combine factors
    let shadow_intensity = (edge_factor * 0.5 + vertical_factor * 0.5) * (elevation / 4.0);

    return clamp(shadow_intensity, 0.0, 0.5);
}

// Smooth minimum for soft shadows (not used but useful)
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

// Gradient function for accent effects
fn gradient(pos: vec2<f32>, color1: vec4<f32>, color2: vec4<f32>) -> vec4<f32> {
    let t = pos.y; // Vertical gradient
    return mix(color1, color2, t);
}
