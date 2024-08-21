
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::loading::EditorLoadingState;



pub fn material_overrides_plugin(app: &mut App) {
    app 	

    	.init_resource::<MaterialOverridesResource>()
    	.init_state::<MaterialOverridesLoadingState>()
       .add_systems(OnEnter(EditorLoadingState::LoadMaterialOverrides), load_material_overrides)

       .add_systems(Update, extract_material_overrides )

      //  .add_systems(Update, update_camera_move)



       ;
}



#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum MaterialOverridesLoadingState{
	#[default]
   Init,
   Extracting,
   Complete
}




#[derive(Resource,Default)]
pub struct MaterialOverridesResource {

	doodad_materials_gltf: Option<Handle<Gltf>>,

	extracted_materials_map :HashMap< String, Handle<StandardMaterial> >


}




//attach this to signal that the material is supposed to be replaced 
#[derive(Component)]
pub struct MaterialOverrideRequestComponent {


	// material node name => material name
	material_overrides: HashMap< MaterialOverrideLayer , MaterialOverrideType >
}



#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum MaterialOverrideLayer {

	Base

}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum MaterialOverrideType {
	Stone1,
	Stone2,
	Stone3, 
}

impl MaterialOverrideType {

	fn get_material_name(&self) -> String {


		match &self {
			Self::Stone1 => "Stone1",
			Self::Stone2 => "Stone2",
			Self::Stone3 => "Stone3"
		}.into()

	}

}

fn load_material_overrides(

	  asset_server: ResMut<AssetServer> ,

	mut material_overrides_resource: ResMut<MaterialOverridesResource>,

	mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,


){	

	let material_overrides_path = "material_overrides/doodad_material_overrides.glb";

	let doodad_materials_gltf = asset_server.load::<Gltf>( material_overrides_path  );

	material_overrides_resource.doodad_materials_gltf = Some(doodad_materials_gltf);

	next_state.set(MaterialOverridesLoadingState::Extracting);


}


fn extract_material_overrides(
	 mut asset_ready_event: EventReader<AssetEvent<Gltf>>,
     mut material_overrides_resource: ResMut<MaterialOverridesResource>,
     mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,

     gltf_assets: Res<Assets<Gltf>>,

	 
){

	for evt in asset_ready_event.read(){
		match evt {
   
			    AssetEvent::LoadedWithDependencies { id } => {
			    	if material_overrides_resource.doodad_materials_gltf.as_ref().is_some_and(|h| h.id() == *id ){

			    		let Some(doodad_materials_gltf) = gltf_assets.get( *id ) else {continue};

			    		for (material_name, material_handle) in &doodad_materials_gltf.named_materials {
			    			info!("extracted override material: {}", material_name.to_string());
			    			material_overrides_resource.extracted_materials_map.insert(material_name.to_string(), material_handle.clone());
			    		}


			    		next_state.set(MaterialOverridesLoadingState::Complete);

			    	}
			    }

			    _ => {}
			}



	}


	

}