 
use bevy_editor_pls::controls::ControlsInteractionState;
use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;

 


pub fn camera_plugin(app: &mut App) {
    app

        .add_systems(Update, init_camera)
        .add_systems(Update, update_camera_look)
        .add_systems(Update, update_camera_move)
        .add_systems(Update, update_camera_frustrum)

        ;
}

#[derive(Component)]
struct CameraInitialized;

fn init_camera (
    mut commands: Commands , 
    camera_query: Query<Entity, (With<Camera3d>, Without< CameraInitialized> )>
){

    for  camera_entity  in camera_query.iter() {

        if let Ok(mut cmd) = commands.get_entity( camera_entity ){

            cmd
          /*  .insert (  EdgeDetection { 
                normal_threshold: 2.2, 
                steep_angle_threshold: 0.9, 
                enable_depth: true, 
               
                ..default()
            }  ) */
            .insert( CameraInitialized ) 


            ; 
        }

    }

}

  fn update_camera_look(
    mut event_reader: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
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
    mut query: Query<(&mut Transform, &Camera3d)>,

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