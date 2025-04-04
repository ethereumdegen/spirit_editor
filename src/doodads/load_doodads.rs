
 
use crate::EditorConfig;
 
use crate::asset_loading::EditorConfigAssets;
use crate::AssetLoadState;
use bevy::prelude::*;
 



 //this is causing stack overflow ? 


#[derive(Default)]
pub(crate) struct DoodadLoadPlugin;

impl Plugin for DoodadLoadPlugin {
    fn build(&self, app: &mut App) {
        app ;
           //.add_systems(OnEnter(AssetLoadState::Complete), load_doodad_manifest)
         //  .add_systems(Update, build_doodad_data_from_manifest  );

    }
}



  




/*
fn load_doodad_manifest(
    asset_server: Res<AssetServer>,

    editor_config_res: Res<EditorConfigAssets>,
    editor_config_assets: Res<Assets<EditorConfig>>, 

    mut doodad_manifest_resource: ResMut<DoodadManifestResource>,
) {


    //"doodad_manifest.doodadmanifest.ron"
    let editor_config_handle = &editor_config_res.editor_config;

    let  editor_config  = editor_config_assets.get(editor_config_handle).expect("Could not load doodad manifest. Is it specified properly in editor config?");

    let doodad_manifest_path = editor_config.get_doodad_manifest_path();

    doodad_manifest_resource.manifest = Some(asset_server.load(doodad_manifest_path));
    info!("loading doodad manifest");
 
}
*/

/*
fn build_doodad_data_from_manifest(
    mut evt_asset: EventReader<AssetEvent<DoodadManifest>>,
    doodad_manifest_resource: Res<DoodadManifestResource>,

    mut doodad_tag_map_resource: ResMut<DoodadTagMapResource>, 
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,

   // mut loaded_gltf_resource: ResMut< GltfAssets>,

    //asset_server: ResMut<AssetServer>,
) {

 
    let Some(doodad_manifest_handle) = &doodad_manifest_resource.manifest else {
        return;
    };




    for evt in evt_asset.read() {
        match evt {
            AssetEvent::LoadedWithDependencies { id } => {

                 info!("build_doodad_data_from_manifest 2");


                if id == &doodad_manifest_handle.id() {
                    let manifest: &DoodadManifest = doodad_manifest_assets
                        .get(doodad_manifest_handle.id())
                        .unwrap();

                    println!(" building doodad data  ");

                   

                    //now that our manifest is loaded, lets populate the doodad tag map resource 
                    for (doodad_name,doodad_definition) in &manifest.doodad_definitions {

                        for tag in &doodad_definition.tags.clone().unwrap_or(Vec::new()){
                            doodad_tag_map_resource.doodad_tag_map.entry(tag.clone()).or_default().push(doodad_name.to_string());
                        }


                        doodad_tag_map_resource.doodad_tag_map.entry("all_doodads".to_string()).or_default().push(doodad_name.to_string());

                    }

                     // Sort tags and doodad names
                    info!("sorting doodad keys");
                    let mut sorted_keys: Vec<_> = doodad_tag_map_resource.doodad_tag_map.keys().cloned().collect();
                    sorted_keys.sort();
                    doodad_tag_map_resource.doodad_tag_map = sorted_keys.into_iter().map(|k| (k.clone(), doodad_tag_map_resource.doodad_tag_map.remove(&k).unwrap())).collect();
                    
                      for doodads in doodad_tag_map_resource.doodad_tag_map.values_mut() {
                        doodads.sort();
                    }
                      info!("sorted dooodad keys");
 

             

                }
            }
            _ => {}
        }
    }
}*/

//uses the heap more ? 
fn build_doodad_data_from_manifest(
    mut evt_asset: EventReader<AssetEvent<DoodadManifest>>,
    doodad_manifest_resource: Res<DoodadManifestResource>,
    mut doodad_tag_map_resource: ResMut<DoodadTagMapResource>, 
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,
) {
    // Early return if there is no manifest handle
    let Some(doodad_manifest_handle) = &doodad_manifest_resource.manifest else {
        return;
    };

    for evt in evt_asset.read() {
        match evt {
            AssetEvent::LoadedWithDependencies { id } => {
                if id == &doodad_manifest_handle.id() {
                    let manifest: &DoodadManifest = doodad_manifest_assets
                        .get(doodad_manifest_handle.id())
                        .unwrap();

                    println!("Building doodad data");

                    // Now that our manifest is loaded, populate the doodad tag map resource.
                    // Boxed HashMap moves this structure to the heap.
                    let mut temp_tag_map: Box<HashMap<String, Vec<String>>> = Box::new(HashMap::new());

                    for (doodad_name, doodad_definition) in &manifest.doodad_definitions {
                        // We avoid cloning the entire tags vector and instead only clone the individual tags.
                        if let Some(tags) = &doodad_definition.tags {
                            for tag in tags {
                                temp_tag_map
                                    .entry(tag.clone())
                                    .or_default()
                                    .push(doodad_name.to_string());
                            }
                        }
                        
                        // Ensure that all doodads are stored under the "all_doodads" tag
                        temp_tag_map
                            .entry("all_doodads".to_string())
                            .or_default()
                            .push(doodad_name.to_string());
                    }

                    // Sort tags and doodad names
                    info!("Sorting doodad keys");

                    // Box the sorted keys to move them to the heap
                    let mut sorted_keys: Box<Vec<String>> = Box::new(temp_tag_map.keys().cloned().collect());
                    sorted_keys.sort();

                    // Rebuild the final tag map in doodad_tag_map_resource
                    doodad_tag_map_resource.doodad_tag_map = sorted_keys
                        .into_iter()
                        .map(|k| (k.clone(), temp_tag_map.remove(&k).unwrap()))
                        .collect();

                    // Sort doodads under each tag
                    for doodads in doodad_tag_map_resource.doodad_tag_map.values_mut() {
                        doodads.sort();
                    }

                    info!("Sorted doodad keys");
                }
            }
            _ => {}
        }
    }
}