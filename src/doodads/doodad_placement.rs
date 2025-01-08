

use bevy::picking::backend::ray::RayMap;
use rand::Rng;
use bevy::prelude::*;


use crate::doodads::doodad_placement_preview::DoodadPlacementComponent;
use bevy_editor_pls_core::Editor;
use bevy_egui::EguiContexts;
 
 

 
 
 


 pub fn doodad_placement_plugin(  app: &mut App ){


    app
            .add_event::<RequestPlaceDoodad>()
            .add_systems(Update, (
               
                update_doodad_placement_raycast, 
                 

                ) .chain()  );


 }


#[derive(Event,Clone)]
pub struct RequestPlaceDoodad {

 pub 	position: Vec3 

}




pub fn update_doodad_placement_raycast(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    ray_map: Res<RayMap>,
    mut raycast: MeshRayCast,

    mut event_writer: EventWriter<RequestPlaceDoodad>,

  //  doodad_tool_resource: Res<DoodadToolState>,

     


  //  placement_tools_state: Res<PlacementToolsState>,

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

    let raycast_settings = RayCastSettings {
        filter: &raycast_filter,
        ..default()
    };



     for (_, cursor_ray) in ray_map.iter() {
        if let Some((_intersection_entity, intersection_data)) =
            raycast.cast_ray(*cursor_ray, &raycast_settings).first()
        {
            let hit_point = intersection_data.point;

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec3::new(hit_point.x, hit_point.y, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

        //    let custom_props = None; 

           /* event_writer.send(PlaceDoodadEvent {
                position: hit_coordinates,
                doodad_name: doodad_definition_name,
                rotation_euler,
                scale,
                custom_props,
                zone: None,
                //clay_tile_block_data : None ,
            });*/


            event_writer.send(RequestPlaceDoodad { position: hit_coordinates  }) ;
        }
    }
}


