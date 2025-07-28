
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

 
use bevy::platform::collections::hash_map::HashMap;






#[derive( Asset, Serialize,Deserialize,Clone,Debug, Resource  )]
pub struct EditorConfig {
	external_game_assets_folder: Option<String>,



//	doodad_manifest: String, 
	

	initial_level_to_load: Option<String>, 



    default_placement_settings: Option<EditorConfigPlacementSettings>,

 

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


 	pub fn get_external_game_assets_folder(&self) -> Option<&String> { 

		return self.external_game_assets_folder.as_ref()
	}


	pub fn get_initial_level_name(&self) -> Option<String> {

		return self.initial_level_to_load.clone()
	}

	pub fn load_from_path(  path: &str   ) -> Option<Self> {

		 

		 // Attempt to read the manifest file
        let file_content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => { 

            	warn!("{:?}", e );
	            return None 
	        }
        };
        
        // Parse the RON file
        let manifest: Self = match ron::from_str(&file_content) {
            Ok(parsed) => parsed,
            Err( e ) => {

            	warn!("{:?}", e );
            	return None
            },
        };
        
       


		Some( manifest  )


	}





	/*pub fn get_initial_terrain_path_full(&self) -> Option<String> { 

		return self.initial_terrain_to_load.as_ref().map(|t| format!("assets/terrain/{}/terrain_config.ron", t)  )
	}

    pub fn get_initial_zones_to_load(&self) -> Option<Vec<String>> { 

		return self.initial_zones_to_load.as_ref().map(|zone_names|   
			zone_names.iter().map(|z|  format!("{}", z)   ).collect()
		 )
	}

	pub fn get_foliage_scene_name(&self) -> Option<String> {


		return self.initial_foliage_scene_to_load.clone()
	}*/

	pub fn get_default_placement_settings(&self) -> Option<EditorConfigPlacementSettings> {


		return self.default_placement_settings.clone()
	}

 
}



pub fn load_editor_config (   world: &mut World  ) {

	let path =  "assets/editor_config.editorconfig.ron" ;

	world.insert_resource(

			EditorConfig::load_from_path( path ) .expect( "unable to load editor config file " ) 

	 );


}



#[derive(  Serialize,Deserialize,Clone,Debug )]
pub struct EditorConfigPlacementSettings{

	pub translation_grid_lock_step: Option<Vec3>,
}
