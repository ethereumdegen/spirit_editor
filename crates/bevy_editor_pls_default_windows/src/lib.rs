#![allow(clippy::type_complexity)]
//! Default windows for the editor

use bevy::prelude::*;
 
 use transform_gizmo_bevy::TransformGizmoPlugin;


pub mod add;
pub mod assets;
pub mod cameras;
pub mod debug_settings;
pub mod diagnostics;
pub mod gizmos;
pub mod hierarchy;
pub mod inspector;
pub mod renderer;
pub mod resources;
pub mod scenes;
pub mod lighting;

 pub mod doodads;
 pub mod prefabs;
 pub mod placement;
 pub mod zones;

pub struct StandardWindowsPlugin {}
impl Plugin for StandardWindowsPlugin {
    fn build(&self, app: &mut App) {
        
         app
            
 
             .add_plugins(TransformGizmoPlugin)
              .add_systems(Startup, (
               gizmos::update_gizmo_options
                ) .chain()
            )

            .add_systems(Update, (
                doodads::update_picking_doodads ,
                placement::update_placement_tool_inputs,
                placement::handle_placement_tool_events
                ) .chain()
            )

              
            ;
    }
}


 