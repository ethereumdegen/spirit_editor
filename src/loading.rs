
use bevy::prelude::*;

//use crate::material_overrides::MaterialOverridesLoadingState; 
 
//use bevy_material_wizard::material_overrides::MaterialOverridesLoadingState;


pub fn loading_plugin(app: &mut App) {
    app 
        .init_state::<EditorLoadingState>()
    	.add_systems(Update, update_loading_state.run_if(not(in_state(EditorLoadingState::Complete))))
    ;

}


//not rly necessary ? 

#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum EditorLoadingState{
	#[default]
   Init,
   //LoadMaterialOverrides,
   Complete
}


fn update_loading_state(
	editor_load_state: Res<State<EditorLoadingState>>,
	//material_overrides_load_state: Res<State<MaterialOverridesLoadingState>>,
	//mut next_material_overrides_load_state: ResMut<NextState<MaterialOverridesLoadingState>>,
    mut next_load_state: ResMut<NextState<EditorLoadingState>>,

	){

	if *editor_load_state == EditorLoadingState::Init{

		next_load_state.set( EditorLoadingState::Complete ) ;
	
	 }


}