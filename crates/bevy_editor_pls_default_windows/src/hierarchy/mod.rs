// pub mod picking;

 // use crate::debug_settings::DebugSettingsWindow;
use spirit_edit_core::doodads::rotate::RotateByDegrees;
use spirit_edit_core::prefabs::{PrefabComponent,SavePrefabToFileEvent};
use spirit_edit_core::zones::SaveZoneToFileEvent;
use spirit_edit_core::zones::ZoneComponent;
use spirit_edit_core::zones::ZoneEvent;


use bevy::ecs::entity::Entities;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::{Extract, RenderApp};
use bevy_editor_pls_core::EditorEvent;
use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::egui::text::CCursorRange;
use bevy_inspector_egui::egui::{self, ScrollArea};

use bevy::ecs::relationship::Relationship;

 use spirit_edit_core::placement::PlacementEvent;


use bevy_editor_pls_core::{
    editor_window::{EditorWindow, EditorWindowContext},
    Editor,
};
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::{IsPointerEvent, PointerClick, PointerButton};

use crate::add::{add_ui, AddWindow, AddWindowState};
//use crate::debug_settings::DebugSettingsWindow;
use crate::inspector::{InspectorSelection, InspectorWindow};
 

#[derive(Component)]
pub struct HideInEditor;

pub struct HierarchyWindow;
impl EditorWindow for HierarchyWindow {
    type State = HierarchyState;
    const NAME: &'static str = "Hierarchy";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let (hierarchy_state, inspector_state, add_state) =
            match cx.state_mut_triplet::<HierarchyWindow, InspectorWindow, AddWindow>() {
                Some((a, b, c)) => (a, b, Some(c)),
                None => {
                    let (a, b) = cx
                        .state_mut_pair::<HierarchyWindow, InspectorWindow>()
                        .unwrap();
                    (a, b, None)
                }
            };

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let type_registry = world.resource::<AppTypeRegistry>().clone();
                let type_registry = type_registry.read();
                let new_selected = Hierarchy {
                    world,
                    state: hierarchy_state,
                    type_registry: &type_registry,
                    add_state: add_state.as_deref(),
                }
                .show(ui);

                if new_selected {
                    inspector_state.selected = InspectorSelection::Entities;
                }
            });
    }

    fn app_setup(app: &mut bevy::prelude::App) {
        // picking::setup(app);
        app.add_systems(PostUpdate, clear_removed_entites)
            .add_systems(Update, listen_for_select_entities_events);

        // .add_system(handle_events);

      //  app.sub_app_mut(RenderApp)
      //      .add_systems(ExtractSchedule, extract_wireframe_for_selected);
    }
}

fn clear_removed_entites(mut editor: ResMut<Editor>, entities: &Entities) {
    let state = editor.window_state_mut::<HierarchyWindow>().unwrap();
    state.selected.retain(|entity| entities.contains(entity));
}

/*fn handle_events(
    mut click_events: EventReader<PointerClick>,
    mut editor: ResMut<Editor>,
    editor_state: Res<EditorState>,
    input: Res<Input<KeyCode>>,
    egui_entity: Query<&EguiPointer>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    for click in click_events.iter() {
        if !editor_state.active {
            return;
        }

        if click.event_data().button != PointerButton::Primary {
            continue;
        }

        if egui_entity.get(click.target()).is_ok() || egui_ctx.ctx_mut().wants_pointer_input() {
            continue;
        };

        let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

        let ctrl = input.any_pressed([KeyCode::LControl, KeyCode::RControl]);
        let shift = input.any_pressed([KeyCode::LShift, KeyCode::RShift]);
        let mode = SelectionMode::from_ctrl_shift(ctrl, shift);

        let entity = click.target();
        info!("Selecting mesh, found {:?}", entity);
        state
            .selected
            .select(mode, entity, |_, _| std::iter::once(entity));
    }
}*/

/*
fn extract_wireframe_for_selected(editor: Extract<Res<Editor>>, mut commands: Commands) {
   
   /* let wireframe_for_selected = editor
        .window_state::<DebugSettingsWindow>()
        .map_or(false, |settings| settings.highlight_selected);
        */

   

    /* if wireframe_for_selected {
        let selected = &editor.window_state::<HierarchyWindow>().unwrap().selected;
        for selected in selected.iter() {
            commands.spawn(selected).insert(Wireframe);
        }
    }*/
}
*/ 

#[derive(Default)]
pub struct HierarchyState {
    pub selected: SelectedEntities,
    rename_info: Option<RenameInfo>,
}

pub struct RenameInfo {
    entity: Entity,
    renaming: bool,
    current_rename: String,
}

struct Hierarchy<'a> {
    world: &'a mut World,
    state: &'a mut HierarchyState,
    type_registry: &'a TypeRegistry,
    add_state: Option<&'a AddWindowState>,
}

impl<'a> Hierarchy<'a> {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut despawn_recursive = None;
        let mut despawn = None;

        let HierarchyState {
            selected,
            rename_info,
        } = self.state;

