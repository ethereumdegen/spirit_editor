 
//use bevy_material_wizard::material_replacements::MaterialReplacementApplySetWhenSceneReadyComponent;
//use bevy_material_wizard::material_replacements::MaterialReplacementWhenSceneReadyComponent;


use spirit_edit_core::zones::zone_file::CustomPropsMap;
use crate::decals::DecalComponent;
use crate::doodads::doodad_placement::RequestPlaceDoodad;
use crate::render::{CascadedNotShadowCaster, CascadedNotShadowReceiver};
use spirit_edit_core::prefabs::PrefabToolState;
use bevy_clay_tiles::clay_tile_block;


use spirit_edit_core::gltf_models::AddGltfModelComponent; 


use spirit_edit_core::doodads::doodad::RebuildDoodad;


use bevy_material_wizard::material_replacements::MaterialReplacementApplySetWhenSceneReadyComponent;

use bevy_material_wizard::material_overrides::{
    MaterialOverrideComponent,MaterialOverrideWhenSceneReadyComponent,RefreshMaterialOverride

};
use spirit_edit_core::doodads::PlaceClayTileEvent;
use crate::doodads::doodad_placement_preview::DoodadPlacementComponent;
use crate::doodads::doodad_placement_preview::GhostlyMaterialMarker;
use bevy_editor_pls_core::Editor;
use spirit_edit_core::doodads::DoodadToolState;
use spirit_edit_core::placement::PlacementToolsState;
use bevy_egui::EguiContexts;
 


use bevy::utils::HashSet; 
use spirit_edit_core::doodads::DoodadToolEvent;
use spirit_edit_core::placement::PlacementResource;
use spirit_edit_core::doodads::DoodadProto;
use spirit_edit_core::zones::zone_file::CustomPropsComponent;
use spirit_edit_core::doodads::PlaceDoodadEvent;
use bevy_editor_pls_core::EditorEvent;
use spirit_edit_core::doodads::doodad::DoodadComponent;
use spirit_edit_core::doodads::doodad_manifest::RenderableType;
use spirit_edit_core::doodads::DoodadNeedsModelAttached;
use spirit_edit_core::doodads::doodad_manifest::DoodadDefinitionsResource; 
use crate::AssetLoadState;

 
 
 
use crate::asset_loading::BuiltVfxHandleRegistry;
use bevy::utils::Duration;
 
use bevy::pbr::wireframe::WireframeColor;
use bevy::{pbr::wireframe::Wireframe, prelude::*, utils::HashMap};


 
 
 use rand::Rng;


//use anyhow::{Context, Result};

 
 
 use bevy_magic_fx::magic_fx::MagicFxVariantComponent;

use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    scene::SceneInstanceReady,
};

use crate::{ 
    liquid::LiquidPlaneComponent};


 pub fn doodad_plugin(  app: &mut App ){


    app
            .init_resource::<DoodadGltfLoadTrackingResource>()
            .add_event::<SpawnDoodadEvent>()
            .add_observer(   add_doodad_collider_markers )
            .add_systems(Update, (
                attach_models_to_doodads.run_if(in_state(AssetLoadState::Complete)), 

                decrement_doodad_gltf_load_tracker, 
               
              
                hide_doodad_collision_volumes,

                remove_recently_failed_to_load,

                update_doodad_placement_preview_model.run_if(in_state(AssetLoadState::Complete)),

                handle_place_doodad_events,
                handle_spawn_doodad_events,
                handle_place_clay_tile_block_events,
                update_place_doodads,
                reset_place_doodads,
                handle_doodad_tool_events,
                replace_proto_doodads_with_doodads,

               // handle_doodad_scene_ready




              //  add_wireframe_to_children

                ) .chain()  );


 }


 const DOODAD_MAX_LOAD_DISTANCE:f32 = 1000.0;
 

#[derive(Resource,Default)]
pub struct DoodadGltfLoadTrackingResource {

    pub doodad_scenes_loading:  HashSet<AssetId<Gltf>  >
}
impl DoodadGltfLoadTrackingResource{

    pub fn is_overloaded (&self) -> bool {
        return self.doodad_scenes_loading.len() >= 10 

    }
}


