 
use spirit_edit_core::doodads::doodad::RebuildDoodad;
use bevy_clay_tiles::bevy_material_tool::material_overrides::{MaterialOverrideComponent,MaterialOverrideWhenSceneReadyComponent};
use crate::doodads::doodad_placement_preview::DoodadPlacementComponent;
use crate::doodads::doodad_placement_preview::GhostlyMaterialMarker;
use bevy_editor_pls_core::Editor;
use spirit_edit_core::doodads::DoodadToolState;
use spirit_edit_core::placement::PlacementToolsState;
use bevy_egui::EguiContexts;
use bevy_mod_raycast::prelude::*;
use spirit_edit_core::doodads::DoodadToolEvent;
use spirit_edit_core::zones::ZoneResource;
use spirit_edit_core::doodads::DoodadProto;
use spirit_edit_core::zones::zone_file::CustomPropsComponent;
use spirit_edit_core::doodads::PlaceDoodadEvent;
use bevy_editor_pls_core::EditorEvent;
use spirit_edit_core::doodads::doodad::DoodadComponent;
use spirit_edit_core::doodads::doodad_manifest::RenderableType;
use spirit_edit_core::doodads::DoodadNeedsModelAttached;
use spirit_edit_core::doodads::doodad_manifest::DoodadDefinitionsResource; 
use crate::AssetLoadState;


use bevy_clay_tiles::bevy_material_tool::material_overrides::RefreshMaterialOverride;
 
 
use crate::asset_loading::BuiltVfxHandleRegistry;
use bevy::utils::Duration;
 
use bevy::pbr::wireframe::WireframeColor;
use bevy::{pbr::wireframe::Wireframe, prelude::*, utils::HashMap};


use bevy_mod_sysfail::*;
 
 use rand::Rng;


//use anyhow::{Context, Result};

 

use bevy_mod_picking::prelude::*;
 use bevy_magic_fx::magic_fx::MagicFxVariantComponent;

use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    scene::SceneInstanceReady,
};

use crate::{ 
    liquid::LiquidPlaneComponent};


 

#[derive(Default)]
pub(crate) struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app
          //.insert_resource(LoadedGltfAssets::default())
            .add_systems(Update, (
                attach_models_to_doodads.run_if(in_state(AssetLoadState::Complete)), 
                add_doodad_collider_markers, 
                hide_doodad_collision_volumes,

                remove_recently_failed_to_load,

                update_doodad_placement_preview_model.run_if(in_state(AssetLoadState::Complete)),

                handle_place_doodad_events,
                update_place_doodads,
                reset_place_doodads,
                handle_doodad_tool_events,
                replace_proto_doodads_with_doodads,

               // handle_doodad_scene_ready




              //  add_wireframe_to_children

                ) .chain()  );

 
            

    }
}

#[derive(Component, Default)]
pub struct RecentlyFailedToLoadModel {

    created_at : Duration 

}



#[derive(Component, Default)]
pub struct DoodadColliderMarker {}

