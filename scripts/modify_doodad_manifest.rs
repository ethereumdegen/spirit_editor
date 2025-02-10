use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use spirit_edit_core::doodads::doodad_manifest::RenderableType;
use std::collections::HashMap;
use std::fs;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spawnable {
    pub name: String , 

    pub model: RenderableType,
   pub material_replacement_set: Option<String> ,
   pub tags: Option<Vec<String> > ,
}
 

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoodadManifest {
    pub spawnables: HashMap<String, Spawnable>,
}
 

 fn read_ron_file(file_path: &str) -> Result<DoodadManifest, ron::Error> {
    let contents = fs::read_to_string(file_path)?;

    println!("{:?}", contents );
   let output = from_str(&contents).unwrap();


   Ok(output)


}

/*
fn read_ron_file(file_path: &str) -> Result<Value, ron::Error> {



    // Deserialize the JSON string into serde_json::Value
  


    let contents = fs::read_to_string(file_path)?;

      let v: Value = serde_json::from_str(&contents).unwrap();

      Ok(v)
   // from_str(&contents)?
}*/

 


fn write_ron_file(manifest: &DoodadManifest, file_path: &str) -> Result<(), std::io::Error> {
    let pretty = PrettyConfig::new()
       // .depth_limit(2)
       // .separate_tuple_members(true) 
       // .enumerate_arrays(true)
       ;
    let serialized = to_string_pretty(manifest, pretty).unwrap();
    fs::write(file_path, serialized)
}

fn main() {
    let file_path = "artifacts/game_assets/doodad_manifests/doodad_manifest_dreamscape.doodadmanifest.ron";
    let mut manifest = read_ron_file(file_path).expect("Failed to load RON file");

  

      // Iterate over the spawnables and modify each one
    for (_key, spawnable) in manifest.spawnables.iter_mut() {
        // Check if the spawnable is an object and insert a new field
        spawnable.material_replacement_set = Some( "dreamscape_castle".to_string() );

         
    } 

  
    let output_file_path = "artifacts/game_assets/doodad_manifests/doodad_manifest_dreamscape_2.doodadmanifest.ron";
    write_ron_file(&manifest,  output_file_path ).expect("Failed to save RON file");
}
