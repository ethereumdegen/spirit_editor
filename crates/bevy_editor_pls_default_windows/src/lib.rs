#![allow(clippy::type_complexity)]
//! Default windows for the editor

use bevy::prelude::*;
 
use doodads::DoodadPlugin;
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent, ZoneResource};

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
pub mod placement;
pub mod zones;

pub struct StandardWindowsPlugin {}
impl Plugin for StandardWindowsPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
            .add_event::<placement::PlacementEvent>()
            .add_event::<ZoneEvent>()
           
            .register_type::<CustomPropsComponent>() //reflect
              .register_type::<CustomProp>() //reflect
            .add_event::<doodads::picking::SelectDoodadEvent>()
            .init_resource::<ZoneResource>()
            .init_resource::<placement::PlacementResource>()
            .add_systems(Update, zones::handle_zone_events)


            .add_plugins(DoodadPlugin {})
             .add_plugins(TransformGizmoPlugin)
          
            .add_systems(Update, placement::update_placement_tool_inputs)
            .add_systems(Update, placement::handle_placement_tool_events)



            ;
    }
}


 