const MISSING_MODEL_CUBE_COLOR:Color = Color::rgb(0.9, 0.4, 0.9) ;

 
fn attach_models_to_doodads(
    mut commands: Commands,
    added_doodad_query: Query<
        (Entity,   &DoodadComponent),
        (
            With<DoodadNeedsModelAttached>,
            With<GlobalTransform>,
            Without<Handle<Mesh>>,
            Without<RecentlyFailedToLoadModel>,
        ),
    >,

   // models: Res<Assets<Gltf>>,
  //  gltf_assets: Res<GltfAssets>,

   asset_server: Res<AssetServer>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

      built_vfx_registry: Res<BuiltVfxHandleRegistry>,
    time: Res<Time>, 
) {
    
    for (new_doodad_entity,  doodad_component) in added_doodad_query.iter() {
     //   let doodad_name = &name_comp.to_string();

      //  let doodad_name_clone = doodad_name.clone();
      //  let name_comp = Name::new(doodad_name_clone);

        commands
            .entity(new_doodad_entity)
        //    .insert(name_comp)
            .insert(PickableBundle::default()) 

            ;


        let material_override = &doodad_component.definition.material_override; 

         

        //handle attaching renderable components based on the renderable type - this lets us see the doodad in the editor
        match (&doodad_component.definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

                let doodad_name_stem = format!("{}#Scene0", model_name);

                 let model_handle = asset_server.load(doodad_name_stem);



                 let scene = commands
                    .spawn(SceneBundle {
                        scene: model_handle,
                        ..Default::default()
                    })
                   
                    
                  //  .insert(
                   //     Visibility::Hidden, //make it hidden until we deal w the colliders
                   // )
                    .id();



                    
                       commands.entity(new_doodad_entity)
                              .remove::<DoodadNeedsModelAttached>()
                                .remove::<RecentlyFailedToLoadModel>()
  
                               .add_child( scene  )
                               
                                 ; 




                    if let Some( material_override  ) = material_override  {
                         info!("found mat override  {:?}", material_override );

                        commands.entity(new_doodad_entity).insert(
                            MaterialOverrideWhenSceneReadyComponent {
                                material_override: material_override.clone() 
                            }

                        );

                    }


              /* match get_loaded_model_from_name(model_name, &gltf_assets, &models){

                        Ok(loaded_model)=> {

                             commands.entity(new_doodad_entity)
                               .insert(
                                loaded_model.named_scenes["Scene"].clone()
                                 )
                               .remove::<DoodadNeedsModelAttached>()
                                .remove::<RecentlyFailedToLoadModel>()
 
                           
                                  ; 


                         }
                        ,
                       Err(err) =>  {
                       
                        eprintln!("{}",err);


                          commands.entity(new_doodad_entity)
                               .insert(
                                RecentlyFailedToLoadModel {
                                    created_at: time.elapsed() 
                                }
                             ) ;
                               

                        /*
                         commands
                            .entity(new_doodad_entity)
                           //   .insert(Visibility::Hidden)  
                            .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
                            .insert(materials.add(MISSING_MODEL_CUBE_COLOR ) );
                            */

                       }

                 };*/
                

               
            }
            RenderableType::CubeShape(cube_shape_def) => {

                let cube_def_color: Color = cube_shape_def.color.clone().into();


                let spawned_entity = commands
                    .entity(new_doodad_entity)
                    .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
                     .remove::<DoodadNeedsModelAttached>()
                    .insert(materials.add( cube_def_color  )).id();


                if cube_shape_def.wireframe {

                    commands.entity(spawned_entity).insert(Wireframe); 
                }
            }

            RenderableType::MagicFx(magic_fx_name) => {

                let Some(magic_fx) = built_vfx_registry.magic_fx_variants.get(&magic_fx_name ) else {

                    info!("could not spawn magic fx  {:?}",magic_fx_name);
               
               /*     commands
                    .entity(new_doodad_entity)
                    .insert(meshes.add(Cuboid::new(2.0, 2.0, 2.0))) 
                     .remove::<DoodadNeedsModelAttached>()
                     .insert(materials.add(MISSING_MODEL_CUBE_COLOR ) );*/

                     
                          commands.entity(new_doodad_entity)
                               .insert(
                                RecentlyFailedToLoadModel {
                                    created_at: time.elapsed() 
                                }
                             ) ;
                               


                    continue 
                };

                 info!("spawn magic fx  {:?}", magic_fx_name.clone());
                commands
                    .entity(new_doodad_entity)
                     .insert(MagicFxVariantComponent {
                            magic_fx: magic_fx.clone(),
                            start_time: time.elapsed(),
                        })
                      .remove::<DoodadNeedsModelAttached>()
                    //.insert(materials.add(cube_shape_def.color.clone())
                     ;
            }


            RenderableType::LiquidPlane (liquid_type) => {

                 commands
                    .entity(new_doodad_entity)
                     .insert(LiquidPlaneComponent { 
                        liquid_type: liquid_type.clone()
                       })
                      .remove::<DoodadNeedsModelAttached>()
                      ;

            }
        };
    }
}


 
fn remove_recently_failed_to_load(
    mut commands: Commands,
     doodad_query: Query<
         (Entity, &RecentlyFailedToLoadModel) ,
        ( 
             With<DoodadComponent>,
            With<RecentlyFailedToLoadModel>,
        ),
    >,

 
    time: Res<Time>, 
) {
    
    for (doodad_entity,recently_failed_to_load_comp) in doodad_query.iter(){

        let failed_at = &recently_failed_to_load_comp.created_at;

        if time.elapsed()  >  *failed_at +   Duration::from_secs_f32(1.0) {

            if let Some(mut cmd) = commands.get_entity(doodad_entity){ 
                cmd.remove::<RecentlyFailedToLoadModel>();
            }

        } 

    }
}

