use spirit_edit_core::doodads::doodad::DoodadComponent;
use spirit_edit_core::doodads::doodad_manifest::DoodadManifest;
use spirit_edit_core::doodads::doodad_manifest::DoodadTagMapResource;
use spirit_edit_core::doodads::doodad_manifest::DoodadDefinitionsResource;
use spirit_edit_core::doodads::DoodadToolState;
use spirit_edit_core::doodads::DoodadToolEvent;
use spirit_edit_core::doodads::picking::SelectDoodadEvent;
use spirit_edit_core::doodads::picking::PreventEditorSelection;
use crate::hierarchy::HierarchyWindow;
  
 
  
use bevy::{asset::ReflectAsset, reflect::TypeRegistry};

use bevy::prelude::*;
use bevy_mod_raycast::immediate::RaycastSettings;
use rand::Rng;

use bevy::utils::HashMap;


//use crate::doodads::doodad_manifest::RenderableType;
 
//use crate::zones::ZoneResource;

 
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls_core::{Editor, EditorEvent};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, ScrollArea}; 

use bevy_common_assets::ron::RonAssetPlugin;

use bevy_mod_raycast::cursor::CursorRay;

use bevy_mod_raycast::prelude::Raycast;
 

  
 

 

 
#[derive(Default)]
pub struct DoodadWindowState {
    //  pub selected: Option<DoodadDefinition> ,
    //  rename_info: Option<RenameInfo>,
}

pub struct DoodadsWindow;

impl EditorWindow for DoodadsWindow {
    type State = DoodadWindowState;
    const NAME: &'static str = "Doodads";

    /// Necessary setup (resources, systems) for the window.
    fn app_setup(app: &mut App) {
        app.add_plugins(RonAssetPlugin::<DoodadManifest>::new(&["doodadmanifest.ron"]))
            
            .insert_resource(DoodadDefinitionsResource::default())
            .insert_resource(DoodadTagMapResource::default())
            .insert_resource(DoodadToolState::default())
          //  .insert_resource(LoadedGltfAssets::default())

           ;
        
    }

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let doodad_definition_resource = world.resource::<DoodadDefinitionsResource>();

         let doodad_tag_map_resource = world.resource::<DoodadTagMapResource>();


       

        //this releases the lock on World
       // let doodad_manifest_handle = &doodad_definition_resource.manifest.clone();

      //  let doodad_manifests_map = world.resource::<Assets<DoodadManifest>>();

        let doodad_definitions = &doodad_definition_resource.loaded_doodad_definitions;/* doodad_manifest_handle
            .as_ref()
            .and_then(|h| doodad_manifests_map.get(h))
            .cloned();*/

        let   doodad_tool_resource = world.resource::<DoodadToolState>();




        /*
                 let doodad_row_state = match cx.state_mut::<DoodadsWindow >() {
                        Some(a) => a,
                        None => {
                            let a = cx
                                .state_mut ::<DoodadsWindow   >()
                                .unwrap();
                            a
                        }
                    };
        */



        let mut events_to_send=  Vec::new();


        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if doodad_definitions.is_none()  {
                    ui.label(format!(" No doodad definitions found. "));
                    return;
                };

                  




                if let Some(selected_doodad_name) = &doodad_tool_resource.selected {
                    ui.label(format!("Placing: {:?}", selected_doodad_name.clone()));

                    ui.separator();

                    if ui.button("reset").clicked() {
                       //    doodad_tool_resource.selected = None;

                      events_to_send.push(  DoodadToolEvent::SetSelectedDoodad( None )    );
                    }
                } else {
                    ui.label("---");
                }

                ui.separator();




                let doodad_tag_map = &doodad_tag_map_resource.doodad_tag_map;
                
                for doodad_tag in doodad_tag_map.keys() {

                    if let Some(doodads_with_tag) = &doodad_tag_map.get(doodad_tag) {
                    egui::CollapsingHeader::new(doodad_tag)
                        .default_open(false)
                        .show(ui, |ui| {
                            for doodad_name in doodads_with_tag.iter() {




                                 let label_text = doodad_name.clone();
                                    let checked = false;

                                    if ui.selectable_label(checked, label_text.clone()).clicked() {
                                        //*selection = InspectorSelection::Entities ;

                                        println!("detected a doodad click  !! {:?}", label_text);
                                         events_to_send.push(  DoodadToolEvent::SetSelectedDoodad( Some( doodad_name.clone() ) )    );

                                      //  doodad_tool_resource.selected = Some(doodad_name.clone());
                                    }




                                 
                            }
                        });
                    }




                }


            }); //end ui 


        world.send_event_batch( events_to_send );
    }
}

// --------------------------------------------------------

 
pub fn update_picking_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

  //  key_input: Res<ButtonInput<KeyCode>>,

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    mut event_writer: EventWriter<SelectDoodadEvent>,

    mut editor: ResMut<Editor>,

    unpickable_query: Query<&PreventEditorSelection>,
    doodad_comp_query: Query<&DoodadComponent>,
    parent_query: Query<&Parent>,
) {
    let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    //shift mutes doodad selection so you can use the gizmo
    /* if key_input.pressed(KeyCode::ShiftLeft) {
        return ;
    }*/

    //must deselect w right click first
    if !state.selected.is_empty() {
        return;
    }

    if let Some(cursor_ray) = **cursor_ray {
        if let Some((intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &default()).first()
        {
            let hit_point = intersection_data.position();

            if unpickable_query.get(*intersection_entity).ok().is_some() {
                println!("This entity is marked as non-selectable");
                return;
            }

            let mut top_doodad_comp_parent_entity: Option<Entity> = None;
            for parent_entity in AncestorIter::new(&parent_query, *intersection_entity) {
                if unpickable_query.get(parent_entity).ok().is_some() {
                    println!("This entity is marked as non-selectable");
                    return;
                }

                if doodad_comp_query.get(parent_entity).ok().is_some() {
                    top_doodad_comp_parent_entity = Some(parent_entity);
                    break;
                }
            }
            println!("select doodad   {:?}", hit_point);

            let focus_entity = top_doodad_comp_parent_entity.unwrap_or(intersection_entity.clone());

            state.selected.select_replace(focus_entity.clone());

            event_writer.send(SelectDoodadEvent {
                entity: focus_entity.clone(),
            });
            //  }
            //
        }
    }
}
