





mod editor_state;
mod loading;
mod clay_tiles;
mod shaders;

mod decals; 
mod decal_manifest; 

//mod material_overrides;
mod editor_config; 
mod camera;
mod commands;
mod editor_pls;
mod tools;
mod ui;
mod asset_loading;
mod liquid;
mod materialize_properties; 

mod level_config;

mod clouds;
 
mod doodads;
mod terrain;
 mod foliage; 

 mod render;

mod regions;

mod utils;
mod virtual_link;
mod material_override_link;
mod physics ; 

mod benchmarking;
mod post_processing; 

 
use bevy_editor_pls_default_windows::cameras::EditorCamera;
use crate::asset_loading::LevelAssets;
use crate::shaders::material_affine_processor::Affine2Processor;
use bevy_editor_pls_core::EditorEvent;
use bevy_materialize::MaterializePlugin;
use bevy_materialize::prelude::TomlMaterialDeserializer;
use bevy::image::ImageSamplerDescriptor;
use bevy::render::render_resource::AddressMode;
use bevy::render::render_resource::FilterMode;
use bevy_foliage_tool::foliage_scene::FoliageScene;
use bevy_foliage_tool::foliage_types::FoliageTypesManifest;
use bevy_foliage_tool::foliage_density::FoliageDensityMapsComponent;
use bevy_foliage_tool::foliage_scene::FoliageRoot;
use bevy::winit::WinitWindows;
//use bevy_foliage_tool::foliage_config::FoliageConfig;
 
//use bevy_foliage_tool::foliage_config::LoadFoliageConfig;
use degen_toon_clouds::DegenToonCloudsPlugin;
use level_config::LevelConfig;
use winit::window::Icon;





use bevy::render::view::ColorGrading;
 
use bevy_foliage_tool::foliage_viewer::FoliageViewer;
 





use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::TaskPoolBuilder;




use bevy::{
    asset::{
        io::{AssetSourceBuilder, AssetSourceId},
        AssetPath,
    } 
};


use bevy::{
    core_pipeline::{
        fxaa:: Fxaa, }
};

        use spirit_edit_core::SpiritEditCorePlugin;
use spirit_edit_core::zones::ZoneEvent; 
use asset_loading::AssetLoadState;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_editor_pls_default_windows::lighting::Sun;
 
 use degen_toon_terrain::TerrainEditMode;

 use bevy_material_wizard::BevyMaterialWizardPlugin;

use crate::editor_config::EditorConfig;

use bevy::core_pipeline::prepass::NormalPrepass;
use bevy::core_pipeline::prepass::DepthPrepass;
//use bevy_foliage_paint::foliage_config::FoliageConfig;
//use bevy_foliage_paint::foliage::FoliageData;
use bevy_foliage_tool::BevyFoliageToolPlugin;
use bevy_foliage_tool::BevyFoliageMaterialPlugin; 
use bevy_foliage_tool::BevyFoliageProtoPlugin;

use bevy_regions::regions::RegionsData;
use bevy_regions::regions_config::RegionsConfig;
use bevy_regions::BevyRegionsPlugin;
use asset_loading::asset_loading_plugin;


use bevy::core_pipeline::bloom::Bloom ;
 
use bevy::pbr::wireframe::WireframePlugin;
use bevy_magic_fx::MagicFxPlugin;
use ui::EditorToolsState;


use crate::doodads::doodad::handle_rebuild_doodads;


use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use degen_toon_terrain::edit::EditingTool;

