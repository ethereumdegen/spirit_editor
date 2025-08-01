
use avian3d::prelude::PhysicsGizmos;
use bevy_editor_pls_core::Editor;

use bevy::input::common_conditions::*; 

use bevy_editor_pls_default_windows::debug_settings::DebugSettingsWindow;
use bevy_egui::EguiContexts;
use avian3d::PhysicsPlugins;
use avian3d::prelude::PhysicsDebugPlugin;
use bevy_editor_pls_default_windows::debug_settings::DebugSettingsWindowState;
use bevy::prelude::*; 

pub fn physics_plugin(app: &mut App) {


	app 

	  .add_plugins(PhysicsPlugins::default())

	  // use this to debug phys ! 
       .add_plugins(PhysicsDebugPlugin::default() )

      

        .add_systems(
		    Update,
		    (|mut gizmo_configs: ResMut<GizmoConfigStore>| {
		        gizmo_configs.config_mut::<PhysicsGizmos>().0.enabled ^= true;
		    })
		    .run_if(run_once.or(input_just_pressed(KeyCode::F1))),
		);


	 

}


/*
pub fn update_debug_physics(

	editor: Res<Editor>,

) {

    let debug_window_state = editor.window_state::<DebugSettingsWindow>().unwrap();
    let debug_physics = &debug_window_state.debug_physics_enabled;
 	
 	if debug_physics {


 	}else {


 	}



}*/