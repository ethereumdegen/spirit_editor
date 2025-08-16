

use bevy::prelude::*; 

pub(crate) mod terrain_manifest;
pub (crate) mod terrain_loading;
pub (crate) mod terrain_generation;
pub (crate) mod terrain_colliders; 

 


pub fn terrain_plugin(app: &mut App){
 
	app
	 	.add_plugins( terrain_generation::terrain_generation_plugin )

	 	.add_plugins( terrain_manifest::terrain_manifest_plugin)
        .add_plugins( terrain_loading::terrain_loading_plugin)
        .add_plugins( terrain_colliders:: terrain_colliders_plugin  )
      

	;

}

