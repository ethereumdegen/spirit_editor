 

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::{TerrainMeshPlugin, terrain::{  TerrainData, TerrainViewer}, edit::{EditTerrainEvent, TerrainCommandEvent}};
 use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::edit::EditingTool;

use bevy::pbr::ShadowFilteringMethod;


use bevy_mod_raycast::prelude::*;

mod ui;
mod camera;
mod tools;
mod commands;

use crate::camera::{update_camera_look,update_camera_move};

use crate::tools::{update_brush_paint };

use crate::commands::{update_commands };
use crate::ui::{  editor_ui_plugin };

use seldom_fn_plugin::FnPluginExt;

fn main() {
    App::new()
        
         
          .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { 
                    present_mode:  bevy::window::PresentMode::AutoNoVsync, //improves latency

                    title: "Mesh Terrain Editor".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )

        .add_plugins(DefaultRaycastingPlugin)
         
        .add_plugins( TerrainMeshPlugin::default() )
        .fn_plugin(editor_ui_plugin)

          
        .add_systems(Startup, setup) 

        //move to brushes and tools lib 
        .add_systems(Update, update_brush_paint )
        .add_systems(Update, update_commands)
        
        
        //move to camera lib 
        .add_systems(Update, update_camera_look ) 
        .add_systems(Update, update_camera_move ) 
        
     
        
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands 
     
   // asset_server: Res<AssetServer> 
) {
    
     
  
     
     commands.spawn(SpatialBundle::default() )  
    .insert(
        TerrainConfig::load_from_file("assets/terrain/default_terrain/terrain_config.ron").unwrap() 
        ) 
    .insert(
        TerrainData::new()  
    ); 
     


    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight{


            shadow_depth_bias: 0.5,
            shadow_normal_bias: 0.5,
            
            color: Color::WHITE,
            ..default()
        },
        
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,

            shadow_depth_bias: 0.5,
            shadow_normal_bias: 0.5,

            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 800.0, 4.0),
        ..default()
    });


    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.62,
    });
 
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(20.0, 162.5, 20.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
            ..default()
        })
        .insert(TerrainViewer::default())
        .insert(  ShadowFilteringMethod::Jimenez14 )
        
        ;

     
}


 
 
 