/*
fn get_loaded_model_from_name<'a>(
    model_name:String,

   
  //  gltf_assets: &Res< GltfAssets>,
     models: &'a Res<'_, Assets<bevy::gltf::Gltf>>,

     ) -> Result< &'a Gltf >{

    let model_handle = gltf_assets
                    .doodad_models
                    .get(model_name.as_str())
                    .context(format!(" no doodad model registered at {:?}", model_name))?;

      let loaded_model = models
                    .get(model_handle)
                    .context(format!("Could not load model handle for {}", model_name))?;


         Ok(loaded_model)
}*/

 

 
pub(crate) fn add_doodad_collider_markers(
    mut commands: Commands,
    doodad_query: Query<
        (Entity, &DoodadComponent),
         (
          
            Without<DoodadColliderMarker>,
            //With<Handle<Mesh>>,
        ),
    > ,

    parent_query: Query< &Parent > , 
   mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>

   
)   {
  
    for evt in scene_instance_evt_reader.read(){

          let parent = evt.parent;

          if let Some((new_doodad_entity, _doodad_component)) = doodad_query.get(parent).ok() {

             
            commands
            .entity(new_doodad_entity)
            
            .insert(DoodadColliderMarker::default())

             ;
             continue;
        } 



         for parent_entity in AncestorIter::new(&parent_query, parent) {


            if let Some((new_doodad_entity, _doodad_component)) = doodad_query.get(parent_entity).ok() {

                 
                commands
                .entity(new_doodad_entity)
                
                .insert(DoodadColliderMarker::default())

                 ;
                 continue

            } 

         }



      

        //let grand_parent_option = parent_query.get( parent  ).ok();
        
        

    }


   
 }



 
pub(crate) fn hide_doodad_collision_volumes(
    mut commands: Commands,
    doodad_query: Query<(Entity, &DoodadComponent, &Children), (Added<DoodadColliderMarker> ) >,

    name_query: Query<&Name>,
    children_query: Query<&Children>,
    transform_query: Query<&Transform>,
)   {
    // Assume you have an `added_doodad_query` that includes entities representing the root of loaded GLTF models.
    for (new_doodad_entity, doodad_component, children) in doodad_query.iter() {
        // Traverse the hierarchy to find the `collision_volumes` node.
        // `children` is a component that contains the direct children of the current entity.
        for child in children.iter() {
            if let Ok((collision_volumes_root_entity, _name)) = find_node_by_name_recursive(
                //&mut commands,
                &name_query,
                &children_query,
                *child,
                "collision_volumes",
                false
            ) {
                // If you want to make the node invisible instead of removing it:
                commands
                    .entity(collision_volumes_root_entity)
                    .insert(Visibility::Hidden);

               /* println!(
                    "found collision volumes root entity for {:?} -- hiding them ",
                    &doodad_component
                );*/



                // If you want to remove the node altogether:
                // commands.entity(entity).despawn_recursive();
            } 
        }

        commands
            .entity(new_doodad_entity)
            .insert(Visibility::Inherited);
    }

     
}

// Recursive function to find a node by name in the scene graph.
pub fn find_node_by_name_recursive(
    //commands: &mut Commands,

    name_query: &Query<&Name>,
    children_query: &Query<&Children>,

    current_entity: Entity,
    target_name: &str,
    log_output : bool 
) -> Result<(Entity, String), &'static str> {
    if let Ok(name) = name_query.get(current_entity) {

        if log_output {
                   info!("find node {:?}",name);
        }
   
        if name.as_str() == target_name {
            return Ok((current_entity, name.to_string()));
        }
    }

    if let Ok(children) = children_query.get(current_entity) {
        for child in children.iter() {
            if let Ok(result) = find_node_by_name_recursive(
               // commands,
                &name_query,
                &children_query,
                *child,
                target_name,
                log_output
            ) {
                return Ok(result);
            }
        }
    }

    Err("Node not found")
}




