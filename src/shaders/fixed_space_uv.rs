

use crate::shaders::fixed_space_uv_material::build_fixed_space_uv_material;
use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterial;
use bevy_material_wizard::material_definition::MaterialDefinitionsMap;
use bevy::prelude::*; 


use bevy_material_wizard::material_overrides::MaterialOverrideCompleted;

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

	    	cmd.queue( UpgradeToMagicRockExtensionMaterial );
	    }



	}else {


		 if let Some(mut cmd) = commands.get_entity( material_override_entity ) {

	    	cmd.queue( DowngradeToStandardMaterial );
	    }

	}



}


pub struct DowngradeToStandardMaterial ;


impl EntityCommand for DowngradeToStandardMaterial {

  		fn apply(self, mat_entity: Entity, world: &mut World) { 
				if let Some(extension_mat_handle) = world.get::< MeshMaterial3d<FixedSpaceUvMaterial> >(mat_entity){
  
						  			let   character_material_assets = world.resource ::< Assets<FixedSpaceUvMaterial> >();

						  				if	let Some(_ext_mat) = character_material_assets.get( extension_mat_handle ){

						  				  // 	let inner_base_material = ext_mat.base.clone(); 


								  			 if let Some(mut cmd) = world.commands().get_entity(mat_entity) {

									  			  //   	let std_mat = extension_mat.0 ;

									  			 	 cmd
									  			 //	 .insert( MeshMaterial3d( inner_base_material ) )
									  			 	 .remove::<MeshMaterial3d<FixedSpaceUvMaterial>>()   ;  
									  		 } 

							  		}
		            }
		 }

}


pub struct UpgradeToMagicRockExtensionMaterial ;

impl EntityCommand for UpgradeToMagicRockExtensionMaterial {



		fn apply(self, mat_entity: Entity, world: &mut World) { 

		//	let ext_material_type = world.get::<ExtensionMaterialType>(mat_entity);



			let Some(original_mesh_material_component) = world.get::<MeshMaterial3d<StandardMaterial>>( mat_entity ) else {
   
				return

			};

			let original_mesh_material_handle = original_mesh_material_component .0.clone() ;


 
			let standard_material_assets = world.resource::< Assets<StandardMaterial> > ();


			
				//how can i make this generic like with  ::T ?
			let Some(original_material) = standard_material_assets.get( &original_mesh_material_handle  ) else {return};
			let char_mat = build_fixed_space_uv_material(original_material.clone());



			let mut character_material_assets = world.resource_mut::< Assets<FixedSpaceUvMaterial> >();
            let char_mat_handle = character_material_assets.add(char_mat.clone());


				  if let Some(mut cmd) = world.commands().get_entity(mat_entity) {
                cmd.remove::<MeshMaterial3d<StandardMaterial>>() 
                .insert(MeshMaterial3d(char_mat_handle)) ;

                info!( "refreshed linked ext materials !" );

            }




		}
}




