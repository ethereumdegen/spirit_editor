

 

use bevy::{asset::{LoadedUntypedAsset, ReflectHandle}, prelude::Handle};
use bevy_materialize::load::processor::MaterialProcessorContext;
use bevy::{prelude::PartialReflect, reflect::{TypeRegistration, TypeRegistry}};
use bevy_materialize::load::processor::MaterialProcessor;

#[derive(Clone)]
pub struct CustomMaterialProcessor<P: MaterialProcessor>(pub P);

impl<P: MaterialProcessor> MaterialProcessor for CustomMaterialProcessor<P>  {
 

  
	type Child = Self;


    fn child(&self) -> Option<&Self::Child> {
        None 
    }


	fn try_deserialize<'de, D: serde::Deserializer<'de>>(
		&self,
		ctx: &mut MaterialProcessorContext,
		registration: &TypeRegistration,
		registry: &TypeRegistry,
		deserializer: D,
	) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error> {

	    // ??? 

	} 

	fn try_deserialize_recursive<'de, D: serde::Deserializer<'de>>(
		&self,
		ctx: &mut MaterialProcessorContext,
		registration: &TypeRegistration,
		registry: &TypeRegistry,
		deserializer: D,
	) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error> {
		if let Some(child) = self.child() {
			match child.try_deserialize_recursive(ctx, registration, registry, deserializer) {
				Ok(Err(returned_deserializer)) => self.try_deserialize(ctx, registration, registry, returned_deserializer),
				out => out,
			}
		} else {
			Ok(Err(deserializer))
		}
	}




}