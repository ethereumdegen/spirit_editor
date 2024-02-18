use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::{TerrainMeshPlugin, terrain::{  TerrainData, TerrainViewer}, edit::{EditTerrainEvent, TerrainCommandEvent}};
 use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::edit::EditingTool;
use crate::ui::{EditorToolsState,ToolMode};



use bevy_mod_raycast::prelude::*;


impl From<EditorToolsState> for EditingTool {
  
  
  
    fn from(state: EditorToolsState) -> Self {
        match state.tool_mode {
            ToolMode::Height => EditingTool::SetHeightMap{
                height: state.color.r,
                radius: state.brush_radius as f32  
            },
            ToolMode::Splat => EditingTool::SetSplatMap{
                r: state.color.r as u8,
                g: state.color.g as u8,
                b: state.color.b as u8,
                radius: state.brush_radius as f32  
            } 
        }
    }

}


pub fn update_brush_paint( 
    mouse_input:  Res< Input<MouseButton> > , //detect mouse click 
        
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast,    
      
    mut edit_event_writer: EventWriter<EditTerrainEvent>,
     mut command_event_writer: EventWriter<TerrainCommandEvent>,

     editor_tools_state: Res<EditorToolsState>
){
     
     
     if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    
    //if tool is paintbrush ... (conditional check)
     
     //make me dynamic or whatever 
  // let tool = EditingTool::SetHeightMap(125,25.0, false);


  let tool = (*editor_tools_state).clone().into();




    
    // let tool = EditingTool::SetSplatMap(5,1,0,25.0,false);
    
   
    if let Some(cursor_ray) = **cursor_ray {
       
      
      
      
        if let Some((intersection_entity,intersection_data)) = raycast.cast_ray(cursor_ray, &default() ).first(){
            
                       
            let hit_point = intersection_data.position();
                         
             
             //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec2::new(hit_point.x, hit_point.z);
            
            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there 
          
           edit_event_writer.send(EditTerrainEvent {
                entity: intersection_entity.clone(), 
                tool, 
                coordinates:hit_coordinates
            });            
             
          
            
        } 
        
    }
    
     
    
}
 