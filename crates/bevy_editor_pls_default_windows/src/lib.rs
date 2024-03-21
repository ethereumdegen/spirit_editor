#![allow(clippy::type_complexity)]
//! Default windows for the editor

use bevy::prelude::*;
 
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent, ZoneResource};

 


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
            .add_event::<doodads::PlaceDoodadEvent>()
            .register_type::<CustomPropsComponent>() //reflect
              .register_type::<CustomProp>() //reflect
            .add_event::<doodads::picking::SelectDoodadEvent>()
            .init_resource::<ZoneResource>()
            .init_resource::<placement::PlacementResource>()
            .add_systems(Update, zones::handle_zone_events)
            .add_systems(Update, doodads::update_place_doodads)
           
            .add_systems(Update, doodads::reset_place_doodads)
            .add_systems(Update, doodads::handle_place_doodad_events)
            .add_systems(Update, doodads::picking::update_picking_doodads)
            .add_systems(Update, placement::update_placement_tool_inputs)
            .add_systems(Update, placement::handle_placement_tool_events)



            ;
    }
}


 