use std::time::Duration;

use bevy::prelude::*;
/*use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};
use bevy_inspector_egui::egui::{self, RichText};*/

//use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};

use crate::{doodads::{doodad::DoodadComponent, PlaceDoodadEvent},
 //hierarchy::HierarchyWindow, 
 zones::zone_file::TransformSimple};

 

#[derive(Resource)]
pub struct PlacementResource {

    pub grid_lock_delay_timer: Timer 
    
}

impl Default for PlacementResource {
    fn default() -> Self {
        PlacementResource {
            // Initialize the Timer with some default value, for example 0.5 seconds
            grid_lock_delay_timer: Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }
}

#[derive(Event)]
pub enum PlacementEvent {

    CloneSelectedDoodad,
    GridLockSelectedDoodad(Vec3)

} 


#[derive(Resource )]
pub struct PlacementToolsState{
	pub  randomize_yaw: bool,
	pub random_scale_multiplier : f32,
    
    
}


impl Default for PlacementToolsState {

	fn default() -> Self {
	    Self {
	    	randomize_yaw: false,
	    	random_scale_multiplier: 0.0 
	    }
	}

}