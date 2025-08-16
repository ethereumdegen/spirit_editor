 
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

//r channel is for chromatic abberation, g channel is for warping 
@group(0) @binding(2) var post_processing_effects_texture: texture_2d<f32>; 
@group(0) @binding(3) var post_processing_effects_sampler: sampler;
 
 

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    
    // Get screen and effects texture dimensions
    let screen_size = vec2<f32>(textureDimensions(screen_texture));
    let effects_size = vec2<f32>(textureDimensions(post_processing_effects_texture));
    
    // Calculate aspect ratios
    let screen_aspect = screen_size.x / screen_size.y;
    let effects_aspect = effects_size.x / effects_size.y;
    
    // Correct UV coordinates for effects texture sampling
    var effects_uv = in.uv;
    

  
    
    // Sample the effects texture with corrected UV coordinates
    let effects_sample = textureSample(post_processing_effects_texture, texture_sampler, effects_uv);
    
    // Extract distortion from RG channels (assuming values are in 0-1 range)
    let distortion_sample = effects_sample.rg;
    // Convert from 0-1 to -0.5 to 0.5 range for distortion
    let uv_distortion = distortion_sample  * 0.2; // Fixed: use constant instead of distortion_sample
    
    // Apply distortion to UV coordinates
    let distorted_uv = in.uv + (in.uv * uv_distortion);
    
    // Chromatic aberration strength from blue channel
    let aberration_strength = effects_sample.b * 0.01; // Fixed: use constant since no settings struct
    
    // Sample each color channel with chromatic aberration offset -- different colors are sampled differently 
    let red_offset = vec2<f32>(aberration_strength, -aberration_strength);
    let green_offset = vec2<f32>(-aberration_strength, 0.0);
    let blue_offset = vec2<f32>(0.0, aberration_strength);
    
    let red_sample = textureSample(screen_texture, texture_sampler,  distorted_uv + red_offset).r;
    let green_sample = textureSample(screen_texture, texture_sampler,  distorted_uv + green_offset).g;
    let blue_sample = textureSample(screen_texture, texture_sampler,  distorted_uv + blue_offset).b;
    
    let output = vec4<f32>(red_sample, green_sample, blue_sample, 1.0);
    
    return output;
 

}