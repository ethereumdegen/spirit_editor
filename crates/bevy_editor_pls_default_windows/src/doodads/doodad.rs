

use bevy::{prelude::*, utils::HashMap};

use super::doodad_manifest::{DoodadDefinition, RenderableType};

use anyhow::{Result,Context }; 

use bevy_mod_sysfail::*;

use bevy_mod_picking::prelude::*;

//use bevy_mod_picking::prelude::{PickRaycastTarget, PickableBundle};

use bevy::{
     gltf::{Gltf, GltfMesh, GltfNode},
     scene::SceneInstanceReady,
};


#[derive(Default)]
pub(crate) struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(LoadedGltfAssets::default())

        .add_systems(
            Update,
            attach_models_to_doodads  
        )
        ;
       
    }
}

#[derive(Resource,Default)]
pub struct LoadedGltfAssets {

	pub gltf_models: HashMap<String, Handle<Gltf>>

}
 

#[derive(Component, Debug)]
pub struct DoodadComponent {
		pub definition:  DoodadDefinition
}


impl DoodadComponent {


	pub fn from_definition( definition: &DoodadDefinition) -> Self {


		Self {

			definition: definition.clone()


		}

	}

}


#[sysfail]
fn attach_models_to_doodads(

	 mut commands: Commands,
    added_doodad_query: Query<
        (Entity, &DoodadComponent),
         (
            Added<DoodadComponent>,
            With<GlobalTransform>,
            Without<Handle<Mesh>>,
        ),
    >,

      models: Res<Assets<Gltf>>,
     gltf_assets: Res<LoadedGltfAssets>,

      mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

	){
  #[cfg(feature = "tracing")]
    let _span = info_span!("add_model_to_doodads").entered();

		for (new_doodad_entity, doodad_component) in added_doodad_query.iter() {

			  let doodad_name = &doodad_component.definition.name;
				

	            let doodad_name_clone = doodad_name.clone();
	            let name_comp = Name::new(doodad_name_clone);
	        	

			    commands
	            .entity(new_doodad_entity)
	            
	            .insert(  name_comp )
	            .insert(  PickableBundle::default() )
	            ;



          //handle attaching renderable components based on the renderable type - this lets us see the doodad in the editor 
		   match (&doodad_component.definition.model).clone() {

				RenderableType::GltfModel(model_name) =>  {

 

					 let model_handle = gltf_assets
			           .gltf_models
			            .get(model_name.as_str())
			            .context(format!(" no doodad model registered at "))?;

			        let loaded_model = models
			            .get(model_handle)
			            .context(format!("Could not load model handle for {}", model_name))?; 

			        	 
			          commands
			            .entity(new_doodad_entity)
			            .insert(
			                loaded_model.named_scenes["Scene"].clone(), //add the scene.. the mesh   but we assume the transform is alrdy there
			            )  ;



				} 
				RenderableType::CubeShape(cube_shape_def) =>  {
						  commands
			            .entity(new_doodad_entity)
			            .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0) ) )
			            .insert(materials.add( cube_shape_def.color.clone() )) 
			            	 ;


				}

			};
 
	           

		}

}


