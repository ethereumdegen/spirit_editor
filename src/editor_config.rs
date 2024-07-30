
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

 



#[derive( Asset, Serialize,Deserialize,Clone,Debug )]
pub struct EditorConfig {


	initial_terrain_to_load: Option<String> ,

	initial_zones_to_load: Option<Vec<String>>,

//	doodad_manifest: String, 

}


impl TypePath for EditorConfig {
    fn short_type_path() -> &'static str {
        "editorconfig.ron"
    }
    fn type_path() -> &'static str {
        "editorconfig.ron"
    }
}




impl EditorConfig{


 

	pub fn get_initial_terrain_path_full(&self) -> Option<String> { 

		return self.initial_terrain_to_load.as_ref().map(|t| format!("assets/terrain/{}/terrain_config.ron", t)  )
	}

    pub fn get_initial_zones_to_load(&self) -> Option<Vec<String>> { 

		return self.initial_zones_to_load.as_ref().map(|zone_names|   
			zone_names.iter().map(|z|  format!("{}", z)   ).collect()
		 )
	}


 
}



