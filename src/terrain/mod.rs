

use bevy::prelude::*; 

pub(crate) mod terrain_manifest;
pub (crate) mod terrain_loading;
pub (crate) mod terrain_generation;


 


pub fn terrain_plugin(app: &mut App){
 
	app
	 	.add_plugins( terrain_generation::terrain_generation_plugin )
	;

}

