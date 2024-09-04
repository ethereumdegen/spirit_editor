
 
use spirit_edit_core::placement::PlacementResource;
use bevy::prelude::*;
use spirit_edit_core::prefabs::{PrefabComponent, SpawnPrefabEvent};



pub fn prefabs_plugin(app: &mut App){

	app 
	  .add_systems(Update, (

	     
                handle_place_prefab_events,
               


              //  add_wireframe_to_children

                ) .chain()  )

	;

}

fn handle_place_prefab_events(

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