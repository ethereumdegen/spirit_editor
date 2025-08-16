#![allow(clippy::type_complexity)]
//! Default windows for the editor

use bevy::prelude::*;
 
 use materials::MaterialNamesResource;
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
pub mod materials; 

 pub mod doodads;
 pub mod prefabs;
 pub mod placement;
 pub mod zones;

pub struct StandardWindowsPlugin {}
impl Plugin for StandardWindowsPlugin {
    fn build(&self, app: &mut App) {
        
         app
             .init_resource::<MaterialNamesResource>()
 
             .add_plugins(TransformGizmoPlugin)
             .add_plugins(doodads::doodads_plugin )

             .add_event::<materials::MaterialEvent>()  // move to core ? 

              .add_systems(Update, (
               gizmos::update_gizmo_options
                ) .chain()
            )

            .add_systems(Update, (
             //   doodads::update_picking_doodads ,
                placement::update_placement_tool_inputs,
                placement::handle_placement_tool_events,
                materials::handle_selected_material_events
                ) .chain()
            )

              
            ;
    }
}


 