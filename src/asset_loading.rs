 
use crate::decal_manifest::DecalManifest;
use crate::level_config::LevelConfig;
use bevy_editor_pls_default_windows::placement::PlacementWindow;
use bevy_editor_pls_core::Editor;
use bevy_material_wizard::registered_materials::RegisteredMaterialsMap;
use bevy_materialize::generic_material::GenericMaterial;

use bevy::platform::collections::hash_map::HashMap; 

use crate::utils::copy_dir_recursive;
use std::path::Path;
use crate::utils::{walk_dir};
use spirit_edit_core::doodads::doodad_manifest::DoodadManifest;
use spirit_edit_core::doodads::doodad_manifest::DoodadDefinitionsResource;
use spirit_edit_core::doodads::doodad_manifest::DoodadTagMapResource;
use bevy_common_assets::ron::RonAssetPlugin;

use spirit_edit_core::prefabs::prefab_definitions::PrefabDefinition;
use spirit_edit_core::prefabs::prefab_definitions::PrefabDefinitionsResource ;
 
 
use crate::EditorConfig;
use bevy::gltf::Gltf;
use bevy_asset_loader::prelude::*; 
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy::{asset::{AssetPath, LoadedFolder}, prelude::* }; 
use bevy_magic_fx::{  magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest}  };


/*

Loads all of the MagicFx assets 

*/


pub fn asset_loading_plugin(app: &mut App) {
		    app


		      .init_state::<AssetLoadState>()
		      .init_resource::<BuiltVfxHandleRegistry>()




    
            .add_plugins(  bevy_obj::ObjPlugin  ) 
            .add_plugins(RonAssetPlugin::<EditorConfig>::new(&["editorconfig.ron"])) 
            .add_plugins(RonAssetPlugin::<LevelConfig>::new(&["level.ron"])) 

            .add_plugins(RonAssetPlugin::<DecalManifest>::new(&["decal.ron"])) 
           // .add_plugins(RonAssetPlugin::<DoodadManifest>::new(&["doodadmanifest.ron"])) //not needed ? 


            

            //  .add_systems(OnExit(AssetLoadState::Init), import_game_assets)


             .add_loading_state(
                    LoadingState::new(AssetLoadState::Init)
                        .continue_to_state(AssetLoadState::DoodadManifestsLoad)
                        .load_collection::<LevelAssets>() 

                          
                )

             
             // .add_systems(OnExit(AssetLoadState::Init), import_game_assets)

              .add_loading_state(
                    LoadingState::new(AssetLoadState::DoodadManifestsLoad)
                        .continue_to_state(AssetLoadState::TextureAssetsLoad)
                       
                        .load_collection::<DoodadManifestAssets>() 
                         
                )


		     .add_loading_state(
                    LoadingState::new(AssetLoadState::TextureAssetsLoad)
                        .continue_to_state(AssetLoadState::MeshAssetsLoad)
                        .load_collection::<TextureAssets>() 
 
                          
                )
                


                
             .add_loading_state(
                    LoadingState::new(AssetLoadState::MeshAssetsLoad)
                        .continue_to_state(AssetLoadState::DecalAssetsLoad)
                        .load_collection::<MeshAssets>() 
 
                
                )


             .add_loading_state(
                    LoadingState::new(AssetLoadState::DecalAssetsLoad)
                        .continue_to_state(AssetLoadState::ShaderAssetsLoad)
                        .load_collection::<DecalAssets>() 
 
                          
                )
                
 

              .add_loading_state(
                    LoadingState::new(AssetLoadState::ShaderAssetsLoad)
                        .continue_to_state(AssetLoadState::ShadersLoad)
                         
                        
                        //  .load_collection::<ShaderVariantAssets>() 
                          .load_collection::<MagicFxVariantAssets>()
                           //.load_collection::<AnimatedMaterialAssets>() 
                )


              // .add_systems(OnEnter(AssetLoadState::ShaderVariantsLoad), load_shader_variants)
              .add_systems(OnEnter(AssetLoadState::ShadersLoad), (
                
                populate_prefab_definitions,
                populate_doodad_definitions,
                populate_doodad_tag_map_data,
                 load_magic_fx,
                ).chain())


                .add_systems(OnEnter(AssetLoadState::Complete), (
                
                  apply_editor_config
                ).chain())
                 
              
         ;





                
      


 }

 #[derive(States,Hash,Eq,PartialEq,Debug,Clone,Default)]
pub enum AssetLoadState {
    #[default]
    Init, //editor config load 
    DoodadManifestsLoad,
    TextureAssetsLoad,
    MeshAssetsLoad,
    DecalAssetsLoad, 
    //GltfAssetsLoad,
    ShaderAssetsLoad,
    //ShaderVariantsLoad,
    ShadersLoad,
    Complete

}



