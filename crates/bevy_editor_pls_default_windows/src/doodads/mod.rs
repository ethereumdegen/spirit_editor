use crate::doodads::doodad_placement_preview::DoodadPlacementPlugin;
use bevy::{asset::ReflectAsset, reflect::TypeRegistry};

use bevy::prelude::*;
use bevy_mod_raycast::immediate::RaycastSettings;
use rand::Rng;

use bevy::utils::HashMap;


use crate::doodads::doodad_manifest::RenderableType;
use crate::placement::PlacementWindow;
use crate::zones::zone_file::{CustomPropsComponent,CustomPropsMap};
use crate::zones::ZoneResource;
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls_core::{Editor, EditorEvent};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, ScrollArea};

use bevy_common_assets::ron::RonAssetPlugin;

use bevy_mod_raycast::cursor::CursorRay;

use bevy_mod_raycast::prelude::Raycast;

use self::doodad::{DoodadComponent,    };
use self::doodad_manifest::{DoodadDefinition, DoodadManifest, DoodadDefinitionsResource, DoodadTagMapResource};
use self::doodad_placement_preview::DoodadPlacementComponent;

 

pub mod doodad_manifest;
pub mod picking;
pub mod doodad;
pub mod doodad_placement_preview;


pub struct DoodadPlugin {}
impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app

             .add_event::< PlaceDoodadEvent>()
               .add_event::< DoodadToolEvent>()

            //.init_resource::<DoodadTagMapResource>()


              .add_plugins(DoodadPlacementPlugin {} )
             .add_systems(Update, update_place_doodads)
         
           
            .add_systems(Update, reset_place_doodads)
            .add_systems(Update, (handle_place_doodad_events,handle_doodad_tool_events , replace_proto_doodads_with_doodads).chain()  )
            .add_systems(Update, picking::update_picking_doodads)
           

            ;
    }
}


 




#[derive(Resource, Default)]
pub struct DoodadToolState {
    pub selected: Option<String>,
}



#[derive(Event)]
pub enum DoodadToolEvent {
    SetSelectedDoodad(Option<String>)
}

#[derive(Component)]
pub struct DoodadProto;


#[derive(Component)]
pub struct DoodadNeedsModelAttached;



#[derive(Event)]
pub struct PlaceDoodadEvent {
    pub position: Vec3,
    pub scale: Option<Vec3>,
    pub rotation_euler: Option<Vec3>,
    pub doodad_name: String,
    pub custom_props: Option<CustomPropsMap>,
    pub zone: Option<Entity> 
    // pub doodad_definition: DoodadDefinition
}

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


pub fn handle_place_doodad_events(
    mut commands: Commands,

    mut evt_reader: EventReader<PlaceDoodadEvent>,

    mut editor_event_writer: EventWriter<EditorEvent>,
    mut doodad_tool_event_writer: EventWriter<DoodadToolEvent>,

    zone_resource: Res<ZoneResource>,

   // doodad_manifest_resource: Res<DoodadManifestResource>,
   // doodad_manifest_assets: Res<Assets<DoodadManifest>>,
) {
   // let manifest_handle = &doodad_manifest_resource.manifest;


   // let manifest = manifest_handle.as_ref().map( |handle| doodad_manifest_assets.get(handle) ).flatten();

    for evt in evt_reader.read() {
        let position = &evt.position;
        let doodad_name = &evt.doodad_name;


 

        let mut transform = Transform::from_xyz(position.x, position.y, position.z);

        if let Some(rot) = evt.rotation_euler {
            transform =
                transform.with_rotation(Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z))
        }
        if let Some(scale) = evt.scale {
            transform = transform.with_scale(scale)
        }

        let doodad_spawned = commands
            .spawn(SpatialBundle {
                transform,
                ..default()
            })
            .insert(Name::new(doodad_name.clone())  )
            .insert( DoodadProto )
            .id();


        editor_event_writer.send( 
            EditorEvent::SetSelectedEntities(Some(vec![ doodad_spawned ]))
         );

        doodad_tool_event_writer.send(
            DoodadToolEvent::SetSelectedDoodad(None) 
        );

        println!("doodad spawned {:?}", doodad_spawned);

        
            //from cloning ! 
        let proto_custom_props_to_attach = match &evt.custom_props {
            Some(props) => Some( props ),
            None  => None
        };


         if let Some(custom_props) = proto_custom_props_to_attach {
          //  println!("insert custom props {:?}", init_custom_props);

            commands
                .entity(doodad_spawned)
                .insert(CustomPropsComponent {
                    props: custom_props.clone(),
                });
        }else{
             commands
                .entity(doodad_spawned)
                .insert( CustomPropsComponent::default()  );
        }




         if let Some(zone_override) = &evt.zone {
            if let Some(mut ent) = commands.get_entity(zone_override.clone()) {
                ent.add_child(doodad_spawned);
            }
        }else  if let Some(primary_zone) = &zone_resource.primary_zone {
            if let Some(mut ent) = commands.get_entity(primary_zone.clone()) {
                ent.add_child(doodad_spawned);
            }
        }
    }
}




