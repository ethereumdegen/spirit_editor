





 use bevy_egui::EguiContexts;
use bevy::prelude::*;

use bevy_editor_pls_core::{
    editor_window::{EditorWindow, EditorWindowContext},
    Editor, EditorEvent,
};

pub fn editor_state_plugin(app: &mut App) {
    app

    .init_resource::<EditorStateResource>()
    .add_systems(Update, (
        update_cursor_over_gui,
       

        ).chain())
         ;
}


#[derive(Resource, Default)]
 pub struct EditorStateResource{


 	pub cursor_overlaps_gui: bool,
  
 }


fn update_cursor_over_gui( 
	mut editor_state_resource: ResMut<EditorStateResource>,
    mut contexts: EguiContexts,
){


    let egui_ctx = contexts.ctx_mut();  

    editor_state_resource.cursor_overlaps_gui = egui_ctx.is_pointer_over_area();


} 