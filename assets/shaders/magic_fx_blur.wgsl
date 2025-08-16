 
 
  
 #import bevy_pbr::{
    mesh_view_bindings::globals, 
    forward_io::{VertexOutput, FragmentOutput}, 
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
      pbr_deferred_functions::deferred_output
}
 #import bevy_pbr::mesh_functions
 #import bevy_pbr::prepass_utils

//  #import bevy_shader_utils::fresnel::fresnel
 #import bevy_pbr::mesh_view_bindings::view


#import bevy_pbr::view_transformations::{
position_clip_to_world,
sition_view_to_clip, 
position_clip_to_view,
position_view_to_world,
depth_ndc_to_view_z
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

    distortion_speed:  vec2<f32>, 
    scroll_repeats :  vec2<f32>, 
     scroll_speed :  vec2<f32>, 
   
   
    distortion_amount: f32 ,
    distortion_cutoff: f32 ,

    depth_cutoff_offset: f32 ,
    animation_frame_dimension: vec2<f32>, 
 
    current_animation_frame_index: u32,

     uv_scale_factor: vec2<f32>, 

    tint_color: vec4<f32>,  

   

    fresnel_power: f32 ,
    //fresnel color ?
    // disturbance effect ?  

    use_masking_texture: u32, 
    animate_masking_texture: u32, 
    
};

 


@group(2) @binding(20)
var<uniform> custom_uniforms: CustomMaterialUniforms;
 
@group(2) @binding(21)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(22)
var base_color_sampler: sampler;
 
 
@group(2) @binding(23)
var  masking_texture: texture_2d<f32>;
@group(2) @binding(24)
var  masking_sampler: sampler;


