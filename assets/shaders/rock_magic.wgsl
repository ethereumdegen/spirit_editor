 
 
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

 
#import bevy_pbr::{

       pbr_bindings,
      pbr_types,
        

    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
 


struct RockMagicMaterialUniforms {
   tint_color: vec4<f32>,
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

        let triplanar_weights = triplanar_mapping_lerp_output ( in.world_normal );


         var color = pbr_bindings::material.base_color;


         var uv_A = in.world_position.xy * 0.125;
         var uv_B = in.world_position.yz * 0.125;

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

          
          let triplanar_color  =  triplanar_weights.x * color_B

         + triplanar_weights.y * color_B //this is for the top and bottom ...w

         + triplanar_weights.z *  color_A
           ;
            
            pbr_input.material.base_color =  triplanar_color;





            var out: FragmentOutput;
            
            //can i change this up ?? toon lighting ? 
           out.color = apply_pbr_lighting(pbr_input);  //shadows 
 
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
 