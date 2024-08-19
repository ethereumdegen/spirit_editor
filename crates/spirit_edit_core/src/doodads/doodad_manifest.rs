use crate::zones::zone_file::CustomPropsMap;
use bevy::utils::HashMap;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
 

pub type DoodadName = String;

#[derive(Resource, Default)]
pub struct DoodadDefinitionsResource {
    pub loaded_doodad_definitions: Option< HashMap<DoodadName,DoodadDefinition> >,
 
}

impl DoodadDefinitionsResource {

    pub fn get_doodad_definition_by_name(&self, doodad_name: &String) -> Option<&DoodadDefinition>{

        return self.loaded_doodad_definitions.as_ref().map(|d| d.get(doodad_name)).flatten()
    }
}

#[derive(Resource, Default)]
pub struct DoodadTagMapResource {
    
    pub doodad_tag_map: HashMap< String, Vec<DoodadName>  >
}


#[derive(Asset,  Clone, Debug, Serialize, Deserialize)]
pub struct DoodadManifest {
  //  pub doodad_tags: Vec<String>,
    pub doodad_definitions: HashMap<DoodadName,DoodadDefinition>,
}


impl TypePath for DoodadManifest {
    fn short_type_path() -> &'static str {
        "doodadmanifest.ron"
    }
    fn type_path() -> &'static str {
        "doodadmanifest.ron"
    }
}



impl DoodadManifest {
    pub fn get_doodad_definition_by_name(&self, name: &str) -> Option<DoodadDefinition> {
     

        return self.doodad_definitions.get(name).cloned();

        
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RenderableType {
    GltfModel(String), //the path
    CubeShape(CubeShapeDefinition),
    MagicFx(String),
    LiquidPlane(String)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeShapeDefinition {
    pub color: LinearRgba,
    pub wireframe: bool 
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DoodadDefinition {
   // pub name: String,
    pub model: RenderableType,
    pub initial_custom_props: Option<CustomPropsMap>,
    pub tags: Option<Vec<String>> ,
    pub snap_dimensions: Option<Vec2>, 
}
/*
impl DoodadManifest {
    pub fn load(file_path:&String) -> Result<Self, Box<dyn std::error::Error>> {
        //let file_path = get_doodad_manifest_file_path();
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = ron::de::from_str(&contents)?;
        Ok(data)
    }
}


fn get_doodad_manifest_file_path() -> String {
    format!("assets/doodad_manifest.ron")
}
*/