#[derive(Component, Default)]
pub struct RecentlyFailedToLoadModel {

    created_at : Duration 

}



#[derive(Component, Default)]
pub struct DoodadColliderMarker {}

const MISSING_MODEL_CUBE_COLOR:Color = Color::rgb(0.9, 0.4, 0.9) ;

 



#[derive(Event)]
pub struct SpawnDoodadEvent {
    pub position: Vec3,
    pub scale: Option<Vec3>,
    pub rotation_euler: Option<Vec3>,
    pub doodad_name: String,
    pub custom_props: Option<CustomPropsMap>,
    pub force_parent: Option<Entity> ,
    pub auto_select: bool,

    pub is_foliage: bool // hack for now 
}




fn attach_models_to_doodads(
    mut commands: Commands,
    mut doodad_load_tracking_resource: ResMut<DoodadGltfLoadTrackingResource>,

    added_doodad_query: Query<
        (Entity,   &DoodadComponent),
        (
            With<DoodadNeedsModelAttached>,
            With<GlobalTransform>,
            Without<Mesh3d>,
            Without<RecentlyFailedToLoadModel>,
        ),
    >,

   // models: Res<Assets<Gltf>>,
  //  gltf_assets: Res<GltfAssets>,

   asset_server: Res<AssetServer>,

   global_xform_query: Query<&GlobalTransform>,
   camera_query: Query<Entity, With<Camera3d> >, 

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    custom_props_query: Query< &CustomPropsComponent  >,


      built_vfx_registry: Res<BuiltVfxHandleRegistry>,
    time: Res<Time>, 
) {

    let Some(camera_entity) = camera_query.get_single().ok() else {return};

    let Some(camera_xform)  = global_xform_query.get(camera_entity).ok() else {return};
    
    for (new_doodad_entity,  doodad_component) in added_doodad_query.iter() {
     //   let doodad_name = &name_comp.to_string();

      //  let doodad_name_clone = doodad_name.clone();
      //  let name_comp = Name::new(doodad_name_clone);





        let Some(doodad_xform)  = global_xform_query.get(new_doodad_entity).ok() else {continue};

            //THIS HELPS PREVENT CRASHES BY THROTTLING DOODAD LOAD 
         let doodad_cam_distance =  doodad_xform.translation().distance( camera_xform.translation() ) ;
         if doodad_cam_distance > DOODAD_MAX_LOAD_DISTANCE {continue};      



      if let Some(mut cmd ) = commands.get_entity( new_doodad_entity  ) {
 
          cmd.try_insert( PickingBehavior::default() ) ;  //optional
      } 


       let custom_props_comp = custom_props_query.get(new_doodad_entity).ok();
        let material_override_from_props = custom_props_comp.map(|c| c.props.get("material_override")).flatten();


        let mut material_override = doodad_component.definition.material_override.clone(); 
        let material_replacement_set = &doodad_component.definition.material_replacement_set; 


        if let Some(material_override_from_props) = material_override_from_props {

            material_override =  Some(material_override_from_props.to_string() );
        }
         

        //handle attaching renderable components based on the renderable type - this lets us see the doodad in the editor
        match (&doodad_component.definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

                if doodad_load_tracking_resource.is_overloaded() {
                    continue
                }

                //let doodad_name_stem = format!("{}#Scene0", model_name);
                 let doodad_name_stem = format!("../artifacts/game_assets/{}", model_name);


                 let model_handle:Handle<Gltf> = asset_server.load(doodad_name_stem);

                     doodad_load_tracking_resource.doodad_scenes_loading.insert( model_handle.id() );

                    if let Some(mut cmd ) = commands.get_entity( new_doodad_entity  ) {
                        
                        
                          let scene = cmd.commands()
                                .spawn( 
                                   ( 
                                     Transform::default(),  
                                     Visibility::default(),

                                   //  CascadedNotShadowCaster,
                                     CascadedNotShadowReceiver,

                                     AddGltfModelComponent( model_handle ) 


                                     ),
                                     
                                   )
                                
                                
                                .id();


                            cmd .remove::<DoodadNeedsModelAttached>()
                                .remove::<RecentlyFailedToLoadModel>()
  
                               .add_child( scene  );



                          
                    }
      

                      



                    if let Some( material_override  ) = material_override  {
                         //info!("found mat override  {:?}", material_override );


                        if let Some(mut cmd ) = commands.get_entity( new_doodad_entity  ) {
                            cmd.try_insert(
                                MaterialOverrideWhenSceneReadyComponent {
                                    material_override: material_override.clone() 
                                }

                            );
                        }

                    } else  if let Some( material_replacement_set  ) = material_replacement_set  {
                        // info!("found   material_replacements  {:?}", material_replacements );

                       
                         if let Some(mut cmd ) = commands.get_entity( new_doodad_entity  ) {
                            cmd.try_insert(
                                MaterialReplacementApplySetWhenSceneReadyComponent  (   material_replacement_set.clone() )
                                   

                            );
                        } 

                    }


               

               
            }
            RenderableType::CubeShape(cube_shape_def) => {

                let cube_def_color: Color = cube_shape_def.color.clone().into();


                let spawned_entity = commands
                    .entity(new_doodad_entity)
                    .insert(Mesh3d( meshes.add(Cuboid::new(1.0, 1.0, 1.0)) ))
                     .remove::<DoodadNeedsModelAttached>()
                    .insert(MeshMaterial3d( materials.add( cube_def_color  ) )  ).id();


                if cube_shape_def.wireframe {

                    commands.entity(spawned_entity).insert(Wireframe); 
                }
            }

             


             RenderableType::Decal(decal_name) => {
 

                let spawned_entity = commands
                    .entity(new_doodad_entity)

                     .insert(DecalComponent { 
                        decal_name: decal_name.clone()
                       })
                      .remove::<DoodadNeedsModelAttached>()

                        .id(); 


                
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

            RenderableType::NoModel => {

               // nothing at all :D 
            }
        };
    }
}