// use degen_toon_terrain::terrain_material::ToonShaderSun; 
use degen_toon_terrain::terrain_config::TerrainConfig;
use degen_toon_terrain::{
    edit::{EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    TerrainMeshPlugin,
};


use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::pbr::ShadowFilteringMethod;
  
use crate::camera::camera_plugin;
use crate::liquid::liquid_plugin;
use bevy_clay_tiles; 
use bevy_clay_tiles::tiles_config::ClayTilesConfig;
 
use crate::tools::brush_tools_plugin;

use crate::commands::update_commands;
use crate::ui::editor_ui_plugin;

/*
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = bevy:: image::open("assets/images/favicon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
*/


fn main() {

        //maybe not necessary 
     AsyncComputeTaskPool::get_or_init(|| TaskPoolBuilder::new()
       .num_threads(32).stack_size(16 * 1024 * 1024).build());



    let mut wgpu_settings = WgpuSettings::default();
   // wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;  //what is this for ? 

     

    App::new()


      .register_asset_source(
            "game_assets",
            AssetSourceBuilder::platform_default("artifacts/game_assets", None),
        )

    //  .add_event::<EditorEvent>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        //present_mode: bevy::window::PresentMode::AutoNoVsync, //improves latency

                        title: "Spirit Editor".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(wgpu_settings),
                    ..default()
                })



                 .set(AssetPlugin {
                     unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow ,  // for using ./artifacts , for now 
                    ..default()
                })



               .set(ImagePlugin{

                    default_sampler : ImageSamplerDescriptor {
                        label: None,
                        address_mode_u:  AddressMode::Repeat.into(),
                        address_mode_v:  AddressMode::Repeat.into(),
                        address_mode_w:  AddressMode::Repeat.into(),
                        mag_filter: FilterMode::Nearest.into(),
                        min_filter: FilterMode::Nearest.into(),
                        mipmap_filter: FilterMode::Nearest.into(),
                        ..default()
                    }

               })   // no linear filter ! 





        )           



         .add_systems(PreStartup, 


           ( editor_config::load_editor_config, 
            asset_loading:: copy_game_assets_into_artifacts ).chain() 


            )

        // .register_type::< TextureSubsetDimensions >()

        .add_plugins( MaterializePlugin::new(TomlMaterialDeserializer)
                    .with_simple_loader(None)   //to prevent bug with PNG loading 
                    .with_processor( Affine2Processor  )
             )

        
     //   .add_plugins(  WindowIconPlugin::new("assets/images/favicon.png") ) 
        
        .add_plugins( MagicFxPlugin )  //this adds the materialize plugin.. so it comes first 

          // .add_plugins(ToonShaderPlugin)
        .add_plugins(loading::loading_plugin)
   //      .add_plugins(CursorRayPlugin)
        .add_plugins(virtual_link::virtual_links_plugin)
        .add_plugins(material_override_link::material_overrides_link_plugin)

        .add_plugins(TerrainMeshPlugin{

            terrain_edit_mode: TerrainEditMode::TerrainEditable 
        })

        .add_plugins(BevyRegionsPlugin::default())


        .add_plugins(render::rendering_plugin) 
        .add_plugins(shaders::shaders_plugin) 
         .add_plugins(terrain::terrain_plugin) 
        
        .add_plugins(clouds::clouds_plugin)
        .add_plugins(physics::physics_plugin) 

            .add_plugins(materialize_properties::materialize_properties_plugin   )  //must be BEFORE teh material wizard 


        .add_plugins(BevyMaterialWizardPlugin{
            material_defs_folder_prefix: Some("../artifacts/game_assets/".to_string()),  //relative to assets 
            material_defs_manifest_path: "artifacts/game_assets/manifests/material_definitions.materialmanifest.ron".to_string(),
            material_replacements_folder_path: "artifacts/game_assets/material_replacements".to_string(), 
        }  )


        .add_plugins(bevy_clay_tiles::BevyClayTilesPlugin {
             config: ClayTilesConfig::load_from_file("assets/tiles_config.ron").unwrap()
        })
    

        
         
        .add_plugins(SpiritEditCorePlugin {})

     //   .add_plugins ( benchmarking::benchmarking_plugin ) 
 


        .add_plugins(BevyFoliageToolPlugin  )

        .add_plugins(BevyFoliageMaterialPlugin)  
        
        .add_plugins(BevyFoliageProtoPlugin )

        .add_plugins(foliage::foliage_plugin   )
       .add_plugins( post_processing::post_processing_plugin )
    
       //  .add_plugins( bevy_contact_projective_decals:: DecalPlugin ) // important! imports the shader 
        .add_plugins(decals::decals_plugin)
      

      
        .add_plugins(doodads::doodads_plugin )

        .add_plugins(terrain::terrain_manifest::terrain_manifest_plugin)
        .add_plugins(terrain::terrain_loading::terrain_loading_plugin)
        
      
        .add_plugins(asset_loading_plugin)

        //.add_plugins(material_overrides::material_overrides_plugin)
        .add_plugins(liquid_plugin)
        .add_plugins(brush_tools_plugin)
        .add_plugins(editor_ui_plugin)
        .add_plugins(camera_plugin)
       // .add_systems(Startup, set_window_icon)



        .add_systems(OnEnter(AssetLoadState::Complete),   (
            setup,
            load_all_zones )

             .chain()  ) 


        //move to brushes and tools lib
        .add_systems(Update, update_commands)
        .add_systems(Update, handle_rebuild_doodads)
         .add_systems(Update, regions::update_regions_plane_visibility)
        .add_systems(Update, update_directional_light_position)

        .add_plugins( clay_tiles::clay_tiles_plugin )

        //move to camera lib
        .add_plugins(editor_pls::editor_ui_plugin)
        .add_plugins(editor_state::editor_state_plugin )
          
        .run();
}

