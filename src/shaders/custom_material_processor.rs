
use crate::materialize_properties::TextureSubsetDimensions;
use std::convert::Infallible;

use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
 
use bevy::image::ImageLoader;
use bevy::math::Affine2;
use bevy::platform::collections::hash_map::HashMap; 
 use bevy::tasks::ConditionalSendFuture;
use bevy_materialize::generic_material::ErasedMaterial;
use bevy_materialize::load::simple::SimpleGenericMaterialLoader;
use bevy_materialize::prelude::GenericMaterial;
use bevy::prelude::*; 


use bevy_materialize::load::ReflectGenericMaterialSubAsset;
use bevy_materialize::load::relative_asset_path;
use bevy::{asset::{LoadedUntypedAsset, ReflectHandle}, prelude::Handle};
use bevy_materialize::load::processor::MaterialProcessorContext;
use bevy::{prelude::PartialReflect, reflect::{TypeRegistration, TypeRegistry}};
use bevy_materialize::load::processor::MaterialProcessor;
use serde::Deserialize;

 


/// Material processor that loads assets from paths.
#[derive(Clone)]
pub struct CustomMaterialProcessor<P: MaterialProcessor>(pub P);
impl<P: MaterialProcessor> MaterialProcessor for CustomMaterialProcessor<P> {
	type Child = P;
	fn child(&self) -> Option<&Self::Child> {
		Some(&self.0)
	}

	fn try_deserialize<'de, D: serde::Deserializer<'de>>(
		&self,
		ctx: &mut MaterialProcessorContext,
		registration: &TypeRegistration,
		registry: &TypeRegistry,
		deserializer: D,
	) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error> {
		if let Some(loader) = registration.data::<ReflectGenericMaterialSubAsset>() {
			let path = String::deserialize(deserializer)?;

			let path = relative_asset_path(ctx.load_context.asset_path(), &path).map_err(serde::de::Error::custom)?;

			let mut loaded = loader.load(ctx, path) ;	


			// can i modify the uv_transform of the material here?? 


			/*
			If the registration represents Affine2:
			    Deserialize your struct with your preferred interface with `deserializer`
			    Convert said struct to an Affine2
			    return Ok(Ok(affine2))
			else
			    Ok(Err(deserializer)) // (give up the deserializer to the next in the stack)
  			*/ 



  			// Try to modify the UV transform if this is a StandardMaterial
            if let Some(standard_material) = loaded.as_mut().try_downcast_mut::<StandardMaterial>() {


               /*	 	
                    // can i get these from the material properties somehow? 

                   let uv_scale_factor  = loaded_generic_material.get_property(GenericMaterial::UV_SCALE_FACTOR) .unwrap_or( &1.0 ) ;

                    let tex_subset_dimensions: Result<TextureSubsetDimensions, GetPropertyError> = loaded_generic_material
                                    .get_property(GenericMaterial::TEXTURE_SUBSET_DIMENSIONS).cloned();

                    */

	                 let uv_scale_factor  = 1.0; 
	                 let tex_subset_dimensions:Option< TextureSubsetDimensions > = None  ;


                     let uv_affine_xform = match    tex_subset_dimensions {

                        Some( tex_subset_dimensions ) =>  tex_subset_dimensions.to_affine( uv_scale_factor  ), 
                        None => Affine2::from_scale(Vec2::splat( uv_scale_factor))  

                     };
 



                // Modify the UV transform as needed
                standard_material.uv_transform = uv_affine_xform.clone() ;
                
                // You can modify other properties here as well
                standard_material.perceptual_roughness = 0.5;
            }

			return Ok(Ok( loaded ));
		}

		Ok(Err(deserializer))
	}
}



// ----- 



// taken from  https://github.com/Noxmore/bevy_materialize/blob/main/src/load/simple.rs 

 
/*#[derive(Debug, Clone)]
pub struct CustomMaterialLoader {
  
	pub material: fn(Handle<Image>) -> Box<dyn ErasedMaterial>,



	pub properties: fn() -> HashMap<String, Box<dyn Reflect>>,  //custom bevy_materialize properties 
}
impl AssetLoader for CustomMaterialLoader {
	type Asset = GenericMaterial;
	type Settings = ();
	type Error = Infallible;

	fn load(
		&self,
		_reader: &mut dyn bevy::asset::io::Reader,
		#[allow(unused)] settings: &Self::Settings,
		#[allow(unused)] load_context: &mut LoadContext,
	) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
		Box::pin(async move {
	 
			let path = load_context.asset_path().clone();

			 
			let material = (self.material)(load_context.load(path));





			Ok(GenericMaterial {
			 
				handle: material.add_labeled_asset(load_context, "Material".to_string()),
				properties: (self.properties)(),
			})
		})
	}

	 
	fn extensions(&self) -> &[&str] {
		 

		&[
			"custommaterial" 
		]

	}
 
}

impl Default for CustomMaterialLoader {
	fn default() -> Self {
		Self {
			 
			material: |image| {
				StandardMaterial {
					base_color_texture: Some(image),
					perceptual_roughness: 1.,
					..default()
				}
				.into()
			},
			properties: HashMap::default,
		}
	}
}*/