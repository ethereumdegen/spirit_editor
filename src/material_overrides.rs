
use bevy::prelude::*;
use bevy::utils::HashMap;



pub fn material_overrides_plugin(app: &mut App) {
    app 	


      //  .add_systems(Update, update_camera_look)
      //  .add_systems(Update, update_camera_move)



       ;
}




#[derive(Component)]
pub struct MaterialOverridesResource {


}




//attach this to signal that the material is supposed to be replaced 
#[derive(Component)]
pub struct MaterialOverrideRequestComponent {


	// material node name => material handle 
	material_overrides: HashMap< MaterialOverrideLayer , Handle<Material> >
}



#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum MaterialOverrideLayer {

	Base

}