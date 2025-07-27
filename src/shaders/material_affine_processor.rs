
use std::any::TypeId; 

 
use bevy::math::Affine2;
 
 
use bevy::prelude::*; 


use bevy_materialize::load::processor::MaterialProcessorContext;
use bevy::{prelude::PartialReflect, reflect::{TypeRegistration, TypeRegistry}};
use bevy_materialize::load::processor::MaterialProcessor;
use serde::Deserialize;

 

// could make this an enum ? we shall see 





#[derive(Deserialize, Reflect ,Clone , Debug )]  // need to register/reflect this ? 
struct MaterialAffineProperties {
	 pub uv_scale_factor: Option<f32>,
	 pub texture_subset_dimensions: Option< TextureSubsetDimensions > ,
 
}


#[derive(Clone,Debug,Reflect,Default, Deserialize )]
pub struct TextureSubsetDimensions {

    pub offset: IVec2 ,
    pub dimensions: IVec2,
    pub base_texture_dimensions: IVec2 
 

}

impl TextureSubsetDimensions {
	/*pub fn to_affine2(&self, uv_scale: f32) -> Affine2 {
 
 
		         let base_texture_dimensions = self. base_texture_dimensions;

		         // fixes the errant lines 
		         let scale_expansion = 0.999; 

                let scale = Vec2::new(
			        (self.dimensions.x as f32 / base_texture_dimensions.x as f32) * scale_expansion,
			        (self.dimensions.y as f32 / base_texture_dimensions.y as f32) * scale_expansion
			    ) * Vec2::splat(uv_scale);

                Affine2 {
                    matrix2: Mat2::from_diagonal(scale),

                     translation: Vec2::new(
			            self.offset.x as f32 / base_texture_dimensions.x as f32,
			            self.offset.y as f32 / base_texture_dimensions.y as f32
			        )
                }


	}*/

	  pub fn to_affine2(&self, uv_scale: f32) -> Affine2 {
        let base_dims = self.base_texture_dimensions.as_vec2();
        let subset_dims = self.dimensions.as_vec2();
        let offset = self.offset.as_vec2();
        
        // Calculate the scale factors for the subset within the base texture
        let subset_scale = Vec2::new(
            subset_dims.x / base_dims.x,
            subset_dims.y / base_dims.y
        );
        
        // Apply the user-specified UV scale
        let final_scale = subset_scale * uv_scale;
        
        // Calculate normalized offset (0.0 to 1.0 range)
        let normalized_offset = Vec2::new(
            offset.x / base_dims.x,
             offset.y / base_dims.y 
        );
        
          Affine2 {
	            matrix2: Mat2::from_diagonal(final_scale),
	            translation: normalized_offset,
	        }
	         
    }

}

#[derive(Clone)]
pub struct Affine2Processor<P: MaterialProcessor>(pub P);
impl<P: MaterialProcessor> MaterialProcessor for Affine2Processor<P> {
	type Child = P;
	fn child(&self) -> Option<&Self::Child> {
		Some(&self.0)
	}

	fn try_deserialize<'de, D: serde::Deserializer<'de>>(
		&self,
		_ctx: &mut MaterialProcessorContext,
		registration: &TypeRegistration,
		_registry: &TypeRegistry,
		deserializer: D,
	) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error> {
		if registration.type_id() == TypeId::of::<Affine2>() {
			let material_affine_properties = MaterialAffineProperties::deserialize(deserializer)?;

			let uv_scale_factor = material_affine_properties.uv_scale_factor.unwrap_or(1.0);


         let uv_affine_xform = match  material_affine_properties.texture_subset_dimensions {

            Some( tex_subset_dimensions ) =>  tex_subset_dimensions.to_affine2(uv_scale_factor  ), 
            None => Affine2::from_scale(Vec2::splat(uv_scale_factor))  

         };


			return Ok(Ok(Box::new( uv_affine_xform )));
		}

		Ok(Err(deserializer))
	}
}