pub fn replace_proto_doodads_with_doodads(
    mut commands: Commands,

    mut doodad_proto_query: Query<(Entity,&Name,Option<&mut CustomPropsComponent>), With<DoodadProto>>,
 

  //  zone_resource: Res<ZoneResource>,

    doodad_definition_resource: Res<DoodadDefinitionsResource>,
   // doodad_manifest_assets: Res<Assets<DoodadManifest>>,
) {
   // let manifest_handle = &doodad_manifest_resource.manifest;


    //let manifest = manifest_handle.as_ref().map( |handle| doodad_manifest_assets.get(handle) ).flatten();

    for (doodad_entity,doodad_name,existing_custom_props_comp) in doodad_proto_query.iter_mut() {
       
       let doodad_name = doodad_name.as_str().to_string();

        let Some(doodad_definition) = doodad_definition_resource.get_doodad_definition_by_name(&doodad_name)  else {
            println!("WARN: Could not replace doodad proto {:?}", doodad_name);
 
            continue;
        };

        let  custom_props_from_manifest = &doodad_definition.initial_custom_props ;

 

 

        let doodad_spawned = commands
            .entity( doodad_entity)  
            .insert(DoodadComponent::from_definition(&doodad_definition))
            .remove::<DoodadProto>()
            .insert(DoodadNeedsModelAttached)
            .id();


    

        println!("doodad spawned {:?}", doodad_spawned);

        if let Some( mut existing_custom_props_comp ) = existing_custom_props_comp {
             if let Some(custom_props) = custom_props_from_manifest {  

                existing_custom_props_comp.set_custom_props_if_empty(custom_props);
            }


        }else {

             if let Some(custom_props) = custom_props_from_manifest {  
              commands
                .entity(doodad_spawned)
                .insert(CustomPropsComponent {
                    props: custom_props.clone(),
                });
             }else {

                 commands
                .entity(doodad_spawned)
                .insert(CustomPropsComponent::default() );

             }

        }

 
        //do stuff w existing_custom_props_comp ? 
      


       
    }
}



pub fn update_place_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    mut event_writer: EventWriter<PlaceDoodadEvent>,

    doodad_tool_resource: Res<DoodadToolState>,

    mut contexts: EguiContexts,

    editor: Res<Editor>,

     doodad_placement_component_query: Query<&Transform, With<DoodadPlacementComponent>>,
       parent_query: Query<&Parent >
) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let clicking_in_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !clicking_in_viewport {
        return;
    }

    // ------- compute our rotation and scale from placement properties
    let placement_window_state = editor.window_state::<PlacementWindow>().unwrap();

    let using_random_yaw = placement_window_state.randomize_yaw;
    let random_scale_multiplier = placement_window_state.random_scale_multiplier;

    let mut rng = rand::thread_rng();

    let rotation_euler: Option<Vec3> = match using_random_yaw {
        true => {
            let random_f32 = rng.gen_range(0.0..1.0);
            Some(( random_f32 * 3.14, 0.0, 0.0).into())
        }
        false => None,
    };

    let scale: Option<Vec3> = match random_scale_multiplier >= 0.001 {
        true => {
            let random_f32 = rng.gen_range(-1.0..1.0);
            let random_scaled_f32 = 1.0 + random_scale_multiplier * random_f32;

            Some((random_scaled_f32, random_scaled_f32, random_scaled_f32).into())
        }

        false => None,
    };
    // -------------------------

    let selected_doodad_definition = &doodad_tool_resource.selected;

    let Some(doodad_definition_name) = selected_doodad_definition.clone() else {
        return;
    };

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }


    let raycast_filter = |entity: Entity| {

        
         let mut current_entity = entity;
        loop {
            if doodad_placement_component_query.get(current_entity).is_ok() {
                return false;
            }
            match parent_query.get(current_entity).ok() {
                Some(parent) => current_entity = parent.get(),
                None => break,
            }
        }
        true
    };

    let raycast_settings = RaycastSettings {
        filter: &raycast_filter,
        ..default()
    };



    if let Some(cursor_ray) = **cursor_ray {
        if let Some((_intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &raycast_settings).first()
        {
            let hit_point = intersection_data.position();

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec3::new(hit_point.x, hit_point.y, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

            let custom_props = None; 

            event_writer.send(PlaceDoodadEvent {
                position: hit_coordinates,
                doodad_name: doodad_definition_name,
                rotation_euler,
                scale,
                custom_props,
                zone: None
            });
        }
    }
}

pub fn reset_place_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    mut doodad_tool_resource: ResMut<DoodadToolState>,
    //  mut contexts: EguiContexts,
) {
    //let egui_ctx = contexts.ctx_mut();
    /*
    if egui_ctx.is_pointer_over_area() {
        return;
    }

    */

    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }

    doodad_tool_resource.selected = None;
}



pub fn handle_doodad_tool_events(
    mut event_reader: EventReader<DoodadToolEvent>,

    mut doodad_tool_resource: ResMut<DoodadToolState>
) {
    for evt in event_reader.read(){

        match evt {
            DoodadToolEvent::SetSelectedDoodad(doodad_name) => {


                doodad_tool_resource.selected = doodad_name.clone();
            }
        }



    }
}
