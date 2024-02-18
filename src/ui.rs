use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::EguiContexts;


pub fn editor_ui_plugin(app: &mut App){
    app 
    .init_resource::<EditorToolsWindow>()
    .add_plugins(EguiPlugin)
    .add_systems(Update, editor_tools)
  
    ; 
}



#[derive(Default, Resource)]
struct EditorToolsState {
    name: String,
    age: u32,
}



 
fn editor_tools(
    mut tools_state: ResMut<EditorToolsState>,

    mut contexts: EguiContexts

) {
    egui::Window::new("Editor Tools").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Click me").clicked() {
            println!("Button clicked");
        }

        ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut tools_state.name)
                .labelled_by(name_label.id);
        });
        ui.add(egui::Slider::new(&mut tools_state.age, 0..=120).text("age"));

    });
} 