//may want to do this in the main game in case NOT doing it causes crashes 
fn decrement_doodad_gltf_load_tracker(

    mut asset_events: EventReader<AssetEvent<Gltf>>,

    mut doodad_load_tracking_resource: ResMut<DoodadGltfLoadTrackingResource>,

){

    for evt in asset_events.read() {

        match evt {
             
            AssetEvent::LoadedWithDependencies { id } =>  {


                  let _removed =  doodad_load_tracking_resource.doodad_scenes_loading.remove( id ); 
 

            }

            _ => {}
        }
 
      
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

    scene_instance_evt_trigger: Trigger<SceneInstanceReady>  ,

    mut commands: Commands,
    doodad_query: Query<
        (Entity, &DoodadComponent),
         (
          
            Without<DoodadColliderMarker>,
            //With<Handle<Mesh>>,
        ),
    > ,

    parent_query: Query< &Parent > , 
 //  mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>

   
)   {
  
   let trig_entity = scene_instance_evt_trigger.entity();

    let Some(parent_entity) = parent_query.get(trig_entity).ok().map( |p| p.get() ) else {return};


          if let Some((new_doodad_entity, _doodad_component)) = doodad_query.get(parent_entity).ok() {

             
            commands
            .entity(new_doodad_entity)
            
            .insert(DoodadColliderMarker::default())

             ;
             return;
        } 



         for parent_entity in AncestorIter::new(&parent_query, parent_entity) {


            if let Some((new_doodad_entity, _doodad_component)) = doodad_query.get(parent_entity).ok() {

                 
                commands
                .entity(new_doodad_entity)
                
                .insert(DoodadColliderMarker::default())

                 ;
                 return

            } 

         }



      

        //let grand_parent_option = parent_query.get( parent  ).ok();
        
        

     


   
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

                  let doodad_name_stem = format!("../artifacts/game_assets/{}#Scene0", model_name);

                 let model_handle = asset_server.load(doodad_name_stem);


                  let gltf_scene = commands.spawn(
                                 SceneRoot( model_handle ))
                                          
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

    placement_resource: Res<PlacementResource>,

    global_xform_query: Query<&GlobalTransform>, 

   // doodad_manifest_resource: Res<DoodadManifestResource>,
   // doodad_manifest_assets: Res<Assets<DoodadManifest>>,
) {
   // let manifest_handle = &doodad_manifest_resource.manifest;


   // let manifest = manifest_handle.as_ref().map( |handle| doodad_manifest_assets.get(handle) ).flatten();

    for evt in evt_reader.read() {
        let position = &evt.position;
        let doodad_name = &evt.doodad_name;


        let auto_select = &evt.auto_select;

        


        // ----
        // determine the doodad parent, if there will be one

        let mut parent = None ;

         if let Some(parent_override) = &evt.force_parent {
            parent = Some(parent_override);
         } else if let Some(primary_parent) = &placement_resource.placement_parent {
            parent = Some(primary_parent);
         }

        // ------


        // if there is a parent, the relative position will be offset by the parent global translation 


        let parent_global_translation = parent.map( |p| global_xform_query.get(*p).ok()  ).flatten().map(|x| x.translation() ) ; 
        let position_relative_to_parent = position - parent_global_translation.unwrap_or_default() ; ;


        // -----





        let mut transform = Transform::from_translation(position_relative_to_parent) ; 

        if let Some(rot) = evt.rotation_euler {
            transform =
                transform.with_rotation(Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z))
        }
        if let Some(scale) = evt.scale {
            transform = transform.with_scale(scale)
        }
 




        let doodad_spawned = commands
            .spawn( transform )
            .insert( Visibility::default() )
            .insert(Name::new(doodad_name.clone())  )
            .insert( DoodadProto )
            .id();


        if *auto_select {

            //why are these different.. ? 
             editor_event_writer.send( 
                EditorEvent::SetSelectedEntities(Some(vec![ doodad_spawned ]))
             );

            doodad_tool_event_writer.send(
                DoodadToolEvent::SetSelectedDoodad(None) 
            );
        }
        

    
        
          
        let proto_custom_props_to_attach = match &evt.custom_props {
            Some(props) => Some( props ),
            None  => None
        };


         if let Some(custom_props) = proto_custom_props_to_attach {
          

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

 
        

         if let Some(parent) = parent {
            commands.entity( doodad_spawned ).set_parent( *parent );
         }
 
    }
}




pub fn handle_spawn_doodad_events(
    mut commands: Commands,

    mut evt_reader: EventReader<SpawnDoodadEvent>,

    mut editor_event_writer: EventWriter<EditorEvent>,
    mut doodad_tool_event_writer: EventWriter<DoodadToolEvent>,

    placement_resource: Res<PlacementResource>,

  //  global_xform_query: Query<&GlobalTransform>, 


){



    for evt in evt_reader.read() {
        let position = &evt.position;
        let doodad_name = &evt.doodad_name;


        let auto_select = &evt.auto_select;

        
        let is_foliage = &evt.is_foliage; 

        // ----
        // determine the doodad parent, if there will be one

        let mut parent = None ;

         if let Some(parent_override) = &evt.force_parent {
            parent = Some(parent_override);
         } else if let Some(primary_parent) = &placement_resource.placement_parent {
            parent = Some(primary_parent);
         }

        // ------


    


        let mut transform = Transform::from_translation(*position) ; 

        if let Some(rot) = evt.rotation_euler {
            transform =
                transform.with_rotation(Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z))
        }
        if let Some(scale) = evt.scale {
            transform = transform.with_scale(scale)
        }









        let doodad_spawned = commands
            .spawn( transform )
            .insert( Visibility::default() )
            .insert(Name::new(doodad_name.clone())  )
            .insert( DoodadProto )
            .id();

 

        if *auto_select {

            //why are these different.. ? 
             editor_event_writer.send( 
                EditorEvent::SetSelectedEntities(Some(vec![ doodad_spawned ]))
             );

            doodad_tool_event_writer.send(
                DoodadToolEvent::SetSelectedDoodad(None) 
            );
        }
        

    //    println!("doodad spawned {:?}", doodad_spawned);

        
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



        /* if let Some(clay_tile_block) = &evt.clay_tile_block_data {
             commands
                .entity(doodad_spawned)
                .insert(    clay_tile_block.clone()   );

        }*/

        

         if let Some(parent) = parent {
            commands.entity( doodad_spawned ).set_parent( *parent );
         }

         /*
         if let Some(zone_override) = &evt.zone {
            if let Some(mut ent) = commands.get_entity(zone_override.clone()) {
                ent.add_child(doodad_spawned);
            }
        }else  if let Some(primary_zone) = &zone_resource.primary_zone {
            if let Some(mut ent) = commands.get_entity(primary_zone.clone()) {
                ent.add_child(doodad_spawned);
            }
        }*/
    }



}





