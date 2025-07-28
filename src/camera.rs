 
use crate::post_processing::PostProcessSettings;
use bevy_foliage_tool::foliage_viewer::FoliageViewer;
use degen_toon_terrain::terrain::TerrainViewer;
use bevy::core_pipeline::tonemapping::Tonemapping;

use bevy::render::view::ColorGrading;

use bevy_editor_pls_default_windows::cameras::EditorCamera;
use bevy_editor_pls::controls::ControlsInteractionState;
use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::core_pipeline::prepass::NormalPrepass;
use bevy::core_pipeline::prepass::DepthPrepass;
 
use bevy::core_pipeline::bloom::Bloom ;
use bevy::pbr::ShadowFilteringMethod;

use bevy::{
    core_pipeline::{
        fxaa:: Fxaa, }
};



pub fn camera_plugin(app: &mut App) {
    app

        .add_systems(Startup, init_camera)
        .add_systems(Update, update_camera_look)
        .add_systems(Update, update_camera_move)
        .add_systems(Update, update_camera_frustrum)

        ;
}

#[derive(Component)]
pub struct DoodadSpawnOrigin;

#[derive(Component)]
struct CameraInitialized;

fn init_camera (
    mut commands: Commands ,
){

    
    // camera
      let mut color_grading = ColorGrading::default();

    color_grading.global.exposure = 1.05;
 

  

    commands
        .spawn( ( Camera3d::default()  ,

                 Camera {
                 hdr: true, // 1. HDR must be enabled on the camera
                ..default()
               },
            Tonemapping::AcesFitted,

            Transform::from_xyz(20.0, 162.5, 20.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
            
        ) )
       .insert( Bloom  ::OLD_SCHOOL )
       .insert( EditorCamera )
       .insert(PostProcessSettings::default())
       
     //  .insert( ToonShaderMainCamera )
         .insert( color_grading ) 
        .insert(TerrainViewer::default())
         .insert( FoliageViewer )
         .insert( DoodadSpawnOrigin )
        .insert( DepthPrepass )
        .insert( NormalPrepass)
        .insert(Fxaa::default()) 
          .insert(ShadowFilteringMethod::Hardware2x2)
       ;

}

  fn update_camera_look(
    mut event_reader: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &EditorCamera)>,
) {
    const MOUSE_SENSITIVITY: f32 = 2.0;

    // Accumulate mouse delta
    let mut delta: Vec2 = Vec2::ZERO;
    for event in event_reader.read() {
        delta += event.delta;
    }
    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // let rotation = transform.rotation;

        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= delta.x / 180.0 * MOUSE_SENSITIVITY;
        pitch -= delta.y / 180.0 * MOUSE_SENSITIVITY;
        pitch = pitch.clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
}

  fn update_camera_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &EditorCamera)>,

    controls_interact_state: Res<State<ControlsInteractionState>>
) {
    const MOVE_SPEED: f32 = 1.0; // You can adjust this value as needed

    let boost_multiplier = match keyboard_input.pressed(KeyCode::ShiftLeft) {
        true => 6.0,
        false => 1.0,
    };

    if *controls_interact_state == ControlsInteractionState::MovementDisallowed {
        return; 
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // Move the camera forward if W is pressed
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * MOVE_SPEED * boost_multiplier;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            let forward = transform.forward();
            transform.translation -= forward * MOVE_SPEED * boost_multiplier;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            let left = transform.left();
            transform.translation += left * MOVE_SPEED * boost_multiplier;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            let left = transform.left();
            transform.translation -= left * MOVE_SPEED * boost_multiplier;
        }

         if keyboard_input.pressed(KeyCode::AltLeft) {
            let up = transform.up();
            transform.translation -= up * MOVE_SPEED * boost_multiplier;
        }

        if keyboard_input.pressed(KeyCode::Space) {
            let up = transform.up();
            transform.translation += up * MOVE_SPEED * boost_multiplier;
        }
    }
}


fn update_camera_frustrum (

        mut camera_query: Query<(&Projection, &mut Transform)>,
) {




}