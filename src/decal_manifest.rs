
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

 



#[derive( Asset, Serialize,Deserialize,Clone,Debug )]
pub struct DecalManifest {

    pub diffuse_texture:  String, 
    pub base_color: LinearRgba ,

    pub emissive_texture: Option<String>,
    pub emissive_color: Option<LinearRgba>,

    //must be false for emissive to work ! 
    #[serde(default)]
    pub unlit:bool,
}



impl TypePath for DecalManifest {
    fn short_type_path() -> &'static str {
        "decal.ron"
    }
    fn type_path() -> &'static str {
        "decal.ron"
    }
}


  