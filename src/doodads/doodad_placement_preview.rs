


use bevy::ecs::relationship::DescendantIter;
use spirit_edit_core::doodads::DoodadToolState;
use bevy_editor_pls_core::Editor;
use bevy::gltf::Gltf;
use bevy::log::tracing_subscriber::filter::FilterFn;
use bevy::pbr::wireframe::{Wireframe, WireframeColor};
use bevy::prelude::*;


use bevy::scene::SceneInstanceReady;
//use bevy_editor_pls_core::Editor;
use  bevy_egui::EguiContexts;
 
 use bevy::  picking::backend::ray::RayMap; 

  

 
pub fn doodad_placement_plugin(app: &mut App){



          app

             .add_observer( apply_ghostly_material )
            
            .add_systems(Startup, spawn_doodad_placement_component)
            .add_systems(Update, update_doodad_placement_preview_position)
             .add_systems(Update, update_doodad_placement_preview_state)
             // .add_systems(Update, update_doodad_placement_preview_model)
             
            // .add_systems(Update, doodad_placement_preview::update_doodad_placement_preview)


           
            ;

}
 

 

#[derive(Component, Default)]
  struct WireframeMarker {}

#[derive(Component, Default)]
 pub struct GhostlyMaterialMarker {} 
 
#[derive(Component,Default)]
pub struct DoodadPlacementComponent {

	pub preview_doodad_name: Option<String> 

}


pub fn spawn_doodad_placement_component(
	mut commands: Commands
	){

	commands.spawn((Transform::default(),Visibility::default()))
	.insert(DoodadPlacementComponent::default());

}


pub fn update_doodad_placement_preview_position (
  //  mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    ray_map: Res<RayMap>,
    mut raycast: MeshRayCast,

    

 //   doodad_tool_resource: Res<DoodadToolState>,

    mut contexts: EguiContexts,

    editor: Res<Editor>,


      doodad_placement_component_query: Query<Entity, With<DoodadPlacementComponent>>,
    mut doodad_placement_transform_query: Query<&mut Transform, With<DoodadPlacementComponent>>,

      parent_query: Query<&ChildOf >

) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let hovering_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !hovering_viewport {
        return;
    }

    // ------- compute our rotation and scale from placement properties
    
   

 /*
 THIS raycast has to ignore doodads! 
 */		

 			//let raycast_filter = [];

 	let raycast_filter = |entity: Entity| {


         let mut current_entity = entity;
        loop {
            if doodad_placement_component_query.get(current_entity).is_ok() {
                return false;
            }
            match parent_query.get(current_entity).ok() {
                Some(parent) => current_entity = parent.parent(),
                None => break,
            }
        }
        true
    };

    let raycast_settings = MeshRayCastSettings {
        filter: &raycast_filter,
        ..default()
    };

     for (_, cursor_ray) in ray_map.iter() {
        if let Some((_intersection_entity, intersection_data)) =
            raycast.cast_ray(*cursor_ray, &raycast_settings

            	
            	).first()
        {
            let hit_point = intersection_data.point;

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec3::new(hit_point.x, hit_point.y, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

           // let custom_props = None; 

         
         	if let Some( mut xform ) = doodad_placement_transform_query.single_mut().ok(){


         		xform.translation =  hit_coordinates.clone() ;

         	}
        }
    }


}



pub fn update_doodad_placement_preview_state (
  

    doodad_tool_resource: Res<DoodadToolState>,

    mut contexts: EguiContexts,

    editor: Res<Editor>,


    mut doodad_placement_component_query: Query<(Entity, &mut DoodadPlacementComponent), With<DoodadPlacementComponent>>

) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let hovering_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !hovering_viewport {
        return;
    }

    // ------- compute our rotation and scale from placement properties
    
   

    let selected_doodad_definition = &doodad_tool_resource.selected;
 

  	

   	if let Some( (_placement_preview_entity, mut doodad_placement_comp) ) = doodad_placement_component_query.get_single_mut().ok(){

   		match selected_doodad_definition.clone() {
   			 Some(doodad_definition_name) => {


   			 	if doodad_placement_comp.preview_doodad_name != Some(doodad_definition_name.clone()) {
   			 		 doodad_placement_comp.preview_doodad_name  = Some(doodad_definition_name.clone());
   			 	}

   			 	
   			 }
   			 None => {

   			 	 doodad_placement_comp.preview_doodad_name  = None;
   			 }

   		}

   	 
         		//xform.translation =  hit_coordinates.clone() ;

        }

}





fn add_wireframe_to_children( 
    scene_instance_evt_trigger: Trigger<SceneInstanceReady>  ,

        mut commands: Commands ,

       doodad_query: Query<   (Entity,  &WireframeMarker) >,
         children_query: Query<&Children>,           
  
         parent_query: Query<&ChildOf >

    ) {

    let trig_entity = scene_instance_evt_trigger.target();
    let Some(parent_entity) = parent_query.get(trig_entity).ok().map( |p| p.parent() ) else {return};

  
        if let Some((new_doodad_entity,wireframe_marker)) = doodad_query.get(parent_entity).ok() {
         
          for child_entity in DescendantIter::new(&children_query, new_doodad_entity) { 
 
           
                commands.entity( child_entity ) 
                        .try_insert(Wireframe)
                        .try_insert(WireframeColor { color: Color::WHITE  } )
                        ;

                    }
             
        }
 


}





fn apply_ghostly_material( 

    scene_instance_evt_trigger: Trigger<SceneInstanceReady>  ,

        mut commands: Commands ,

       doodad_query: Query<   (Entity,  &GhostlyMaterialMarker), With<GhostlyMaterialMarker> >,
       children_query: Query<&Children>,           
  
    //  mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,


      	standard_material_query: Query<&MeshMaterial3d<StandardMaterial>>,

     mut   standard_material_assets : ResMut<Assets<StandardMaterial>>,

           parent_query: Query<&ChildOf >


    ) {

       let trig_entity = scene_instance_evt_trigger.target();
        let Some(parent_entity) = parent_query.get(trig_entity).ok().map( |p| p.parent() ) else {return};

        
        if let Some((new_doodad_entity,_marker )) = doodad_query.get(parent_entity).ok() {
         
          for child_entity in DescendantIter::new(&children_query, new_doodad_entity) { 
 
           	let Some(material_handle) = standard_material_query.get(child_entity).ok() else {continue};
               let Some(  mat) = standard_material_assets.get(material_handle) else {continue}; 

               let mut  new_mat = mat.clone();

               new_mat.alpha_mode = AlphaMode::Blend;
               new_mat.base_color = mat.base_color.clone().with_alpha( 0.25 );

                let new_material_handle = standard_material_assets.add(new_mat);
               commands.entity(child_entity).insert(  MeshMaterial3d( new_material_handle  ) );

                   
           	}
             
        }

    

       



}