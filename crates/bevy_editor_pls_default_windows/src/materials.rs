use crate::materials::egui::ScrollArea;
use spirit_edit_core::zones::zone_file::CustomProp;
use spirit_edit_core::zones::zone_file::CustomPropsComponent;

use bevy::prelude::*;
use bevy_editor_pls_core::{editor_window::{EditorWindow, EditorWindowContext}, Editor};
use bevy_inspector_egui::egui::{self};

use crate::{   hierarchy::HierarchyWindow   };

 
 
#[derive(Default)]
pub struct MaterialsWindowState {
    //pub randomize_yaw: bool,
    //pub random_scale_multiplier: f32,
    //pub translation_grid_lock_step: Vec3,

    pub selected_material :  Option<String> ,
}

//need to make this update PlacementToolsState !! 

#[derive(Event,Clone)]
pub enum MaterialEvent {

	SetSelectedMaterial (String),

}


pub struct MaterialsWindow;

impl EditorWindow for MaterialsWindow {
    type State = MaterialsWindowState;
    const NAME: &'static str = "Materials";

    fn ui( world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<MaterialsWindow>().unwrap();

        //  let placement_resource = world.resource::<PlacementResource>();


        /*let placement_resource = world.resource::<PlacementResource>();
        let placement_parent_entity = placement_resource.placement_parent;

        let placement_parent_name = placement_parent_entity
            .and_then(|ent| {
                // Temporarily fetch the component to avoid holding the borrow
                world.get::<Name>(ent).map(|n| n.as_str().to_owned())
            })
            .unwrap_or_else(|| "None".to_owned());

				*/

		let material_names = vec![
		"Wall1",
		"Wall2",
		"Rocks2",
		"Cliff1",
		"Cliff2",
		"Cliff3",
        "Cliff3_blue",
		"Cliff4",
		"Cliff5",
        "Cliff5_blue",
		"Cobbles1",
		"Cobbles2",
		"Cobbles3",
		"Wood2",
		"Wood5",

		];

		let mut events_to_send=  Vec::new();


        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if material_names.is_empty()  {
                    ui.label(format!(" No material types found. "));
                    return;
                };
 
 
                ui.separator();
 
                for material_name in material_names.iter() {
  

                 let label_text = material_name.clone();
                    let checked = false;

                    if ui.selectable_label(checked, label_text.clone()).clicked() {
                        //*selection = InspectorSelection::Entities ;

                        println!("detected a material select !! {:?}", label_text);
                         events_to_send.push(  MaterialEvent::SetSelectedMaterial(  material_name.to_string().clone()  )    );

                      //  doodad_tool_resource.selected = Some(doodad_name.clone());
                    }



                }


            }); //end ui 


        world.send_event_batch( events_to_send );



    }
}
    


 
  

pub fn handle_selected_material_events(  
 
  mut material_evt_reader: EventReader<MaterialEvent>,
  
  //mut  doodad_query: Query< (Entity, &Name, &DoodadComponent, &mut Transform, Option<&Parent>), With<DoodadComponent>  >,

 
  editor: Res<Editor>,


 mut  custom_props_query: Query<&mut CustomPropsComponent>,


) {

   // let egui_ctx = contexts.ctx_mut();
    let selected_entities = &editor.window_state::<HierarchyWindow>().unwrap().selected;




    for evt in material_evt_reader.read() {


            //be able to clone prefab ? 
        match evt {
            MaterialEvent::SetSelectedMaterial(mat_name) =>  {

                //clone the selected doodad and select the newly cloned one 

               for selected_entity in selected_entities.iter( ) {
 

               	  if let Some(mut custom_props)  = custom_props_query.get_mut(selected_entity).ok(){

               	  		custom_props.props.insert("material_override".to_string(), CustomProp::String(mat_name.clone()) );

               	  }


               }

                


            },
           

           
           // _ => {} 
        }




    }


}