// WHY IS THIS CRASHIGN WHEN LOADING PNG ??? 
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {

     #[asset(path = "../artifacts/game_assets/textures/decal_textures", collection(typed, mapped))]
    pub(crate) decal_textures: HashMap<AssetFileName, Handle<Image>>,
       

       //remove for now !   might want to preload these later idk 
   //  #[asset(path = "../artifacts/game_assets/textures/vfx_textures", collection(typed, mapped))]
 //   pub(crate) vfx_textures: HashMap<AssetFileName, Handle<Image>>,


}

#[derive(AssetCollection, Resource)]
pub struct  LevelAssets {
   
   //  #[asset(path = "editor_config.editorconfig.ron" )]
   // pub(crate) editor_config:   Handle<EditorConfig> ,

     #[asset(path = "levels", collection(typed, mapped))]
    pub(crate) levels:  HashMap<AssetFileStem, Handle<LevelConfig> > ,

    
}


 
#[derive(AssetCollection, Resource)]
pub struct DoodadManifestAssets {
   
    #[asset(path = "../artifacts/game_assets/doodad_manifests", collection(typed, mapped))]
    pub(crate) doodad_manifests: HashMap<String, Handle<DoodadManifest>>,
} 

/*
#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
   
     #[asset(path = "models/doodads", collection(typed, mapped))]
    pub(crate) doodad_models: HashMap<String, Handle<Gltf>>,


}
*/


#[derive(AssetCollection, Resource)]
pub struct MeshAssets {
   
     #[asset(path = "../artifacts/game_assets/models/obj", collection(typed, mapped))]
    pub(crate) meshes: HashMap<AssetFileName, Handle<Mesh>>,


}


#[derive(AssetCollection, Resource)]
pub struct DecalAssets {

  #[asset(path = "../artifacts/game_assets/decals", collection(typed, mapped))]
    pub(crate) decals: HashMap<AssetFileStem, Handle<DecalManifest>>, //see bevy shader play

}

/*
#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ShaderVariantAssets {
    #[asset(path = "../artifacts/game_assets/shader_variants", collection(typed, mapped))]
    pub(crate) variants: HashMap<AssetFileStem, Handle<ShaderVariantManifest>>, //see bevy shader play
}
*/ 


#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct MagicFxVariantAssets {
    #[asset(path = "magic_fx", collection(typed, mapped))]
    pub(crate) magic_fx_variants: HashMap<AssetFileStem, Handle<MagicFxVariantManifest>>, //see bevy shader play
}




#[derive(Resource, Default)]
pub struct BuiltVfxHandleRegistry {

    //shader var name -> animated material
    //pub animated_materials_map: HashMap<String, Handle<AnimatedMaterial>>,


    pub magic_fx_variants: HashMap<String, MagicFxVariant>  , 

    
}
 

/*
fn load_shader_variants( 
    
    mut next_state: ResMut<NextState<AssetLoadState>>,
 

 //   mut asset_loading_resource: ResMut<AssetLoadingResource>,
    mut animated_materials: ResMut<Assets<AnimatedMaterial>>,

    loaded_textures: Res<TextureAssets>,
    loaded_shader_variants: Res<ShaderVariantAssets>, 


    shader_variant_manifest_resource: Res<Assets<ShaderVariantManifest>>,


    mut built_vfx_resource: ResMut<BuiltVfxHandleRegistry>

   // asset_server: ResMut<AssetServer>,
) {

 
                //once the shader variant loads, we can start loading our magic fx

                for (file_stem, shader_manifest_handle) in loaded_shader_variants.variants.clone().iter() {
             

                     let shader_variant_manifest: &ShaderVariantManifest = shader_variant_manifest_resource
                        .get( shader_manifest_handle.id())
                        .expect(format!("could not load {:?}", &file_stem).as_str());

                    //finish loading and building the shader variant and add it to the map 
                    let texture_handles_map = &loaded_textures.vfx_textures;
                    let mut rebuilt_texture_handle_map: HashMap<String, Handle<Image>> = HashMap::new();

                    for (key, value) in texture_handles_map.iter() {
                        rebuilt_texture_handle_map.insert(key.clone().into(), value.clone());
                    }

                   // let file_stem_clone = file_stem.clone();
                    let shadvar_name =  file_stem.clone() ; 


                    let Some(built_material) = build_animated_material(
                        shader_variant_manifest,
                        &rebuilt_texture_handle_map
                     ).ok() else {
                        warn!("could not load {:?}", &shadvar_name);
                        continue;
                    };


                    let shader_material_handle = animated_materials.add( built_material ); 
                    println!("adding shadvar_name {:?}",&shadvar_name);

                    built_vfx_resource.animated_materials_map.insert( shadvar_name.into(), shader_material_handle );


                  //  if asset_loading_resource.animated_material_map.len() >= asset_loading_resource.shader_variants_map.len() {
                    		
                    //}
                    

               
                   
                }

                    next_state.set(AssetLoadState::ShadersLoad);
            
           
}

*/