        let new_selection = bevy_inspector_egui::bevy_inspector::hierarchy::Hierarchy {
            extra_state: rename_info,
            world: self.world,
            type_registry: self.type_registry,
            selected,
            context_menu: Some(&mut |ui, entity, world, rename_info| {
                let entity_is_zone = world.entity(entity).get::<ZoneComponent>().is_some();
                let entity_is_prefab = world.entity(entity).get::<PrefabComponent>().is_some();

                if ui.button("Despawn").clicked() {
                    despawn_recursive = Some(entity);
                }

                if ui.button("Remove keeping children").clicked() {
                    despawn = Some(entity);
                }

                if ui.button("Rename").clicked() {
                    let entity_name = guess_entity_name(world, entity);
                    *rename_info = Some(RenameInfo {
                        entity,
                        renaming: true,
                        current_rename: entity_name,
                    });
                    ui.close_menu();
                }

                if entity_is_zone {
                   
                    if ui.button("Save zone file").clicked() {
                        world.send_event::<SaveZoneToFileEvent>(SaveZoneToFileEvent(entity).into());
                        ui.close_menu();
                    }
                     if ui.button("Set as placement parent").clicked() {
                        world.send_event::<PlacementEvent>(PlacementEvent::SetPlacementParent(Some( entity ) ) );
                        ui.close_menu();
                    }

                }

                 if entity_is_prefab {
                        
                    if ui.button("Save prefab file").clicked() {
                        world.send_event::<SavePrefabToFileEvent>(SavePrefabToFileEvent(entity).into());
                        ui.close_menu();
                    }
                     if ui.button("Set as placement parent").clicked() {
                        world.send_event::<PlacementEvent>(PlacementEvent::SetPlacementParent(Some( entity ) ) );
                        ui.close_menu();
                    }

                }

                if let Some(add_state) = self.add_state {
                    ui.menu_button("Add", |ui| {
                        if let Some(add_item) = add_ui(ui, add_state) {
                            add_item.add_to_entity(world, entity);
                            ui.close_menu();
                        }
                    });
                }
            }),
            shortcircuit_entity: Some(&mut |ui, entity, world, rename_info| {
                if let Some(rename_info) = rename_info {
                    if rename_info.renaming && rename_info.entity == entity {
                        rename_entity_ui(ui, rename_info, world);

                        return true;
                    }
                }

                false
            }),
        }
      //  .show::<Without<HideInEditor>>(ui);
          .show::<(Without<HideInEditor>, Without<Observer>)>(ui);


        if let Some(entity) = despawn_recursive {

           let _despawned =   self.world.despawn( entity );  //also despawns children! 
         //    despawn_with_children_recursive(self.world, entity, false);
        }
        if let Some(entity) = despawn {
            self.world.entity_mut(entity).despawn();
            self.state.selected.remove(entity);
        }


        new_selection
    }
}

fn rename_entity_ui(ui: &mut egui::Ui, rename_info: &mut RenameInfo, world: &mut World) {
    use egui::epaint::text::cursor::CCursor;
    use egui::widgets::text_edit::{TextEdit, TextEditOutput};

    let id = egui::Id::new(rename_info.entity);

    let edit = TextEdit::singleline(&mut rename_info.current_rename).id(id);
    let TextEditOutput {
        response,
        state: mut edit_state,
        ..
    } = edit.show(ui);

    // Runs once to end renaming
    if response.lost_focus() {
        rename_info.renaming = false;

        match world.get_entity_mut(rename_info.entity).ok() {
            Some(mut ent_mut) => match ent_mut.get_mut::<Name>() {
                Some(mut name) => {
                    name.set(rename_info.current_rename.clone());
                }
                None => {
                    ent_mut.insert(Name::new(rename_info.current_rename.clone()));
                }
            },
            None => {
                error!("Failed to get renamed entity");
            }
        }
    }

    // Runs once when renaming begins
    if !response.has_focus() {
        response.request_focus();
        edit_state.cursor.set_char_range(Some(CCursorRange::two(
            CCursor::new(0),
            CCursor::new(rename_info.current_rename.len()),
        )));
    }

    TextEdit::store_state(ui.ctx(), id, edit_state);
}



pub fn listen_for_select_entities_events(  

    mut editor_evt_reader: EventReader<EditorEvent>,


    mut editor: ResMut<Editor>,

    mut commands: Commands , 


    
    ){


    for evt in editor_evt_reader.read(){


        match evt {


            EditorEvent::SetSelectedEntities(entities_to_select) => {
                let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

                state.selected.clear();


                for entity in entities_to_select.clone().unwrap_or(Vec::new()){
                       state.selected.select_maybe_add( entity, true );  
                } 


            }


             EditorEvent::RotateSelectedDoodadByDegrees( degrees_vec ) => {
                 let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

                 
                    for entity in  state.selected.iter() {
                        if let Ok(mut cmd) = commands.get_entity( entity ){
                            cmd.queue( RotateByDegrees( degrees_vec.clone () )  ) ;  // now an entity command ! 
                           // cmd.insert(RotateByDegrees( degrees_vec.clone () ));
                        }
                   
                    }


            }


            EditorEvent::DeleteSelectedEntities => {
                   let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

                //if ui.input(|input| input.key_pressed(egui::Key::Delete)) {
                    for entity in  state.selected.iter() {
                        if let Ok(mut cmd) = commands.get_entity( entity ){

                            cmd.despawn_recursive();
                        }
                   
                    }
                     state.selected.clear();
                //}


            }

            _ => {}
        }


    }


}

/*
pub fn clear_selection(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    mut editor: ResMut<Editor>,
) {
    let state = editor.window_state_mut::<HierarchyWindow>().unwrap();

    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }

    state.selected.clear();
}
*/