/// set up a simple 3D scene
fn setup(

   mut commands: Commands,

   mut zone_event_writer: EventWriter<ZoneEvent>,

   editor_config : Res<EditorConfig >,
  


   level_assets: Res< LevelAssets >,

   level_config_assets: Res<Assets<LevelConfig >>,


    

 // asset_server: Res<AssetServer>
) {
 
    
   // let mut editor_config = None;

 
 
    

    if let Some(level_name) = &editor_config.get_initial_level_name(){

        if let Some(level_config)  = level_assets.levels.get( level_name.as_str() )

        .map(|h| level_config_assets.get(h)  )  .flatten() {



             if let Some(terrain_path) = &level_config.get_initial_terrain_path_full(){
   
                commands
                    .spawn(Transform::default())
                     .insert(Visibility::Inherited)
                    .insert(
                        TerrainConfig::load_from_file(terrain_path)
                            .unwrap(),
                    )
                    .insert(TerrainData::new()); 

            }





            if let Some(foliage_scene_name) = &level_config.get_foliage_scene_name() {



               // let loaded_foliage_config = FoliageConfig::load_from_file(  )
        

                let foliage_scenes_folder_path = "assets/foliage/foliage_scenes/";
            

             let foliage_config_path = format!("{}{}",  &foliage_scenes_folder_path , &foliage_scene_name  );

       
     
               let foliage_scene = FoliageScene::load_from_file(&foliage_config_path)
                    .expect("Could not load foliage config");

                let foliage_types_manifest =
                    FoliageTypesManifest::load_from_file(&foliage_scene.foliage_types_manifest_path)
                        .expect("Could not load foliage types manifest");

                let foliage_density_data_path = foliage_scene.foliage_density_data_path.clone(); 
                let foliage_definitions = foliage_types_manifest.foliage_definitions .clone(); 
                let foliage_dimensions = foliage_scene.boundary_dimensions .clone(); 

 
                let foliage_density_maps_component = 
                    FoliageDensityMapsComponent::create_or_load( 
                        foliage_density_data_path, 
                        foliage_dimensions, 
                        foliage_definitions

                     ) ;
 


                  commands.spawn( (

                    FoliageRoot, 
                    foliage_scene,

                    foliage_types_manifest,
                    foliage_density_maps_component

                ) );
                 


                        // foliage config files are per-level 
                   /*   commands.queue(   LoadFoliageConfig {

                        path: foliage_scenes_folder_path.to_string(),
                        name: foliage_scene_name.to_string() ,
                  

                      }  );*/

         
               /* commands
                    .spawn(Transform::default())
                    .insert(Visibility::Inherited)
                    .insert( 
                        FoliageSceneData::create_or_load(  
                        foliage_scenes_folder_path, 
                        foliage_scene_name  
                        ) //this will be unpacked automagically 
                    ).insert(
                        Name::new( foliage_scene_name.clone() )
                    ) ; */

             }



        }
    
        

    }
     
      
 

        //spawn regions painting plane 
     commands
        .spawn( Transform::from_xyz(0.0, 40.0, 0.0) )
          .insert(Visibility::Inherited)
        .insert(RegionsConfig::load_from_file("assets/regions/regions_config.ron").unwrap())
        .insert(RegionsData::new()) 
        .insert(Visibility::Hidden)  // only in editor 
        ;


    

    //add toon shader 


 
    commands.spawn(  DirectionalLight {
           // shadow_depth_bias: 0.5,
           // shadow_normal_bias: 0.5,


            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            color: Color::WHITE,
            ..default()
        }   )
    .insert(Sun)
   // .insert( ToonShaderSun )

    ; 


        //efficient for low poly 
   // *msaa = Msaa::Sample4; 

}

 


fn load_all_zones(

   //   mut commands: Commands,

   mut zone_event_writer: EventWriter<ZoneEvent>,

   editor_config : Res<EditorConfig >,

    level_assets : Res<LevelAssets >,

   //editor_config_assets: Res<Assets<EditorConfig >> ,

    level_config_assets: Res<Assets<LevelConfig >>,

){

 



       if let Some(level_name) = &editor_config.get_initial_level_name(){

        if let Some(level_config)  = level_assets.levels.get( level_name.as_str() )

        .map(|h| level_config_assets.get(h)  )  .flatten() {

              
            //initialize zones 
            for zone_name in level_config.get_initial_zones_to_load().unwrap_or(Vec::new()) {
         
              zone_event_writer.write(   ZoneEvent::LoadZoneFile(zone_name)  );
         
            }



        }
    }





}


fn update_directional_light_position(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
   
    time: Res<Time>,
) {

    let current_time = time.elapsed();


 //   let delta_time = time.delta_seconds();
    
    let SECONDS_IN_A_CYCLE = 80.0;

    let angle = (current_time.as_millis() as f32 / (SECONDS_IN_A_CYCLE* 1000.0) ) * std::f32::consts::PI * 2.0; // Convert time to radians

    let radius = 20.0; // Adjust the radius of the sun's orbit
    let x = angle.cos() * radius;
    let y = angle.sin() * radius + 10.0; // Adjust the height of the sun
    let z = 0.0;

    for mut transform in query.iter_mut() {

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