fn load_magic_fx( 
    
    mut next_state: ResMut<NextState<AssetLoadState>>,
 

    //  asset_loading_resource: Res <AssetLoadingResource>,
   // mut animated_materials: ResMut<Assets<AnimatedMaterial>>,

   // loaded_textures: Res<TextureAssets>, 
     loaded_meshes: Res<MeshAssets>, 

   loaded_magic_fx_variants: Res<MagicFxVariantAssets>, 

     fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,

     registered_materials_map: Res<RegisteredMaterialsMap> ,

      generic_materials_assets: Res<Assets<GenericMaterial>>,
    mut asset_server: ResMut<AssetServer>,


    mut built_vfx_resource: ResMut<BuiltVfxHandleRegistry>

     
) {


   for (file_stem, magic_fx_handle) in loaded_magic_fx_variants.magic_fx_variants.clone().iter() {




   	        let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get( magic_fx_handle.id() )
                        .unwrap();

                     let mesh_handles_map = &loaded_meshes.meshes;
                    let mut rebuilt_mesh_handle_map: HashMap<String, Handle<Mesh>> = HashMap::new();

                    for (key, value) in mesh_handles_map.iter() {

                        info!("loaded mesh {:?} {:?}", key,value);
                        rebuilt_mesh_handle_map.insert(key.clone().into(), value.clone());
                    }

                    let built_materials_map = & registered_materials_map.0 ;
    

                        info!("loading magic fx {:?}", file_stem );




                   match MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &rebuilt_mesh_handle_map,
                      
                            &built_materials_map,
                             &generic_materials_assets,
                          &mut asset_server 
     
                        
                    ) {

                    Ok( magic_fx) => {


                        info!("loaded magic fx {:?}", file_stem );

                        let variant_name = file_stem.clone(); 

                         built_vfx_resource.magic_fx_variants.insert(  variant_name.into(), magic_fx)   ;


                    }

                    Err( e ) => {

                          panic!("unable to load  magic fx {:?} {:?}", file_stem, e   );
                    }

                   }



                 /*   if let Some(magic_fx) = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &rebuilt_mesh_handle_map,
                      
                            &built_materials_map,
                             &generic_materials_assets,
                          &mut asset_server 
     
                        
                    ) {


                        info!("loaded magic fx {:?}", file_stem );

                        let variant_name = file_stem.clone(); 

                         built_vfx_resource.magic_fx_variants.insert(  variant_name.into(), magic_fx)   ;

                    }else {

                        panic!("unable to load  magic fx {:?}", file_stem  );
                    } */


   }	


   next_state.set(AssetLoadState::Complete);

   info!("Asset loading complete.");

}





const COPY_DIR_ARRAY: [ [&str; 2]  ; 1] =  [  [

        "textures/material_overrides",  //relative to the external assets folder 
        "textures/material_overrides",  //relative to local assets

  ]];

pub fn copy_game_assets_into_artifacts(

    editor_config_res: Res<EditorConfig >,
    editor_config_assets: Res<Assets<EditorConfig>>,

){

  let editor_config  = &editor_config_res;


  let local_assets_path = "./assets";
   let local_artifacts_path = "./artifacts/game_assets";

   info!("creating dir ");
   std::fs::create_dir_all(Path::new(local_artifacts_path)).expect("Could not create artifacts dir");



 //  if let Some( editor_config  ) = editor_config_assets.get( editor_config_handle ){

    let default_external_assets_path = "./example_game_assets".to_string();
    let external_assets_folder = editor_config.get_external_game_assets_folder().unwrap_or(   &default_external_assets_path   );
  

        // copy all external assets into our artifacts folder 
      let copied = copy_dir_recursive( 
         Path::new(&external_assets_folder),
         Path::new(local_artifacts_path)
        );
      info!("copied {:?}" ,  copied );

      // copy external assets into our assets folder 
      for copy_dir_props in COPY_DIR_ARRAY {
        let _ = copy_dir_recursive( 
         &Path::new(&external_assets_folder).join(copy_dir_props[0]),
         &Path::new(local_assets_path).join(copy_dir_props[1])
        ) ;
    }

    
 




}


