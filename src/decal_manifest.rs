
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

 



#[derive( Asset, Serialize,Deserialize,Clone,Debug )]
pub struct DecalManifest {


	pub diffuse_texture:  String, 
	pub base_color: LinearRgba ,

 

}



impl TypePath for DecalManifest {
    fn short_type_path() -> &'static str {
        "decal.ron"
    }
    fn type_path() -> &'static str {
        "decal.ron"
    }
}


  