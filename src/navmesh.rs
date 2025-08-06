
use avian3d::prelude::PhysicsGizmos;
use bevy_editor_pls_core::Editor;

use bevy::input::common_conditions::*; 


use avian_rerecast::prelude::*;
use bevy_rerecast::{debug::DetailNavmeshGizmo, prelude::*};

use bevy_editor_pls_default_windows::debug_settings::DebugSettingsWindow;
 
use avian3d::PhysicsPlugins;
 
use bevy::prelude::*; 

pub fn navmesh_plugin(app: &mut App) {

		app.add_plugins((NavmeshPlugins::default(), AvianBackendPlugin::default()));

			 	
		app.add_systems(
		    Update,
		    generate_navmesh.run_if(input_just_pressed(KeyCode::F2)),
		);

}


#[derive(Resource)]
#[allow(dead_code)]
struct NavmeshHandle(Handle<Navmesh>);

fn generate_navmesh(mut generator: NavmeshGenerator, mut commands: Commands) {
    let config = NavmeshConfigBuilder::default();
    let navmesh = generator.generate(config);

    // Spawn visualization gizmos
    commands.spawn(DetailNavmeshGizmo::new(&navmesh));

    // Store the handle somewhere so it doesn't get dropped
    commands.insert_resource(NavmeshHandle(navmesh));
}
