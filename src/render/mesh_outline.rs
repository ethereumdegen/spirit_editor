//use crate::dev::dev_editor::dev_editor_plugin;

use std::f32::consts::PI;
use crate::doodads::doodad::DoodadMeshMarker;
use spirit_edit_core::doodads::doodad::DoodadComponent;
use bevy::{  scene::SceneInstanceReady};

use bevy::ecs::relationship::DescendantIter;


 use bevy_mesh_outline::MeshOutlinePlugin;

 use bevy_mesh_outline::{MeshOutline};

use bevy::prelude::*;
use bevy::platform::collections::HashSet;
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, SILVER, YELLOW},
} ;

 
pub(crate) fn mesh_outline_plugin(app: &mut App) {
    {
        app

          .add_plugins( MeshOutlinePlugin )
         

         .add_observer (add_outline_for_doodad ) 

         //	.add_systems(Startup, spawn_test_cubes)
        
          ;
          
          
    }
}



fn add_outline_for_doodad(

     trigger: Trigger<SceneInstanceReady>,

    mut commands: Commands,

    scene_ready_query: Query< ( Entity, &DoodadMeshMarker )   >,
 
    children_query: Query<&Children>, //    scene_bundle_link_query: Query<&SceneBundleLink>
) {


    let trigger_entity = trigger.target(); 

    let Ok( (scene_entity, _doodad_comp) ) = scene_ready_query.get( trigger_entity ) else {return;};

   
        for child in DescendantIter::new(&children_query, scene_entity) {
            let _ = commands.get_entity(child).map(|mut cmd| {
                   cmd.insert(

                            MeshOutline::new( 22.0 ).with_priority( 100.0 )
                    );
          });
   }


}

 


 fn spawn_test_cubes(

  mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

      ){


 	    // Yellow cube with red outline, low priority
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::from(YELLOW))),
        Transform::from_xyz(0.0, 1.0, 0.0)
            .with_rotation(Quat::from_rotation_x(PI / 5.0) * Quat::from_rotation_y(PI / 3.0)),
        MeshOutline::new(10.0).with_priority(1.0) ,
       
    ));

    // Blue sphere with green outline, high priority
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(-0.5, 1.0, 0.5),
        
        
    ));

 }