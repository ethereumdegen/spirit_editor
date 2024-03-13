 
//see bindings in terrain_material.rs 
  
 #import bevy_pbr::{
    mesh_view_bindings::globals, 
    forward_io::{VertexOutput, FragmentOutput}, 
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
      pbr_deferred_functions::deferred_output,
}
   

struct StandardMaterial {
    time: f32,
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    alpha_cutoff: f32,
};
 
struct CustomMaterialUniforms {
   distortion_speed_x:  f32   ,
    distortion_speed_y:  f32   ,
   scroll_repeats_x: f32 ,
   scroll_repeats_y: f32 ,
    scroll_speed_x: f32,
    scroll_speed_y: f32,
   
    distortion_amount: f32 ,
    distortion_cutoff: f32 ,
    
    
};

 


@group(2) @binding(20)
var<uniform> custom_uniforms: CustomMaterialUniforms;
 
@group(2) @binding(21)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(22)
var base_color_sampler: sampler;
 
 



fn get_repeated_uv_coords(coords: vec2<f32>) -> vec2<f32> {
    let repeated_coords = vec2<f32>(
        (coords.x % (1. / f32(custom_uniforms.scroll_repeats_x))) * f32(custom_uniforms.scroll_repeats_x),
        (coords.y % (1. / f32(custom_uniforms.scroll_repeats_y))) * f32(custom_uniforms.scroll_repeats_y)
    );
    return repeated_coords;
}


//should consider adding vertex painting to this .. need another binding of course.. performs a color shift 

 
@fragment
fn fragment(
    mesh: VertexOutput,
    @builtin(front_facing) is_front: bool,
) ->   FragmentOutput {
    
    let scroll_amount_x = (globals.time * custom_uniforms.scroll_speed_x)  ;
    let scroll_amount_y = (globals.time * custom_uniforms.scroll_speed_y)  ;
  //make the cutoff big and it wont have any effect
    let distortion_radians_x = 6.28 * (globals.time * custom_uniforms.distortion_speed_x % mesh.uv[0]) ;
    let distortion_amount_x = ( sin(distortion_radians_x) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff   ;
    
    let distortion_radians_y = 6.28 * (globals.time * custom_uniforms.distortion_speed_y % mesh.uv[1]) ;
    let distortion_amount_y = ( cos(distortion_radians_y) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff  ;
 
    let tiled_uv =   get_repeated_uv_coords (mesh.uv + vec2(scroll_amount_x,scroll_amount_y)  ) 
       + vec2( distortion_amount_x, distortion_amount_y ) ;
     
    
    //this technique lets us use 255 total textures BUT we can only layer 2 at a time.  
    let color_from_texture_0 = textureSample(base_color_texture, base_color_sampler, tiled_uv );
 
    let blended_color = color_from_texture_0   ;


   
  // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(mesh, is_front);
 
    //hack the material (StandardMaterialUniform)  so the color is from the terrain splat 
  
     // alpha discard
    pbr_input.material.base_color =  pbr_input.material.base_color * blended_color ;

    var final_color = alpha_discard(pbr_input.material, pbr_input.material.base_color  )  ;

    
    
    var pbr_out: FragmentOutput;
     //only apply lighting if bit is set
       if ((pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
       
            
        // pbr_input.material.base_color =  blended_color;

         pbr_out.color = apply_pbr_lighting(pbr_input);
    
         pbr_out.color = main_pass_post_lighting_processing(pbr_input, pbr_out.color);
      
          final_color = pbr_out.color;
       }  


    // -----
   pbr_out.color = final_color;
   // pbr_out.emissive = pbr_input.material.emissive;

   // if (final_color.a < 0.1) { // Use your threshold value here
    //    discard;
   // }

       
      
 
    return pbr_out;
    
}
 