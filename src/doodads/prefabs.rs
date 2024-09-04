
 
use spirit_edit_core::placement::PlacementToolsState;
use crate::doodads::doodad_placement::RequestPlaceDoodad;
use spirit_edit_core::prefabs::PrefabToolState;
use spirit_edit_core::prefabs::PrefabToolEvent;
use spirit_edit_core::placement::PlacementResource;
use bevy::prelude::*;
use spirit_edit_core::prefabs::{PrefabComponent, SpawnPrefabEvent};
use spirit_edit_core::doodads::DoodadToolState;



pub fn prefabs_plugin(app: &mut App){

	app 

    .add_event::<PrefabToolEvent>( )

	  .add_systems(Update, (

	             handle_prefab_tool_events,

                handle_place_prefabs, 
                handle_spawn_prefab_events,

                reset_place_prefabs 
               


              //  add_wireframe_to_children

                ) .chain()  )

	;

}

fn handle_spawn_prefab_events(

	mut commands: Commands,

    mut evt_reader: EventReader<SpawnPrefabEvent>,

      placement_resource: Res<PlacementResource>,


){

	for evt in evt_reader.read(){


		let position = &evt.position;
        let prefab_name = &evt.prefab_name;


 

        let mut transform = Transform::from_xyz(position.x, position.y, position.z);

        if let Some(rot) = evt.rotation_euler {
            transform =
                transform.with_rotation(Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z))
        }
        

        let prefab_spawned = commands
            .spawn(SpatialBundle {
                transform : transform.clone(),
                ..default()
            })
            .insert(Name::new( prefab_name.clone() )  )
            .insert( PrefabComponent )
            .id();


             let mut parent = None ;

         if let Some(zone_override) = &evt.zone {
            parent = Some(zone_override);
         } else if let Some(primary_zone) = &placement_resource.placement_parent {
            parent = Some(primary_zone);
         }

         if let Some(parent) = parent {
            commands.entity( prefab_spawned ).set_parent( * parent );
         }




	}


}






pub fn handle_place_prefabs(
   mut event_reader: EventReader<RequestPlaceDoodad>,

    mut event_writer: EventWriter<SpawnPrefabEvent>,

    prefab_tool_resource: Res<PrefabToolState>,

      doodad_tool_resource: Res<DoodadToolState>,

    placement_tools_state: Res<PlacementToolsState>,

      
      
) {

    for evt in event_reader.read(){


 

    // ------- compute our rotation and scale from placement properties
 //   let placement_tools_state = editor.window_state::<PlacementWindow>().unwrap();

    let using_random_yaw = placement_tools_state.randomize_yaw;
    let random_scale_multiplier = placement_tools_state.random_scale_multiplier;

    let mut rng = rand::thread_rng();

    let rotation_euler: Option<Vec3> =  None;

    let scale: Option<Vec3> =  None;
    // -------------------------



    let selected_doodad_definition = &doodad_tool_resource.selected;

    if selected_doodad_definition.is_some() {
        return;
    };



    let selected_prefab_definition = &prefab_tool_resource.selected;

    let Some(prefab_definition_name) = selected_prefab_definition.clone() else {
        return;
    };

    
 

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let place_at_coordinates = & evt.position;

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

            

            event_writer.send(SpawnPrefabEvent {
                position: *place_at_coordinates,
                prefab_name: prefab_definition_name,
                rotation_euler,
             //   scale,
             //   custom_props,
                zone: None,
                //clay_tile_block_data : None ,
      

            });

        
    }
}




pub fn handle_prefab_tool_events(
    mut event_reader: EventReader<PrefabToolEvent>,

    mut prefab_tool_resource: ResMut<PrefabToolState>
) {
    for evt in event_reader.read(){

        match evt {
            PrefabToolEvent::SetSelectedPrefab(prefab_name) => {


                prefab_tool_resource.selected = prefab_name.clone();
            }
        }



    }
}



pub fn reset_place_prefabs(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    mut prefab_tool_resource: ResMut<PrefabToolState>,
     
) {
    

    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }

    prefab_tool_resource.selected = None;

    
}