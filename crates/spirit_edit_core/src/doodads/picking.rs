 
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_editor_pls_core::Editor;
use bevy_egui::EguiContexts;
use bevy_mod_raycast::{immediate::Raycast, cursor::CursorRay};

use super::{doodad::DoodadComponent, DoodadToolState, PlaceDoodadEvent};

#[derive(Event)]
pub struct SelectDoodadEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct PreventEditorSelection {}
