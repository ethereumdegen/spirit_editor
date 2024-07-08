		
use bevy_asset_loader::prelude::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy::{asset::{AssetPath, LoadedFolder}, prelude::*, utils::HashMap}; 
use bevy_magic_fx::{animated_material::{build_animated_material, AnimatedMaterial}, magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest}, shader_variant::ShaderVariantManifest};


/*

Loads all of the MagicFx assets 

*/


pub fn asset_loading_plugin(app: &mut App) {
		    app


		      .init_state::<AssetLoadState>()
		      .init_resource::<BuiltVfxResource>()
		       .init_resource::<AssetLoadingResource>()
		        .init_resource::<FolderLoadingResource>()
		    //  .add_systems(Startup, setup)

		   //   .add_systems(Update, update_load_folders)

		     // .add_systems(OnEnter(LoadingState::FundamentalAssetsLoad),  load_shader_variants )

		       //.add_systems(OnEnter(LoadingState::ShadersLoad),  load_magic_fx )
		        

                 
                .add_loading_state(
                    LoadingState::new(AssetLoadState::Init)
                        .continue_to_state(AssetLoadState::FundamentalAssetsLoad)
                        .load_collection::<AudioAssets>(),
                )
                .add_systems(OnEnter(AssetLoadState::ShadersLoad), load_magic_fx)
         ;





                
      


 }



#[derive(Resource, Default)]
 pub  struct BuiltVfxResource {


  pub   magic_fx_variants: HashMap<String, MagicFxVariant>      

}






#[derive(Resource, Default)]
  struct AssetLoadingResource {
    texture_handles_map: HashMap<String, Handle<Image>>,
    mesh_handles_map: HashMap<String, Handle<Mesh>>,
    shader_variants_map: HashMap<String, Handle<ShaderVariantManifest>>,

     magic_fx_variants_map: HashMap<String, Handle<MagicFxVariantManifest>>,

    
     animated_material_map: HashMap<String, Handle<AnimatedMaterial>>,
 
}



#[derive(Resource, Default)]
  struct FolderLoadingResource {
   

    textures_folder_handle: Handle<LoadedFolder>,
    shadvars_folder_handle: Handle<LoadedFolder>,
    meshes_folder_handle: Handle<LoadedFolder>,
    magicfx_folder_handle: Handle<LoadedFolder>,


   
}


#[derive(States,Hash,Eq,PartialEq,Debug,Clone,Default)]
pub enum AssetLoadState {
	#[default]
	Init,
    FundamentalAssetsLoad,
    ShadersLoad,
    Complete

}


fn setup(
 
    asset_server: ResMut<AssetServer>,
 

    mut folder_loading_resource: ResMut<FolderLoadingResource>,
 
) {
  
    let textures_folder = asset_server.load_folder("textures/");

    let shadvars_folder = asset_server.load_folder("shader_variants/");

    let meshes_folder = asset_server.load_folder("models/meshes/");

    let magicfx_folder = asset_server.load_folder("magic_fx/");

    folder_loading_resource.textures_folder_handle = textures_folder;
    folder_loading_resource.shadvars_folder_handle = shadvars_folder;
    folder_loading_resource.meshes_folder_handle = meshes_folder;
    folder_loading_resource.magicfx_folder_handle = magicfx_folder;


 
   
}




fn update_load_folders(
       mut ev_asset: EventReader<AssetEvent<LoadedFolder>>,

       asset_server: ResMut<AssetServer>,

       loaded_folder_assets: Res<Assets<LoadedFolder>>,

      mut asset_loading_resource: ResMut<AssetLoadingResource>,

      mut next_state: ResMut<NextState<LoadingState>>

    ){


  for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
             
            let loaded_folder = loaded_folder_assets.get( *id  ).unwrap();  


            for handle in &loaded_folder.handles {
                let asset_path = asset_server.get_path( handle.id()  ).unwrap(); 

               // info!("asset path {:?}", asset_path); 

              
                if (&asset_path.path()).starts_with("models/meshes") { 
                         asset_loading_resource.mesh_handles_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }
 
                if (&asset_path.path()).starts_with("textures") { 
                         asset_loading_resource.texture_handles_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

                if (&asset_path.path()).starts_with("shader_variants") { 
                         asset_loading_resource.shader_variants_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

                 if (&asset_path.path()).starts_with("magic_fx") { 
                         asset_loading_resource.magic_fx_variants_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

               
            }


            if  !asset_loading_resource.mesh_handles_map.is_empty() 
            &&  !asset_loading_resource.texture_handles_map.is_empty()
            &&  !asset_loading_resource.shader_variants_map.is_empty() 
            &&  !asset_loading_resource.magic_fx_variants_map.is_empty() 
            {             

                
                next_state.set(LoadingState::FundamentalAssetsLoad);
            }




         }
         _ => {} 


     }

    }

}



fn load_shader_variants( 
    
    mut next_state: ResMut<NextState<LoadingState>>,
 

    mut asset_loading_resource: ResMut<AssetLoadingResource>,
    mut animated_materials: ResMut<Assets<AnimatedMaterial>>,


    shader_variant_manifest_resource: Res<Assets<ShaderVariantManifest>>,

   // asset_server: ResMut<AssetServer>,
) {

 
                //once the shader variant loads, we can start loading our magic fx

                for (file_path, shader_manifest_handle) in asset_loading_resource.shader_variants_map.clone().iter() {
             

                     let shader_variant_manifest: &ShaderVariantManifest = shader_variant_manifest_resource
                        .get( shader_manifest_handle.id())
                        .expect(format!("could not load {:?}", &file_path).as_str());

                    //finish loading and building the shader variant and add it to the map 
                    let texture_handles_map = &asset_loading_resource.texture_handles_map;
                    

                    let file_path_clone = file_path.clone();
                    let shadvar_name = AssetPath::parse(file_path_clone.as_str()).path().file_stem().unwrap().to_str().unwrap().to_string()  ;


                    let Some(built_material) = build_animated_material(
                        shader_variant_manifest,
                        &texture_handles_map
                     ) else {
                        warn!("could not load {:?}", &shadvar_name);
                        continue;
                    };


                    let shader_material_handle = animated_materials.add( built_material ); 
                    println!("adding shadvar_name {:?}",&shadvar_name);

                    asset_loading_resource.animated_material_map.insert( shadvar_name, shader_material_handle );


                    if asset_loading_resource.animated_material_map.len() >= asset_loading_resource.shader_variants_map.len() {
                    			next_state.set(LoadingState::ShadersLoad);
                     }
                    

               
                   
                }
            
           
}


fn load_magic_fx( 
    
    mut next_state: ResMut<NextState<LoadingState>>,
 

      asset_loading_resource: Res <AssetLoadingResource>,
   // mut animated_materials: ResMut<Assets<AnimatedMaterial>>,


     fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,


    mut built_vfx_resource: ResMut<BuiltVfxResource>

     
) {


   for (file_path, magic_fx_handle) in asset_loading_resource.magic_fx_variants_map.clone().iter() {




   	        let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get( magic_fx_handle.id() )
                        .unwrap();

                     let mesh_handles_map = &asset_loading_resource.mesh_handles_map;

                    let animated_materials_map = &asset_loading_resource.animated_material_map;
  
                    let magic_fx = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &mesh_handles_map,
                      
                        &animated_materials_map,
                     
                        
                    ).unwrap();

                    info!("loaded magic fx {:?}", file_path.to_string());

   				 built_vfx_resource.magic_fx_variants.insert(  file_path.to_string() , magic_fx) ;

   }	


   next_state.set(LoadingState::Complete);

   info!("Asset loading complete.");

}