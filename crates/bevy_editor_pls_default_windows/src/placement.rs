use std::time::Duration;

use bevy::prelude::*;
use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};
use bevy_inspector_egui::egui::{self, RichText};

use crate::{doodads::{doodad::DoodadComponent, PlaceDoodadEvent}, hierarchy::HierarchyWindow, zones::zone_file::TransformSimple};

 

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

#[derive(Default)]
pub struct PlacementWindowState {
    pub randomize_yaw: bool,
    pub random_scale_multiplier: f32,
    pub translation_grid_lock_step: Vec3,
}

pub struct PlacementWindow;

impl EditorWindow for PlacementWindow {
    type State = PlacementWindowState;
    const NAME: &'static str = "Placement";

    fn ui(_world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<PlacementWindow>().unwrap();

        //  let placement_resource = world.resource::<PlacementResource>();

        ui.vertical(|ui| {
            ui.label("Randomize Rotation (Yaw)");
            if ui.checkbox(&mut state.randomize_yaw, "").changed() {
                // state.randomize_yaw = !state.randomize_yaw;
            }
            ui.end_row();

            ui.label("Random Scale Multiplier");
            let mut scale_mult = state.random_scale_multiplier;
            if ui
                .add(
                    egui::DragValue::new(&mut scale_mult)
                        .clamp_range(0..=1)
                        .speed(0.01),
                )
                .changed()
            {
                state.random_scale_multiplier = scale_mult;
            }
            ui.end_row();

           
            ui.label("Translation Grid Lock Step [ key: LShift ]"); 
             ui.horizontal(|ui| {
            let mut lock_step = state.translation_grid_lock_step;
            if ui
                .add(
                    egui::DragValue::new(&mut lock_step.x)
                        .clamp_range(0..=10)
                        .speed(0.1),
                )
                .changed()
            {
                state.translation_grid_lock_step.x = lock_step.x;
            }

             let mut lock_step = state.translation_grid_lock_step;
            if ui
                .add(
                    egui::DragValue::new(&mut lock_step.y)
                        .clamp_range(0..=10)
                        .speed(0.1),
                )
                .changed()
            {
                state.translation_grid_lock_step.y = lock_step.y;
            }

             let mut lock_step = state.translation_grid_lock_step;
            if ui
                .add(
                    egui::DragValue::new(&mut lock_step.z)
                        .clamp_range(0..=10)
                        .speed(0.1),
                )
                .changed()
            {
                state.translation_grid_lock_step.z = lock_step.z;
            }

          });    

             ui.end_row();
              ui.label("Additional shortcuts");
               ui.label("Clone selected doodad: [ key: ctrl+C ]");

        }); // ---- v
    }
}
    



 

pub fn update_placement_tool_inputs(
  key_inputs: Res<ButtonInput<KeyCode> >,

  mut placement_evt_writer: EventWriter<PlacementEvent>,

  mut placement_resource: ResMut<PlacementResource>,
  time: Res<Time>,

  editor: Res<Editor>,
){


  placement_resource.grid_lock_delay_timer.tick(time.delta());


  if key_inputs.pressed(KeyCode::ShiftLeft) {
        let placement_window_state = editor.window_state::<PlacementWindow>().unwrap();

        let grid_step = placement_window_state.translation_grid_lock_step;

        if placement_resource.grid_lock_delay_timer.finished() { //only grid lock immediately and then each second 
            placement_resource.grid_lock_delay_timer.reset();
            println!("grid lock ");
            placement_evt_writer.send(PlacementEvent::GridLockSelectedDoodad(grid_step) );
        }
  } else{
    placement_resource.grid_lock_delay_timer.set_elapsed(Duration::from_secs_f32(1.0));
  }


  if key_inputs.just_pressed(KeyCode::KeyC) {
     if key_inputs.pressed(KeyCode::ControlLeft) {

        placement_evt_writer.send(PlacementEvent::CloneSelectedDoodad);
    }
  }
   


}


/*

    mut contexts: EguiContexts,

    editor: Res<Editor>,
) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let clicking_in_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !clicking_in_viewport {
        return;
    }


*/

pub fn handle_placement_tool_events(  
 
  mut placement_evt_reader: EventReader<PlacementEvent>,
  mut place_doodad_evt_writer: EventWriter<PlaceDoodadEvent>, 

  mut  doodad_query: Query< (Entity, &Name, &DoodadComponent, &mut Transform), With<DoodadComponent>  >,


 
    editor: Res<Editor>,


) {

   // let egui_ctx = contexts.ctx_mut();
    let selected_entities = &editor.window_state::<HierarchyWindow>().unwrap().selected;




    for evt in placement_evt_reader.read() {



        match evt {
            PlacementEvent::CloneSelectedDoodad =>  {

                //clone the selected doodad and select the newly cloned one 

                let first_selected_entity = selected_entities.iter().next();

                if let Some((entity, name_comp, doodad_comp, doodad_xform)) = first_selected_entity.and_then(|ent|  doodad_query.get(ent).ok() ) {

                  //  let mut translation = doodad_xform.translation ;

                    let simple_xform:TransformSimple = doodad_xform.clone().into();

                    place_doodad_evt_writer.send(
                        PlaceDoodadEvent {
                             position: simple_xform.translation, 
                             scale: Some(simple_xform.scale), 
                             rotation_euler: Some(simple_xform.rotation), 
                             doodad_name: name_comp.to_string().clone(),
                             custom_props: None ,
                             zone: None 
                      });



                }



            },
            PlacementEvent::GridLockSelectedDoodad(grid_step) => {

                //set the transform to within a near step 

                  let first_selected_entity = selected_entities.iter().next();

                if let Some((_, _, _, mut doodad_xform)) = first_selected_entity.and_then(|ent|  doodad_query.get_mut(ent).ok() ) {

                    let mut translation = doodad_xform.translation ; 

                    if *&grid_step.x >= 0.02 {
                          translation.x = (translation.x / grid_step.x).round() * grid_step.x;
                    }

                      if *&grid_step.y >= 0.02 {
                          translation.y = (translation.y / grid_step.y).round() * grid_step.y;
                    }

                      if *&grid_step.z >= 0.02 {
                          translation.z = (translation.z / grid_step.z).round() * grid_step.z;
                    }

                   
                   

                    doodad_xform.translation = translation;

                    

                }


            }
        }




    }


}