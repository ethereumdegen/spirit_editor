
use bevy_editor_pls_core::Editor;
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

       //.add_systems(Update,  update_debug_physics )

	;

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