
/*


DEPRECATED !!! Use bevy_material_wizard and  materialize for this now ! 

*/

use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterialSidesOnly;
use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterialBase;
  
//use bevy_material_wizard::material_definition::MaterialDefinitionsMap;
use bevy::prelude::*; 
use std::marker::PhantomData;

use bevy_material_wizard::material_overrides::MaterialOverrideCompleted;

use bevy::pbr::{ExtendedMaterial,MaterialExtension}; 

/*

Listen for trigger of material override occuring 

when  an override is complete, look up the corresponding matdef file 

if it has a custom prop of  MagicRockShader,  we are going to upgrade to an extension material- magic rock shader ! 



*/

pub fn fixed_space_uv_plugin(app: &mut App) {
    app
      .add_observer( handle_material_override_performed  )
       ;
}




fn handle_material_override_performed (

	trigger: Trigger< MaterialOverrideCompleted >,

	 material_definitions_res: Res<MaterialDefinitionsMap>,

 
   mut commands : Commands, 



) {


	let material_override_entity = trigger.entity();

	let material_def_name = &trigger.0; 



	let Some(material_def) = material_definitions_res.material_definitions.get( material_def_name ) else {return};

	if material_def.custom_props.contains( "FixedSpaceUvShader" ) {


		// need to upgrade the material into an extension here 


		 if let Some(mut cmd) = commands.get_entity( material_override_entity ) {

	    	cmd.queue( UpgradeToExtensionMaterial::< FixedSpaceUvMaterialBase > ::default() );
	    }



	 }else if material_def.custom_props.contains( "FixedSpaceUvSideShader" ) {  



	  	if let Some(mut cmd) = commands.get_entity( material_override_entity ) {

	    	cmd.queue( UpgradeToExtensionMaterial::< FixedSpaceUvMaterialSidesOnly > ::default() );
	    }


	}else {


		 if let Some(mut cmd) = commands.get_entity( material_override_entity ) {
		 		//this is janky but it works ...! 
	    	cmd.queue( DowngradeToStandardMaterial::< FixedSpaceUvMaterialBase > ::default() );
	    	cmd.queue( DowngradeToStandardMaterial::< FixedSpaceUvMaterialSidesOnly > ::default() );
	    }

	}



}


#[derive(Default)]
pub struct DowngradeToStandardMaterial<T : MaterialExtension>{
    _marker: PhantomData<T>, // Helps the compiler know T is used
 
}

impl<T:  MaterialExtension> EntityCommand for DowngradeToStandardMaterial<T> {

  		fn apply(self, mat_entity: Entity, world: &mut World) { 
				if let Some(extension_mat_handle) = world.get::< MeshMaterial3d< ExtendedMaterial<StandardMaterial, T > > >(mat_entity){
  
						  			let ext_material_assets = world.resource ::< Assets< ExtendedMaterial<StandardMaterial, T > > >();

						  				if	let Some(_ext_mat) = ext_material_assets.get( extension_mat_handle ){
 

								  			 if let Some(mut cmd) = world.commands().get_entity(mat_entity) { 

									  			 	 cmd 
									  			 	 .remove::<MeshMaterial3d< ExtendedMaterial<StandardMaterial, T > >>()   ;  
									  		 } 

							  		}
		            }
		 }

}

#[derive(Default)]
pub struct UpgradeToExtensionMaterial <T : MaterialExtension + Default >{
    _marker: PhantomData<T>, // Helps the compiler know T is used
 
}

impl<T:  MaterialExtension + Default >  EntityCommand for UpgradeToExtensionMaterial<T> {



		fn apply(self, mat_entity: Entity, world: &mut World) { 





			let Some(original_mesh_material_component) = world.get::<MeshMaterial3d<StandardMaterial>>( mat_entity ) else {
   
				return

			};

			let original_mesh_material_handle = original_mesh_material_component .0.clone() ;


 
			let standard_material_assets = world.resource::< Assets<StandardMaterial> > ();

 
			let Some(original_material) = standard_material_assets.get( &original_mesh_material_handle  ) else {return};
		
	 
		 let ext_mat =  ExtendedMaterial {
	        base: original_material.clone(),  
	        extension: T::default(),
	    };

			let mut ext_material_assets = world.resource_mut::< Assets< ExtendedMaterial<StandardMaterial, T >  > >();
            let ext_mat_handle = ext_material_assets.add(ext_mat.clone());


				  if let Some(mut cmd) = world.commands().get_entity(mat_entity) {
                cmd.remove::<MeshMaterial3d<StandardMaterial>>() 
                .insert(MeshMaterial3d(ext_mat_handle)) ;

                info!( "refreshed linked ext materials !" );

            }




		}
}




