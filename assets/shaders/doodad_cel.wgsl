
 
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
      mesh_view_bindings as view_bindings,
}

 
#import bevy_pbr::{

       pbr_bindings,
      pbr_types,
     
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
 


struct DoodadMaterialUniforms {
   tint_color: vec4<f32>,

   //fixed_uv_config_bits: u32, 

   use_fixed_world_uv:u32, //bool 
   blank_top_bottom:u32, //bool 


   uv_input_scale: f32 , 
 
    
};

@group(2) @binding(20)
var<uniform> custom_uniforms: DoodadMaterialUniforms;
 



@group(2) @binding(100) var mask: texture_2d<f32>;
@group(2) @binding(101) var mask_sampler: sampler;
@group(2) @binding(102) var<uniform> highlight_color: vec4<f32>;
@group(2) @binding(103) var<uniform> shadow_color: vec4<f32>;
@group(2) @binding(104) var<uniform> rim_color: vec4<f32>;


  

@fragment
fn fragment(
     in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    
      var pbr_input = pbr_input_from_standard_material(in, is_front);
         

         let fixed_world_uv = (custom_uniforms.use_fixed_world_uv  ) != 0;
         let blank_top_bottom = (custom_uniforms.blank_top_bottom  ) != 0;

            if fixed_world_uv == true {
            
                let triplanar_weights = triplanar_mapping_lerp_output ( in.world_normal );

                  let uv_transform = pbr_bindings::material.uv_transform;

                   var original_base_color = pbr_bindings::material.base_color;
                  //  var color = pbr_bindings::material.base_color;

                   
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
                    
                    pbr_input.material.base_color = original_base_color * triplanar_color;

             }



         // cel shading code 

          pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);





         // remove and store texture
            let texture = pbr_input.material.base_color;
            pbr_input.material.base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

            var out: FragmentOutput;
            out.color = apply_pbr_lighting(pbr_input);  //apply lighting to a white texture to understand just the lighting 

               // FIXED: Clamp the lighting output BEFORE calculating average
            // This prevents bright lights from causing values > 1.0
            out.color = clamp(out.color, vec4<f32>(0.0), vec4<f32>(0.9));


  


            let lighting_average  = (out.color.r + out.color.g + out.color.b ) / 3.0 ;

            let saturated_lighting_average = saturate(lighting_average);


              // Source for cel shading: https://www.youtube.com/watch?v=mnxs6CR6Zrk]
            // sample mask at the current fragment's intensity as u to get the cutoff
            let uv = vec2<f32>(saturated_lighting_average, 0.0);
            let quantization = textureSample(mask, mask_sampler, uv);
            out.color = mix(shadow_color, highlight_color, quantization);


            // apply rim highlights. Inspired by Breath of the Wild: https://www.youtube.com/watch?v=By7qcgaqGI4
            let eye = normalize(view_bindings::view.world_position.xyz - in.world_position.xyz);
            let rim = 1.0 - abs(dot(eye, in.world_normal));
            let rim_factor = rim * rim * rim * rim;
            out.color = mix(out.color, rim_color, rim_factor);

            // Reapply texture
            out.color = out.color * texture;
            pbr_input.material.base_color = texture;

            // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
            // note this does not include fullscreen postprocessing effects like bloom.
            out.color = main_pass_post_lighting_processing(pbr_input, out.color);

      


         // Modify the final result based on tint_color
        if (all(custom_uniforms.tint_color.rgb > vec3<f32>(1.0, 1.0, 1.0))) {
            out.color += custom_uniforms.tint_color - vec4<f32>(1.0, 1.0, 1.0, 1.0);
        } else {
            out.color *= custom_uniforms.tint_color;
        }


         out.color= clamp(out.color, vec4<f32>(0.0), vec4<f32>(1.0));
    

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
    // The transform matrix contains scale and translation for atlas subset
    
    // Get the scale and translation from the transform matrix
    let scale = vec2<f32>(transform[0][0], transform[1][1]);
    let translation = vec2<f32>(transform[2][0], transform[2][1]);
    
    // Apply fractional part to handle tiling, then scale and translate
    let tiled_uv = fract(original_uv);
    let transformed_uv = tiled_uv * scale + translation;
    
    // Clamp to ensure we stay within the atlas subrectangle bounds
    let min_bounds = translation;
    let max_bounds = translation + scale;
    
    return clamp(transformed_uv, min_bounds, max_bounds);
}

 