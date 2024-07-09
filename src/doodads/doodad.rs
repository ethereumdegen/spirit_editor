use crate::AssetLoadState;
use bevy_editor_pls_default_windows::doodads::doodad_manifest::DoodadManifestResource;
use bevy_editor_pls_default_windows::doodads::doodad_manifest::DoodadManifest;
use bevy_editor_pls_default_windows::doodads::doodad_placement_preview::DoodadPlacementComponent;
use bevy_editor_pls_default_windows::doodads::doodad_placement_preview::GhostlyMaterialMarker;
use crate::asset_loading::BuiltVfxHandleRegistry;
use bevy::utils::Duration;
use bevy_editor_pls_default_windows::doodads::DoodadNeedsModelAttached;
use bevy::pbr::wireframe::WireframeColor;
use bevy::{pbr::wireframe::Wireframe, prelude::*, utils::HashMap};


use bevy_mod_sysfail::*;
use bevy_editor_pls_default_windows::doodads::{doodad::
{DoodadComponent,  }, doodad_manifest::RenderableType};
 

use anyhow::{Context, Result};

use bevy_mod_sysfail::*;

use bevy_mod_picking::prelude::*;
 use bevy_magic_fx::magic_fx::MagicFxVariantComponent;

use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    scene::SceneInstanceReady,
};

use crate::{ 
    liquid::LiquidPlaneComponent};


use crate::asset_loading::GltfAssets;


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

                update_doodad_placement_preview_model.run_if(in_state(AssetLoadState::Complete))
              //  add_wireframe_to_children

                ));
    }
}

#[derive(Component, Default)]
pub struct RecentlyFailedToLoadModel {

    created_at : Duration 

}



#[derive(Component, Default)]
pub struct DoodadColliderMarker {}

const MISSING_MODEL_CUBE_COLOR:Color = Color::rgb(0.9, 0.4, 0.9) ;

#[sysfail]
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

    models: Res<Assets<Gltf>>,
    gltf_assets: Res<GltfAssets>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

      built_vfx_registry: Res<BuiltVfxHandleRegistry>,
    time: Res<Time>, 
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("add_model_to_doodads").entered();

    for (new_doodad_entity,  doodad_component) in added_doodad_query.iter() {
     //   let doodad_name = &name_comp.to_string();

      //  let doodad_name_clone = doodad_name.clone();
      //  let name_comp = Name::new(doodad_name_clone);

        commands
            .entity(new_doodad_entity)
        //    .insert(name_comp)
            .insert(PickableBundle::default()) 

            ;

        //handle attaching renderable components based on the renderable type - this lets us see the doodad in the editor
        match (&doodad_component.definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

               match get_loaded_model_from_name(model_name, &gltf_assets, &models){

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

                 };
                

               
            }
            RenderableType::CubeShape(cube_shape_def) => {
                let spawned_entity = commands
                    .entity(new_doodad_entity)
                    .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
                     .remove::<DoodadNeedsModelAttached>()
                    .insert(materials.add(cube_shape_def.color.clone())).id();


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

fn get_loaded_model_from_name<'a>(
    model_name:String,

   
    gltf_assets: &Res< GltfAssets>,
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
}

 

#[sysfail]
pub(crate) fn add_doodad_collider_markers(
    mut commands: Commands,
    doodad_query: Query<
        (Entity, &DoodadComponent),
         (
          
            Without<DoodadColliderMarker>,
            //With<Handle<Mesh>>,
        ),
    > ,
   mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>

   
)   {
    #[cfg(feature = "tracing")]
    let _span = info_span!("add_doodad_collider_markers").entered();

    for evt in scene_instance_evt_reader.read(){
        let parent = evt.parent;
        
        if let Some((new_doodad_entity, doodad_component)) = doodad_query.get(parent).ok() {

             
            commands
            .entity(new_doodad_entity)
            
            .insert(DoodadColliderMarker::default())

             ;

        }

    }


   
 }





 #[sysfail]
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
                &mut commands,
                &name_query,
                &children_query,
                *child,
                "collision_volumes",
            ) {
                // If you want to make the node invisible instead of removing it:
                commands
                    .entity(collision_volumes_root_entity)
                    .insert(Visibility::Hidden);

                println!(
                    "found collision volumes root entity for {:?} -- hiding them ",
                    &doodad_component
                );



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
fn find_node_by_name_recursive(
    commands: &mut Commands,

    name_query: &Query<&Name>,
    children_query: &Query<&Children>,

    current_entity: Entity,
    target_name: &str,
) -> Result<(Entity, String), &'static str> {
    if let Ok(name) = name_query.get(current_entity) {
   //     info!("find node {:?}",name);
        if name.as_str() == target_name {
            return Ok((current_entity, name.to_string()));
        }
    }

    if let Ok(children) = children_query.get(current_entity) {
        for child in children.iter() {
            if let Ok(result) = find_node_by_name_recursive(
                commands,
                &name_query,
                &children_query,
                *child,
                target_name,
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
 


    doodad_manifest_resource: Res<DoodadManifestResource>,
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,

    gltf_assets: Res<GltfAssets>,
     models:  Res< Assets<bevy::gltf::Gltf>>,


     //this is happening too often !! 
      doodad_placement_component_query: Query<(Entity,&DoodadPlacementComponent), Changed<DoodadPlacementComponent>>

) {
    //we can tell if we are clicking in viewport
  

   
    // ------- compute our rotation and scale from placement properties
    
   

   // let selected_doodad_definition = &doodad_tool_resource.selected;
 

    let Some((placement_preview_entity, doodad_placement_comp)) = doodad_placement_component_query.get_single().ok() else {return};
    
         commands.entity(placement_preview_entity).despawn_descendants() ;

              let Some(doodad_name) =  &doodad_placement_comp.preview_doodad_name else {return};


                  let Some(manifest_handle) = &doodad_manifest_resource.manifest else {
                        println!("WARN: no doodad manifest file found");
                        return;
                    };


                     let Some(manifest) = doodad_manifest_assets.get(manifest_handle) else {
                        println!("WARN: no doodad manifest file found");
                        return;
                    };


                   let Some(doodad_definition) = manifest.get_doodad_definition_by_name(doodad_name) else {
                        println!("WARN: Could not spawn doodad {:?}", doodad_name);
                        return;
                    };
               
            
            


           match (&doodad_definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

               match get_loaded_model_from_name(model_name, &gltf_assets, &models){

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