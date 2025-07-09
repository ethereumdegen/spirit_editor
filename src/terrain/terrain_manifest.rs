use bevy::{asset::ReflectAsset, reflect::TypeRegistry};

use bevy::prelude::*;
 

use bevy::platform::collections::hash_map::HashMap; 
use serde::{Serialize,Deserialize} ;

 
use bevy_common_assets::ron::RonAssetPlugin;
 

pub fn terrain_manifest_plugin(app: &mut App) {


     app.add_plugins(RonAssetPlugin::<TerrainManifest>::new(&["terrainmanifest.ron"]))
            
            .insert_resource(TerrainManifestResource::default())
           
            .add_systems(Startup, load_terrain_manifest)
             ;
 
}
 
 
   

// --------------------------------------------------------

fn load_terrain_manifest(
    asset_server: Res<AssetServer>,
    mut terrain_manifest_resource: ResMut<TerrainManifestResource>,
) {
    terrain_manifest_resource.manifest = Some(asset_server.load("terrain_manifest.terrainmanifest.ron"));
}
  




#[derive(Resource, Default)]
pub struct TerrainManifestResource {
    pub manifest: Option<Handle<TerrainManifest>>,
}


 


#[derive(Asset, TypePath, Clone, Debug, Serialize, Deserialize)]
pub struct TerrainManifest {
    pub terrain_definitions: Vec<TerrainDefinition>,
}

impl TerrainManifest {


	pub fn get_terrain_type(&self, index: u16) -> Option<&TerrainDefinition> {

		self.terrain_definitions.get(index as usize)
	}
}


/*
impl TerrainManifest {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = get_save_file_path();
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = ron::de::from_str(&contents)?;
        Ok(data)
    }
}*/


#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct TerrainDefinition {
    pub name: String, 
}


/*
impl TerrainManifest {
    pub fn get_terrain_definition_by_name(&self, name: &str) -> Option<TerrainDefinition> {
        //maybe use a hashmap for this ?
        for doodad_definition in &self.doodad_definitions {
            if doodad_definition.name == name {
                return Some(doodad_definition.clone());
            }
        }

        None
    }
}*/