
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::loading::EditorLoadingState;



pub fn material_overrides_plugin(app: &mut App) {
    app 	

    	.init_resource::<MaterialOverridesResource>()
    	.init_state::<MaterialOverridesLoadingState>()
       .add_systems(OnEnter(EditorLoadingState::LoadMaterialOverrides), load_material_overrides)
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


}




//attach this to signal that the material is supposed to be replaced 
#[derive(Component)]
pub struct MaterialOverrideRequestComponent<M: Material> {


	// material node name => material handle 
	material_overrides: HashMap< MaterialOverrideLayer , Handle<M > >
}



#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum MaterialOverrideLayer {

	Base

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

     material_overrides_resource: Res<MaterialOverridesResource>,

	 
){




	next_state.set(MaterialOverridesLoadingState::Complete);

}