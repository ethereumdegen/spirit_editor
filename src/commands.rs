use bevy::prelude::*;
 
use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::{TerrainMeshPlugin, terrain::{  TerrainData, TerrainViewer}, edit::{EditTerrainEvent, TerrainCommandEvent}};
 use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::edit::EditingTool;


use bevy_mod_raycast::prelude::*;


pub fn update_commands ( 
    key_input:  Res< Input<KeyCode> > , //detect mouse click 
         
      
      mut edit_event_writer: EventWriter<EditTerrainEvent>,
     mut command_event_writer: EventWriter<TerrainCommandEvent>,
){
  
        if key_input.pressed(KeyCode::ControlLeft) || key_input.pressed(KeyCode::ControlRight) {
        if key_input.just_pressed(KeyCode::S) {
            
               println!("saving chunks !");
               
             command_event_writer.send(
                 TerrainCommandEvent::SaveAllChunks(true,true,true)
                 
             )
        }}
     
}