fn populate_prefab_definitions (

      mut prefab_definitions: ResMut< PrefabDefinitionsResource > ,  


){


    let folder_path = "./assets/prefabs";

    let mut definitions_array = HashMap::new();

    let mut file_names_array : Vec<String> = Vec::new();

     walk_dir( folder_path, "ron" , &mut file_names_array) ;
 


    for file_path in file_names_array {

        let Some(prefab_def) = PrefabDefinition::load_from_path( Path::new(&file_path) )  else {
            eprintln!("could not parse {:?}", file_path);
            continue;
        };  

        let file_path_parts = file_path.split("/");

        let Some(file_name) = file_path_parts.last() else {continue};


         let fixed_prefab_name = match file_name.ends_with( "prefab.ron" ) || file_name.ends_with( "prefab" ){

            true => {

                  let   parts: Vec<&str> = file_name.split('.').collect();
                 
                    parts.first().unwrap() .to_string()  
                   

              }, 
            false => file_name.to_string()

        };

        definitions_array .insert(
            fixed_prefab_name,

            prefab_def 
        );  
       

    }


      prefab_definitions.loaded_prefab_definitions = Some(definitions_array) ;

}


fn populate_doodad_definitions(

    doodad_manifest_handles: Res<DoodadManifestAssets>,
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,

    mut doodad_definitions: ResMut< DoodadDefinitionsResource > , //this is what is modified 


){

    let mut loaded_doodad_definitions = HashMap::new();


    for (_, doodad_manifest_handle) in &doodad_manifest_handles.doodad_manifests {

        if let Some(loaded_manifest) = doodad_manifest_assets.get(  doodad_manifest_handle ) {

            for (doodad_name, doodad_definition) in &loaded_manifest.spawnables {

                loaded_doodad_definitions.insert(doodad_name.to_string(), doodad_definition.clone());
            }

        }

    }


    doodad_definitions.loaded_doodad_definitions = Some(loaded_doodad_definitions);


}

/*
fn populate_doodad_tag_map_data(
        doodad_definitions_resource: Res < DoodadDefinitionsResource > ,

    mut doodad_tag_map_resource: ResMut<DoodadTagMapResource>, 
    //doodad_manifest_assets: Res<Assets<DoodadManifest>>,

    
) {
 


            //now that our manifest is loaded, lets populate the doodad tag map resource 
            for (doodad_name,doodad_definition) in  doodad_definitions_resource.loaded_doodad_definitions.as_ref().unwrap_or(&HashMap::new()) {

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


     
 
     
}*/

fn populate_doodad_tag_map_data(
    doodad_definitions_resource: Res<DoodadDefinitionsResource>,
    mut doodad_tag_map_resource: ResMut<DoodadTagMapResource>, 
) {
    // Box the HashMap to move it to the heap
    let mut temp_tag_map: Box<HashMap<String, Vec<String>>> = Box::new(HashMap::new());

    // Now that our manifest is loaded, populate the doodad tag map resource
    for (doodad_name, doodad_definition) in doodad_definitions_resource
        .loaded_doodad_definitions
        .as_ref()
        .unwrap_or(&HashMap::new())
    {
        // Use `.to_owned()` to avoid cloning the entire Vec at once, only individual tags
        for tag in &doodad_definition.tags.clone().unwrap_or_else(Vec::new) {
            temp_tag_map
                .entry(tag.clone())
                .or_default()
                .push(doodad_name.to_string());
        }

        // Add all doodads under "all_doodads" tag
        temp_tag_map
            .entry("all_doodads".to_string())
            .or_default()
            .push(doodad_name.to_string());
    }

    // Sort the tag keys and the doodad names under each tag
    info!("Sorting doodad keys");
    let mut sorted_keys: Box<Vec<String>> = Box::new(temp_tag_map.keys().cloned().collect());
    sorted_keys.sort();

    // Rebuild the final tag map, moving from temp_tag_map to doodad_tag_map_resource
    doodad_tag_map_resource.doodad_tag_map = sorted_keys
        .into_iter()
        .map(|k| (k.clone(), temp_tag_map.remove(&k).unwrap()))
        .collect();

    // Sort the doodad names under each tag
    for doodads in doodad_tag_map_resource.doodad_tag_map.values_mut() {
        doodads.sort();
    }

    info!("Sorted doodad keys");
}

fn apply_editor_config(


    mut editor_cx: ResMut<Editor>,
    editor_config : Res<EditorConfig >,
    


){


 //   let editor_config_handle = &editor_config_handles.editor_config;
    //let Some(editor_config) = editor_config_assets.get(editor_config_handle) else {return} ;

   let Some(   state ) = editor_cx.window_state_mut::<PlacementWindow>() else {return};
        

        if let Some( placement_config ) = editor_config.get_default_placement_settings() {


          state.translation_grid_lock_step = placement_config.translation_grid_lock_step
                    .clone().unwrap_or(Vec3::splat(0.0));


        }



}