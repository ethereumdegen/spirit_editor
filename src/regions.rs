use crate::ui::ToolMode;
use bevy_regions::regions::RegionsData;
use crate::EditorToolsState;
use bevy::prelude::*;


pub fn update_regions_plane_visibility (

   mut region_plane_query: Query<&mut Visibility, With<RegionsData>>,
   editor_tools_state: Res<EditorToolsState>



  ){

    let Some( mut region_plane_vis ) = region_plane_query.get_single_mut().ok() else {return};

    *region_plane_vis = match &editor_tools_state.tool_mode {
        
        ToolMode::Regions => Visibility::Visible, 
        _ => Visibility::Hidden
    }

}