pub fn handle_place_clay_tile_block_events(
    mut commands: Commands,

    mut evt_reader: EventReader<PlaceClayTileEvent>,

    mut editor_event_writer: EventWriter<EditorEvent>,
    mut doodad_tool_event_writer: EventWriter<DoodadToolEvent>,

    placement_resource: Res<PlacementResource>,

   // doodad_manifest_resource: Res<DoodadManifestResource>,
   // doodad_manifest_assets: Res<Assets<DoodadManifest>>,
) {
   // let manifest_handle = &doodad_manifest_resource.manifest;


   // let manifest = manifest_handle.as_ref().map( |handle| doodad_manifest_assets.get(handle) ).flatten();

    for evt in evt_reader.read() {
        let position = &evt.position;
       // let doodad_name = &evt.doodad_name;


 

        let mut transform = Transform::from_xyz(position.x, position.y, position.z);

        if let Some(rot) = evt.rotation_euler {
            transform =
                transform.with_rotation(Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z))
        }
        if let Some(scale) = evt.scale {
            transform = transform.with_scale(scale)
        }

        let doodad_spawned = commands
            .spawn(transform)
            .insert(Visibility::default())
            .insert(Name::new( "ClayTileBlock" )  )
            .insert( DoodadProto )
            .id();


        editor_event_writer.send( 
            EditorEvent::SetSelectedEntities(Some(vec![ doodad_spawned ]))
         );

        doodad_tool_event_writer.send(
            DoodadToolEvent::SetSelectedDoodad(None) 
        );

     //    println!("doodad spawned {:?}", doodad_spawned);

        
            //from cloning ! 
       /* let proto_custom_props_to_attach = match &evt.custom_props {
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
        }*/



           let clay_tile_block = &evt.clay_tile_block_data ;
             commands
                .entity(doodad_spawned)
                .insert(    clay_tile_block.clone()   );

        

        let mut parent   = None ;

         if let Some(zone_override) = &evt.zone {
            parent = Some(zone_override);
         } else if let Some(primary_parent) = &placement_resource.placement_parent {
            parent = Some(primary_parent);
         }

         if let Some(parent) = parent {
            commands.entity( doodad_spawned ).set_parent( * parent );
         }

         /*
         if let Some(zone_override) = &evt.zone {
            if let Some(mut ent) = commands.get_entity(zone_override.clone()) {
                ent.add_child(doodad_spawned);
            }
        }else  if let Some(primary_zone) = &zone_resource.primary_zone {
            if let Some(mut ent) = commands.get_entity(primary_zone.clone()) {
                ent.add_child(doodad_spawned);
            }
        }*/
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


    

       //  println!("doodad spawned {:?}", doodad_spawned);

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
   mut event_reader: EventReader<RequestPlaceDoodad>,

    mut event_writer: EventWriter<PlaceDoodadEvent>,

    doodad_tool_resource: Res<DoodadToolState>,

      
    placement_tools_state: Res<PlacementToolsState>,

      
      
) {

    for evt in event_reader.read(){


 

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

    
 

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let place_at_coordinates = & evt.position;

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

            let custom_props = None; 

            event_writer.send(PlaceDoodadEvent {
                position: *place_at_coordinates,
                doodad_name: doodad_definition_name,
                rotation_euler,
                scale,
                custom_props,
                force_parent: None,
                 auto_select: true,
                //clay_tile_block_data : None ,
      

            });

        
    }
}

pub fn reset_place_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    mut doodad_tool_resource: ResMut<DoodadToolState>,
     
) {
    

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