fn get_repeated_uv_coords(coords: vec2<f32>) -> vec2<f32> {
    let repeated_coords = vec2<f32>(
        (coords.x % (1. / f32(custom_uniforms.scroll_repeats.x))) * f32(custom_uniforms.scroll_repeats.x),
        (coords.y % (1. / f32(custom_uniforms.scroll_repeats.y))) * f32(custom_uniforms.scroll_repeats.y)
    );
    return repeated_coords;
}

 
fn get_slideshow_uv_coords(coords: vec2<f32>, anim_frame_dimension_x: u32, anim_frame_dimension_y: u32, index: u32) -> vec2<f32> {
    

    let num_layers_x = anim_frame_dimension_x;
    let num_layers_y = anim_frame_dimension_y;
    
     let layer_width = 1.0 / f32(num_layers_x);
    let layer_height = 1.0 / f32(num_layers_y);

    let x_index = index % num_layers_x;
     let y_index = index / num_layers_x;

    let x_offset = f32(x_index) * layer_width;
    let y_offset = f32(y_index) * layer_height;
    
    let slideshow_coords = vec2<f32>(
        (coords.x * layer_width) + x_offset ,
        (coords.y * layer_height) + y_offset
    );
    
    return slideshow_coords;


}

 

 
@fragment
fn fragment(
    mesh: VertexOutput,
    @builtin(front_facing) is_front: bool,
 
  //  #ifdef MULTISAMPLED
  //      @builtin(sample_index) sample_index: u32,
  //  #endif

) ->   FragmentOutput {
    
  //  #ifndef MULTISAMPLED
        let sample_index = 0u;
  //  #endif


    let scroll_amount_x = (globals.time * custom_uniforms.scroll_speed.x)  ;
    let scroll_amount_y = (globals.time * custom_uniforms.scroll_speed.y)  ; 



    
    let scaled_uv = (mesh.uv / custom_uniforms.uv_scale_factor) % 1.0;
 
    var tiled_uv =   get_repeated_uv_coords ( scaled_uv + vec2(scroll_amount_x,scroll_amount_y)  )    ;


    if (u32(custom_uniforms.animation_frame_dimension.x) > 1u || u32(custom_uniforms.animation_frame_dimension.y) > 1u) {
        

       let current_layer_index = custom_uniforms.current_animation_frame_index;

        //this should 
        tiled_uv =  get_slideshow_uv_coords( 
         mesh.uv ,
         u32(custom_uniforms.animation_frame_dimension.x),
         u32(custom_uniforms.animation_frame_dimension.y),
         current_layer_index
         )   ;   


     }


  

      //make the cutoff big and it wont have any effect
    
    let distortion_radians_x =  (globals.time * custom_uniforms.distortion_speed.x + mesh.uv[0] * 2.0 ) % 6.28 ;
    let distortion_amount_x = ( sin(distortion_radians_x) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff   ;
    
    let distortion_radians_y =   (globals.time * custom_uniforms.distortion_speed.y + mesh.uv[1] * 2.0 ) % 6.28 ;
    let distortion_amount_y = ( cos(distortion_radians_y) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff  ;


    let distorted_uv = tiled_uv + vec2( distortion_amount_x, distortion_amount_y );
 
    let blended_color = textureSample(base_color_texture, base_color_sampler, distorted_uv )   ;



  

 


   
  // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(mesh, is_front);
 
    //hack the material (StandardMaterialUniform)  so the color is from the terrain splat 
  
     // alpha discard
    pbr_input.material.base_color =  pbr_input.material.base_color * blended_color ;


    pbr_input.material.emissive = pbr_input.material.emissive * blended_color;

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

    //tint also affect emissive color? 

   pbr_out.color = final_color * custom_uniforms.tint_color;
   pbr_out.color.a *= custom_uniforms.tint_color.a; //exponential alpha decay from the tint color !! 

   //using fresnel

     let fresnel_power = custom_uniforms.fresnel_power;

    if  (fresnel_power > 0.01){ 
          
           let fresnel_strength = 1.0;
           var fresnel = saturate (0.4 * fresnel(
            view.world_position.xyz, 
            mesh.world_position.xyz, 
            mesh.world_normal, 
            fresnel_power, 
            fresnel_strength
            ));


           pbr_out.color.a =  pbr_out.color.a * fresnel ; 
      }
    // pbr_out.emissive = pbr_input.material.emissive * blended_color;



     let animate_mask_texture = (custom_uniforms.animate_masking_texture ) != 0;

    var mask_texture_uv = mesh.uv; 
    if (animate_mask_texture) {
        mask_texture_uv = tiled_uv;
    }

      let use_mask_texture = (custom_uniforms.use_masking_texture ) != 0;
   
 
     // Apply masking texture if available
    if ( use_mask_texture ) {
        let mask_value = textureSample(masking_texture, masking_sampler, mask_texture_uv ).r;
        pbr_out.color.a =  pbr_out.color.a * mask_value;  // apply mask 
    }



  // var position = mesh.position; //this is frag_coord ? 

  

      #ifdef DEPTH_PREPASS


       // Get the scene depth from the depth prepass
        let scene_depth = prepass_utils::prepass_depth(mesh.position, sample_index);

        // Convert fragment depth to view space for proper comparison
        let fragment_depth_view = depth_ndc_to_view_z(mesh.position.z);
        let scene_depth_view = depth_ndc_to_view_z(scene_depth);

        // Calculate depth difference in view space (more intuitive units)
        let depth_difference = fragment_depth_view - scene_depth_view;

        // Apply soft depth fade instead of hard cutoff
        let fade_distance = custom_uniforms.depth_cutoff_offset;
        
        if depth_difference < 1.0 {
             discard; 
           //      pbr_out.color = vec4<f32>(0.0,0.0,0.0,1.0);
        }

 
          
   
          
    #endif

      
 
    return pbr_out;
    
}




fn alpha_blend(top: vec4<f32>, bottom: vec4<f32>) -> vec4<f32> {
    let color = top.rgb * top.a + bottom.rgb * (1.0 - top.a);
    let alpha = top.a + bottom.a * (1.0 - top.a);
    return vec4<f32>(color, alpha);  
}
 


 //from bevy_shader_utils 
fn fresnel(
    camera_view_world_position: vec3<f32>,
    world_position: vec3<f32>,
    world_normal: vec3<f32>,
    power: f32,
    strength: f32,
) -> f32 {
    // The view vector. V is a unit vector pointing from the fragment
    // on the sphere toward the camera.
    //
    // this comment is how you would write it in your own code
    // var V = normalize(view.world_position.xyz - world_position.xyz);
    var V = normalize(camera_view_world_position - world_position);

    // The dot product returns the angle between N and V where 
    // fragments on the sphere that are pointing at the camera
    // (have the same angle as the V) are 1.0, faces perpendicular 
    // to V are 0.0, faces pointing away are -1.0.
    var fresnel = 1.0 - dot(world_normal, V);

    // The fresnel value here is the inverse of NdotV. 
    // So fragments pointing away will now be 1.0 and ones 
    // pointing at the camera will be 0.0
    // var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);

    // Here's were increasing the contrast with pow 
    // and making it brighter by multiplying by 2
    return pow(fresnel, power) * strength;
};


 