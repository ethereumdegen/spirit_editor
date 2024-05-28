
use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct EditorConfig {


	terrain_path: Option<String> ,

}


impl EditorConfig{


	//this is a super nice way to read a manifest file ! no need to wait for bevy asset server 
	pub fn load (

	     
	) -> Self{

	 //load the ron file 

	  let ron_str = include_str!("../assets/editor_config.editorconfig.ron");

	    // Parse the .ron string into the Config struct
	 
	   	let   editor_config  = ron::de::from_str::<EditorConfig>(ron_str).expect("Failed to parse RON file");


	   	editor_config
	}


	pub fn get_terrain_path_full(&self) -> Option<String> {



		return self.terrain_path.as_ref().map(|t| format!("assets/terrain/{}/terrain_config.ron", t)  )
	}



}



