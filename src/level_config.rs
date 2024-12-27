

use bevy::prelude::*;
use serde::{Deserialize, Serialize};



#[derive( Asset, Serialize,Deserialize,Clone,Debug )]
pub struct LevelConfig {


	initial_terrain_to_load: Option<String> ,

	initial_zones_to_load: Option<Vec<String>>,

	initial_foliage_scene_to_load: Option<String>, 


}






impl TypePath for LevelConfig {
    fn short_type_path() -> &'static str {
        "level.ron"
    }
    fn type_path() -> &'static str {
        "level.ron"
    }
}




impl LevelConfig{

 
	pub fn get_initial_terrain_path_full(&self) -> Option<String> { 

		return self.initial_terrain_to_load.as_ref().map(|t| format!("assets/terrain/{}/terrain_config.ron", t)  )
	}

    pub fn get_initial_zones_to_load(&self) -> Option<Vec<String>> { 

		return self.initial_zones_to_load.as_ref().map(|zone_names|   
			zone_names.iter().map(|z|  format!("{}", z)   ).collect()
		 )
	}

	pub fn get_foliage_scene_name(&self) -> Option<String> {


		return self.initial_foliage_scene_to_load.clone()
	}

 
 
}