pub fn update_doodad_placement_preview_model (
  

    mut commands: Commands,

   // doodad_tool_resource: Res<DoodadToolState>,
 


    doodad_definitions_resource: Res < DoodadDefinitionsResource > ,
   // doodad_manifest_assets: Res<Assets<DoodadManifest>>,
 

      asset_server: Res<AssetServer>,


     //this is happening too often !! 
      doodad_placement_component_query: Query<(Entity,&DoodadPlacementComponent), Changed<DoodadPlacementComponent>>

) {
    //we can tell if we are clicking in viewport
  

   
    // ------- compute our rotation and scale from placement properties
    
   

   // let selected_doodad_definition = &doodad_tool_resource.selected;
 

    let Some((placement_preview_entity, doodad_placement_comp)) = doodad_placement_component_query.get_single().ok() else {return};
    
         commands.entity(placement_preview_entity).despawn_descendants() ;

              let Some(doodad_name) =  &doodad_placement_comp.preview_doodad_name else {return};

 

                   let Some(doodad_definition) = doodad_definitions_resource.get_doodad_definition_by_name(doodad_name) else {
                        println!("WARN: Could not spawn doodad {:?}", doodad_name);
                        return;
                    };
               
            
            


           match (&doodad_definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

                  let doodad_name_stem = format!("{}#Scene0", model_name);

                 let model_handle = asset_server.load(doodad_name_stem);


                  let gltf_scene = commands.spawn(
                                SceneBundle {
                                    scene: model_handle,
                                    ..Default::default()
                                } )
                                          
                             .insert(GhostlyMaterialMarker {})
                             .id();


                            commands 
                              .entity(placement_preview_entity)
                               .add_child(
                                gltf_scene
                                 )
                           //    .insert( Wireframe )
                              
                           
                                  ; 






              /* match get_loaded_model_from_name(model_name, &gltf_assets, &models){

                        Ok(loaded_model)=> {

                            info!("spawn preview placement model ");

                             let gltf_scene = commands.spawn(SpatialBundle::default())
                             .insert(  loaded_model.named_scenes["Scene"].clone() )
                             .insert(GhostlyMaterialMarker {})
                             .id();


                            commands 
                              .entity(placement_preview_entity)
                               .add_child(
                                gltf_scene
                                 )
                           //    .insert( Wireframe )
                              
                           
                                  ; 


                         }
                        ,
                       Err(_err) =>  {
                       
                        


                       }

                 };
                */

              
            },
 

           _ =>  {

            warn!("no preview for this model type");
           }
        }
}


/*

fn get_loaded_model_from_name<'a>(
    model_name:String,

   
    gltf_assets: &Res<GltfAssets>,
     models: &'a Res<'_, Assets<bevy::gltf::Gltf>>,

     ) -> Result< &'a Gltf >{

    let model_handle = gltf_assets
                    .gltf_models
                    .get(model_name.as_str())
                    .context(format!(" no doodad model registered at "))?;

      let loaded_model = models
                    .get(model_handle)
                    .context(format!("Could not load model handle for {}", model_name))?;


         Ok(loaded_model)
}
*/




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



         if let Some(clay_tile_block) = &evt.clay_tile_block_data {
             commands
                .entity(doodad_spawned)
                .insert(    clay_tile_block.clone()   );

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









pub fn handle_rebuild_doodads(

    mut commands:Commands,

    doodad_query: Query<Entity, With<RebuildDoodad>>
){

    for doodad_entity in doodad_query.iter(){

        commands.entity(doodad_entity).remove::<RebuildDoodad>();


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
          //  warn!("WARN: Could not replace doodad proto {:?}", doodad_name);
 
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

    placement_tools_state: Res<PlacementToolsState>,

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
 //   let placement_tools_state = editor.window_state::<PlacementWindow>().unwrap();

    let using_random_yaw = placement_tools_state.randomize_yaw;
    let random_scale_multiplier = placement_tools_state.random_scale_multiplier;

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
                zone: None,
                clay_tile_block_data : None ,
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


