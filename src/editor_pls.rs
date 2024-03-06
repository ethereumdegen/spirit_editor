use bevy::prelude::*;

use bevy_editor_pls::controls::EditorControls;
use bevy_editor_pls::default_windows::cameras::camera_3d_free;
use bevy_editor_pls::EditorPlugin;
use bevy_editor_pls::controls;
//use bevy_editor_pls_default_windows::hierarchy::picking::EditorRayCastSource;


pub fn editor_ui_plugin(app: &mut App) {
    app

       .add_plugins(EditorPlugin::default())
        .insert_resource(editor_controls())
        .add_systems(Startup, disable_cam3d_controls) //we handle camera controls on our own 
        ;
}


fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn disable_cam3d_controls(
    mut query: Query<&mut  camera_3d_free::FlycamControls>,
) {
    let mut controls = query.single_mut();
    //controls.key_up = KeyCode::KeyQ;
    //controls.key_down = KeyCode::KeyE;

    controls.enable_movement = false;
    controls.enable_look = false; 
}