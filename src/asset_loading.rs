		
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_editor_pls_default_windows::doodads::doodad_manifest::DoodadManifest;
use crate::EditorConfig;
use bevy::gltf::Gltf;
use bevy_asset_loader::prelude::*; 
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy::{asset::{AssetPath, LoadedFolder}, prelude::*, utils::HashMap}; 
use bevy_magic_fx::{animated_material::{build_animated_material, AnimatedMaterial}, magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest}, shader_variant::ShaderVariantManifest};


/*

Loads all of the MagicFx assets 

*/


pub fn asset_loading_plugin(app: &mut App) {
		    app


		      .init_state::<AssetLoadState>()
		      .init_resource::<BuiltVfxHandleRegistry>()



         .add_plugins(RonAssetPlugin::<EditorConfig>::new(&["editorconfig.ron"])) 


              .add_loading_state(
                    LoadingState::new(AssetLoadState::Init)
                        .continue_to_state(AssetLoadState::TextureAssetsLoad)
                        .load_collection::<EditorConfigAssets>() 
                         
                )


		     .add_loading_state(
                    LoadingState::new(AssetLoadState::TextureAssetsLoad)
                        .continue_to_state(AssetLoadState::ShaderAssetsLoad)
                        .load_collection::<TextureAssets>() 
 
                          
                )
 
             /* .add_loading_state(
                    LoadingState::new(AssetLoadState::GltfAssetsLoad)
                        .continue_to_state(AssetLoadState::ShaderAssetsLoad)
                        
                         .load_collection::<GltfAssets>() 
                         
                )*/


              .add_loading_state(
                    LoadingState::new(AssetLoadState::ShaderAssetsLoad)
                        .continue_to_state(AssetLoadState::ShaderVariantsLoad)
                        
                    
                         .load_collection::<MeshAssets>()
                          .load_collection::<ShaderVariantAssets>() 
                          .load_collection::<MagicFxVariantAssets>()
                           //.load_collection::<AnimatedMaterialAssets>() 
                )


               .add_systems(OnEnter(AssetLoadState::ShaderVariantsLoad), load_shader_variants)
                 .add_systems(OnEnter(AssetLoadState::ShadersLoad), load_magic_fx)
                 
              
         ;





                
      


 }

 #[derive(States,Hash,Eq,PartialEq,Debug,Clone,Default)]
pub enum AssetLoadState {
    #[default]
    Init,
    TextureAssetsLoad,
    //GltfAssetsLoad,
    ShaderAssetsLoad,
    ShaderVariantsLoad,
    ShadersLoad,
    Complete

}




#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
   
     #[asset(path = "textures", collection(typed, mapped))]
    pub(crate) textures: HashMap<String, Handle<Image>>,


}

#[derive(AssetCollection, Resource)]
pub struct EditorConfigAssets {
   
     #[asset(path = "editor_config.editorconfig.ron" )]
    pub(crate) editor_config:   Handle<EditorConfig> ,

    // #[asset(path = "doodad_manifest.doodadmanifest.ron" )]
    //pub(crate) doodad_manifest:   Handle<DoodadManifest> ,
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
   
     #[asset(path = "models/meshes", collection(typed, mapped))]
    pub(crate) meshes: HashMap<String, Handle<Mesh>>,


}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ShaderVariantAssets {
    #[asset(path = "shader_variants", collection(typed, mapped))]
    pub(crate) variants: HashMap<AssetFileStem, Handle<ShaderVariantManifest>>, //see bevy shader play
}


#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct MagicFxVariantAssets {
    #[asset(path = "magic_fx", collection(typed, mapped))]
    pub(crate) magic_fx_variants: HashMap<AssetFileStem, Handle<MagicFxVariantManifest>>, //see bevy shader play
}




#[derive(Resource, Default)]
pub struct BuiltVfxHandleRegistry {

    //shader var name -> animated material
    pub animated_materials_map: HashMap<String, Handle<AnimatedMaterial>>,


    pub magic_fx_variants: HashMap<String, MagicFxVariant>  , 

    
}
 


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
                    let texture_handles_map = &loaded_textures.textures;
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


fn load_magic_fx( 
    
    mut next_state: ResMut<NextState<AssetLoadState>>,
 

    //  asset_loading_resource: Res <AssetLoadingResource>,
   // mut animated_materials: ResMut<Assets<AnimatedMaterial>>,

    loaded_textures: Res<TextureAssets>, 
     loaded_meshes: Res<MeshAssets>, 

   loaded_magic_fx_variants: Res<MagicFxVariantAssets>, 

     fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,


    mut built_vfx_resource: ResMut<BuiltVfxHandleRegistry>

     
) {


   for (file_stem, magic_fx_handle) in loaded_magic_fx_variants.magic_fx_variants.clone().iter() {




   	        let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get( magic_fx_handle.id() )
                        .unwrap();

                     let mesh_handles_map = &loaded_meshes.meshes;
                    let mut rebuilt_mesh_handle_map: HashMap<String, Handle<Mesh>> = HashMap::new();

                    for (key, value) in mesh_handles_map.iter() {
                        rebuilt_mesh_handle_map.insert(key.clone().into(), value.clone());
                    }

                    let animated_materials_map = &built_vfx_resource.animated_materials_map;
    

                        info!("loading magic fx {:?}", file_stem );

                    if let Some(magic_fx) = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &rebuilt_mesh_handle_map,
                      
                        &animated_materials_map,
                     
                        
                    ) {


                        info!("loaded magic fx {:?}", file_stem );

                        let variant_name = file_stem.clone(); 

                         built_vfx_resource.magic_fx_variants.insert(  variant_name.into(), magic_fx)   ;

                    }else {

                        warn!("unable to load  magic fx {:?}", file_stem  );
                    }


   }	


   next_state.set(AssetLoadState::Complete);

   info!("Asset loading complete.");

}