use bevy::pbr::wireframe::WireframeColor;
use bevy::{pbr::wireframe::Wireframe, prelude::*, utils::HashMap};


use bevy_mod_sysfail::*;
use bevy_editor_pls_default_windows::doodads::{doodad::
{DoodadComponent, LoadedGltfAssets}, doodad_manifest::RenderableType};
 

use anyhow::{Context, Result};

use bevy_mod_sysfail::*;

use bevy_mod_picking::prelude::*;
 use bevy_magic_fx::magic_fx::MagicFxVariantComponent;

use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    scene::SceneInstanceReady,
};

use crate::{asset_loading::BuiltVfxResource, 
    liquid::LiquidPlaneComponent};




#[derive(Default)]
pub(crate) struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedGltfAssets::default())
            .add_systems(Update, (attach_models_to_doodads, 
                add_doodad_collider_markers, 
                hide_doodad_collision_volumes,
              //  add_wireframe_to_children

                ));
    }
}




#[derive(Component, Default)]
pub struct DoodadColliderMarker {}

const MISSING_MODEL_CUBE_COLOR:Color = Color::rgb(0.9, 0.4, 0.9) ;

#[sysfail]
fn attach_models_to_doodads(
    mut commands: Commands,
    added_doodad_query: Query<
        (Entity, &DoodadComponent),
        (
            Added<DoodadComponent>,
            With<GlobalTransform>,
            Without<Handle<Mesh>>,
        ),
    >,

    models: Res<Assets<Gltf>>,
    gltf_assets: Res<LoadedGltfAssets>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut built_vfx_registry: ResMut<BuiltVfxResource>,
    time: Res<Time>, 
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("add_model_to_doodads").entered();

    for (new_doodad_entity, doodad_component) in added_doodad_query.iter() {
        let doodad_name = &doodad_component.definition.name;

        let doodad_name_clone = doodad_name.clone();
        let name_comp = Name::new(doodad_name_clone);

        commands
            .entity(new_doodad_entity)
            .insert(name_comp)
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
 
                           
                                  ; 


                         }
                        ,
                       Err(err) =>  {
                       
                        eprintln!("{}",err);
                         commands
                            .entity(new_doodad_entity)
                           //   .insert(Visibility::Hidden)  
                            .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
                            .insert(materials.add(MISSING_MODEL_CUBE_COLOR ) );


                       }

                 };
                

               
            }
            RenderableType::CubeShape(cube_shape_def) => {
                let spawned_entity = commands
                    .entity(new_doodad_entity)
                    .insert(meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
                    .insert(materials.add(cube_shape_def.color.clone())).id();


                if cube_shape_def.wireframe {

                    commands.entity(spawned_entity).insert(Wireframe); 
                }
            }

            RenderableType::MagicFx(magic_fx_name) => {

                let Some(magic_fx) = built_vfx_registry.magic_fx_variants.get(&magic_fx_name ) else {

                    info!("spawn magic fx fallback for {:?}",magic_fx_name);
                    commands
                    .entity(new_doodad_entity)
                    .insert(meshes.add(Cuboid::new(2.0, 2.0, 2.0))) 
                     .insert(materials.add(MISSING_MODEL_CUBE_COLOR ) );


                    continue 
                };

                 info!("spawn magic fx  {:?}", magic_fx_name.clone());
                commands
                    .entity(new_doodad_entity)
                     .insert(MagicFxVariantComponent {
                            magic_fx: magic_fx.clone(),
                            start_time: time.elapsed(),
                        })
                    //.insert(materials.add(cube_shape_def.color.clone())
                     ;
            }


            RenderableType::LiquidPlane (liquid_type) => {

                 commands
                    .entity(new_doodad_entity)
                     .insert(LiquidPlaneComponent { 
                        liquid_type: liquid_type.clone()
                       });

            }
        };
    }
}


fn get_loaded_model_from_name<'a>(
    model_name:String,

   
    gltf_assets: &Res<LoadedGltfAssets>,
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
