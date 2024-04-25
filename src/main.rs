use asset_loading::asset_loading_plugin;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_magic_fx::MagicFxPlugin;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_mesh_terrain::edit::EditingTool;
use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::{
    edit::{EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    TerrainMeshPlugin,
};


use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::pbr::ShadowFilteringMethod;

use bevy_mod_raycast::prelude::*;

mod camera;
mod commands;
mod editor_pls;
mod tools;
mod ui;
mod asset_loading;
mod water;

mod doodads;

use crate::camera::camera_plugin;
use crate::water::water_plugin;

use crate::tools::brush_tools_plugin;

use crate::commands::update_commands;
use crate::ui::editor_ui_plugin;

use seldom_fn_plugin::FnPluginExt;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync, //improves latency

                        title: "Mesh Terrain Editor".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(wgpu_settings),
                    ..default()
                }),


        )   
       
        .add_plugins(DefaultRaycastingPlugin)
        .add_plugins(TerrainMeshPlugin::default())

        .add_plugins(doodads::doodad::DoodadPlugin)

        .add_plugins(bevy_obj::ObjPlugin)
        .add_plugins( MagicFxPlugin )
        .add_plugins(asset_loading_plugin)

 
        .fn_plugin(water_plugin)
        .fn_plugin(brush_tools_plugin)
        .fn_plugin(editor_ui_plugin)
        .fn_plugin(camera_plugin)
        .add_systems(Startup, setup)
        //move to brushes and tools lib
        .add_systems(Update, update_commands)
        .add_systems(Update, update_directional_light_position)
        //move to camera lib
        .add_plugins(editor_pls::editor_ui_plugin)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, // asset_server: Res<AssetServer>
) {
    commands
        .spawn(SpatialBundle::default())
        .insert(
            TerrainConfig::load_from_file("assets/terrain/default_terrain/terrain_config.ron")
                .unwrap(),
        )
        .insert(TerrainData::new());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
           // shadow_depth_bias: 0.5,
           // shadow_normal_bias: 0.5,

            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            color: Color::WHITE,
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(7.0, 20.0, 2.0),
            
            ..default()
        },

        ..default()
    });

    /*commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
           // shadow_depth_bias: 0.5,
           // shadow_normal_bias: 0.5,

            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            color: Color::WHITE,
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI  * 1.1 ),
            ..default()
        },

        ..default()
    });

     commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_depth_bias: 0.5,
            shadow_normal_bias: 0.5,

            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            color: Color::WHITE,
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x( PI * 0.1),
            ..default()
        },

        ..default()
    });*/


    // light
    /* commands.spawn(PointLightBundle {
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
    */
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0 ,
    });







    // camera

    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                 hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },

            transform: Transform::from_xyz(20.0, 162.5, 20.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
            ..default()
        })
        .insert( BloomSettings::default())
        .insert(TerrainViewer::default())
       // .insert(ShadowFilteringMethod::Jimenez14)
       ;
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