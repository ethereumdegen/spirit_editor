
use bevy::prelude::*; 



pub fn loading_plugin(app: &mut App) {
    app 
        .init_state::<EditorLoadingState>()
    	.add_systems(Update, update_loading_state.run_if(not(in_state(EditorLoadingState::Complete))))
    ;

}



#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum EditorLoadingState{
	#[default]
   Init,
   LoadMaterialOverrides,
   Complete
}


fn update_loading_state(
	state: Res<State<EditorLoadingState>>,
    mut next_state: ResMut<NextState<EditorLoadingState>>,

	){

	if *state == EditorLoadingState::Init{

		next_state.set( EditorLoadingState::LoadMaterialOverrides ) ;
	
	}else if *state == EditorLoadingState::LoadMaterialOverrides {

		//if the stuff is loaded....


		next_state.set( EditorLoadingState::Complete ) ;

	}


}