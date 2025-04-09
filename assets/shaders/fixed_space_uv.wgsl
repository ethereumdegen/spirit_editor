 
 
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

 
#import bevy_pbr::{

       pbr_bindings,
      pbr_types,
       mesh_view_bindings as view_bindings,

    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
 


struct RockMagicMaterialUniforms {
   tint_color: vec4<f32>,

   fixed_uv_config_bits: u32, 

   uv_input_scale: f32 
    
};

@group(2) @binding(20)
var<uniform> custom_uniforms: RockMagicMaterialUniforms;
 
  

@fragment
fn fragment(
     in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    
      var pbr_input = pbr_input_from_standard_material(in, is_front);
        
        
      
       
        // apply lighting
        


        // Modify the final result based on tint_color
       /* if (all(custom_uniforms.tint_color.rgb > vec3<f32>(1.0, 1.0, 1.0))) {
            out.color += custom_uniforms.tint_color - vec4<f32>(1.0, 1.0, 1.0, 1.0);
        } else {
            out.color *= custom_uniforms.tint_color;
        }*/
        

        let blank_top_bottom = (custom_uniforms.fixed_uv_config_bits & 0x1) != 0;
 



        let triplanar_weights = triplanar_mapping_lerp_output ( in.world_normal );


        //   uv_transform: mat3x3<f32>,
        let uv_transform = pbr_bindings::material.uv_transform;


         var color = pbr_bindings::material.base_color;


         var uv_scale_factor =  custom_uniforms.uv_input_scale ; 
         var uv_scale_factor_inverse = 1.0 / uv_scale_factor; 


         var uv_A = in.world_position.xy * uv_scale_factor_inverse;
         var uv_B = in.world_position.zy * uv_scale_factor_inverse;

         var uv_C = in.world_position.xz * uv_scale_factor_inverse; 
         var uv_flat = vec2<f32>(0.0,0.0);

         if blank_top_bottom == true {
            uv_C = uv_flat; 
         }


         // ---- 
             // For uv_A (mapping world XY)
      uv_A = apply_uv_transform(uv_A , uv_transform);
      
      // For uv_B (mapping world ZY)
      uv_B = apply_uv_transform(uv_B, uv_transform);
      
      // For uv_C (mapping world XZ)
      if !blank_top_bottom {
          uv_C = apply_uv_transform(uv_C, uv_transform);
      }

      // -----
       

         let color_A = textureSample(
                pbr_bindings::base_color_texture,
                pbr_bindings::base_color_sampler,
                uv_A, 
            );
         let color_B = textureSample(
                pbr_bindings::base_color_texture,
                pbr_bindings::base_color_sampler,
                uv_B, 
            );

         let color_C = textureSample(
                pbr_bindings::base_color_texture,
                pbr_bindings::base_color_sampler,
                uv_C, 
            );

          
          let triplanar_color  =  triplanar_weights.x * color_B

         + triplanar_weights.y * color_C //this is for the top and bottom ... 

         + triplanar_weights.z *  color_A
           ;
            
            pbr_input.material.base_color =  triplanar_color;





            var out: FragmentOutput;
            
            //can i change this up ?? toon lighting ? 
            //https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr_functions.wgsl
           out.color = apply_pbr_lighting(pbr_input);  //shadows 
 
            

           /* let rim_color = vec4<f32>(1.0,1.0,1.0,0.2);

           //apply rim highlights 
            let eye = normalize(view_bindings::view.world_position.xyz - in.world_position.xyz);
            let rim = 1.0 - abs(dot(eye, in.world_normal));
            let rim_factor = rim * rim * rim * rim;
            out.color = mix(out.color, rim_color, rim_factor);*/


             out.color = main_pass_post_lighting_processing(pbr_input, out.color);  // fog 

        //let toon_lighting = calculate_toon_lighting( normal_mixed , view_dir, toon_material.sun_dir, toon_material.sun_color );
 
       // pbr_out.color  *= (toon_lighting + toon_material.ambient_color);  
 


    

        return out;


}



fn triplanar_mapping_lerp_output(
     world_normal: vec3<f32>,
 
   
) -> vec3<f32> {
    // Absolute value of the world normal to determine axis dominance
    let abs_normal = abs(world_normal);

    // Calculate blending weights for each axis (X, Y, Z)
    let sum = abs_normal.x + abs_normal.y + abs_normal.z;
    let weights = abs_normal / sum;

    // Optionally apply a bias or tweak to control blending
//    let x_weight = weights.x; // Contribution of X-axis (sides)
  //  let y_weight = weights.y; // Contribution of Y-axis (sides)
  //  let z_weight = weights.z; // Contribution of Z-axis (top)

    // Combine X and Y weights to control side texture blending
  //  let side_weight = (x_weight + z_weight)  ;


  //some clamping .. dont want so much blending ..
  //if the vector is quite upward facing, just render it as completely upward facing .  
   if weights.y > 0.33 {  
      return vec3<f32>(0.0,1.0,0.0) ;
   } 

    if weights.x > 0.33 {  
      return vec3<f32>(1.0,0.0,0.0) ;
    } 

    if weights.z > 0.33 {  
      return vec3<f32>(0.0,0.0,1.0) ;
   } 
 


  // the y weight is used for the normal diffuse tex !! 
    return  weights;
}
 

 // Helper function to apply UV transform to constrain coordinates to subtile
fn apply_uv_transform(original_uv: vec2<f32>, transform: mat3x3<f32>) -> vec2<f32> {
    // Apply the transformation to the UV coordinates
    // The transform matrix contains scale and translation
    
    // First, we need to handle the UV in homogeneous coordinates
    let uv_homogeneous = vec3<f32>(original_uv.x % 1.0, original_uv.y % 1.0 , 1.0);
    
    // Apply the transformation
    let transformed = transform * uv_homogeneous;
    
    // Convert back to 2D coordinates
    return transformed.xy;
}
