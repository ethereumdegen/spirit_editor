use crate::ui::SubTool;
use bevy_clay_tiles::tile_edit::RectangleTileBuildTool;
use bevy_clay_tiles::tile_edit::BuildTileTool;
use bevy::prelude::*;

use crate::editor_pls::bevy_pls_editor_is_active;
use crate::ui::{BrushType, EditorToolsState, ToolMode};
use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::edit::{ BrushType as TerrainBrushType,  EditingTool as TerrainEditingTool , TerrainBrushEvent};

use bevy_regions::edit::{
    BrushType as RegionsBrushType, 
    EditRegionEvent, 
    EditingTool as RegionsEditingTool,
     RegionBrushEvent};
/*
use bevy_foliage_paint::edit::{
    BrushType as FoliageBrushType, 
    EditFoliageEvent, 
    EditingTool as FoliageEditingTool,
     FoliageBrushEvent};*/

use bevy_clay_tiles::tile_edit::{EditingTool as TilesEditingTool} ;

     
use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::{
    edit::{  EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    tool_preview::{ToolPreviewResource as TerrainToolPreviewResource},
    TerrainMeshPlugin,
};

use bevy_regions::tool_preview::{ToolPreviewResource as RegionsToolPreviewResource};

use bevy_egui::EguiContexts;

use bevy_mod_raycast::prelude::*;

pub fn brush_tools_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_brush_paint,
            handle_brush_events_from_terrain,



            ).chain().run_if(not(bevy_pls_editor_is_active)),
    );
}

struct EditingToolData {
    editing_tool: EditingTool,
    brush_type: BrushType,
    brush_radius: f32,
    brush_hardness: f32,
}

impl From<EditorToolsState> for EditingToolData {
    fn from(state: EditorToolsState) -> Self {
        let editing_tool = EditingTool::from(state.clone());

        Self {
            editing_tool,
            brush_radius: state.brush_radius as f32,
            brush_type: state.brush_type,
            brush_hardness: (state.brush_hardness as f32) / 100.0,
        }
    }
}


enum EditingTool {

    TerrainEditingTool(TerrainEditingTool),
    RegionsEditingTool(RegionsEditingTool),
    TilesEditingTool(TilesEditingTool)
   // FoliageEditingTool(FoliageEditingTool),

}



/*

make a system so when brush type changes, the editing tool will change ... to be one that is valid ! s

*/


impl From<EditorToolsState> for EditingTool {
    fn from(state: EditorToolsState) -> Option<Self> {
 

        match state.tool_mode {
                    ToolMode::Terrain => {

                         let sub_tool = state.sub_tool; 
                       /* EditingTool::TerrainEditingTool( TerrainEditingTool::SetHeightMap {
                                            height: state.color.r,
                                        })*/

                        match sub_tool{
                          Some(SubTool::TerrainHeight) => Some(EditingTool::TerrainEditingTool( 
                             TerrainEditingTool::SetHeightMap {
                                height: state.color.r,
                            }) ),
                           Some(SubTool::TerrainSplat) => Some( EditingTool::TerrainEditingTool( 
                                TerrainEditingTool::SetSplatMap {
                                r: state.color.r as u8,
                                g: state.color.g as u8,
                                b: state.color.b as u8,
                            }) ),

                           _ => None 

                        }


                    },
                    ToolMode::Tiles =>{ 


                        let sub_tool = state.sub_tool; 
                        // make this depend on sub tool ! 
                        let tiles_edit_tool_mode = TilesEditingTool::BuildTile( 
                         BuildTileTool::RectangleTileBuild( 
                            //remove this?? depends on plugin states 
                            RectangleTileBuildTool::PlaceOrigin
                            ) ) ;

                        Some(  EditingTool::TilesEditingTool(
                            tiles_edit_tool_mode  ) )


                    },
                     ToolMode::Regions => Some( EditingTool::RegionsEditingTool( RegionsEditingTool::SetRegionMap {
                        region_index: state.color.r as u8,
                    }) ),  
                  /*  ToolMode::Foliage => EditingTool::FoliageEditingTool( 
                        FoliageEditingTool::SetFoliageDensity {
                        density: state.color.r as u8,
                    }),  */
            } 
       
    }
}

fn update_brush_paint(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    mut edit_terrain_event_writer: EventWriter<EditTerrainEvent>,
   // mut edit_foliage_event_writer: EventWriter<EditFoliageEvent>,
    mut edit_regions_event_writer: EventWriter<EditRegionEvent>,
    // command_event_writer: EventWriter<TerrainCommandEvent>,
    editor_tools_state: Res<EditorToolsState>,

    mut terrain_tool_preview_state: ResMut<TerrainToolPreviewResource>,
    mut regions_tool_preview_state: ResMut<RegionsToolPreviewResource>,

    mut contexts: EguiContexts,
) {
     

    let egui_ctx = contexts.ctx_mut();
    if egui_ctx.is_pointer_over_area() {
        return;
    }

    //if tool is paintbrush ... (conditional check)

    //make me dynamic or whatever
    // let tool = EditingTool::SetHeightMap(125,25.0, false);

    let tool_data: EditingToolData = (*editor_tools_state).clone().into();

    let radius = tool_data.brush_radius;
    let brush_hardness = tool_data.brush_hardness;
    let brush_type = tool_data.brush_type;

    // let tool = EditingTool::SetSplatMap(5,1,0,25.0,false);



    if let Some(cursor_ray) = **cursor_ray {
        if let Some((intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &default()).first()
        {
            let hit_point = intersection_data.position();

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec2::new(hit_point.x, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there


            terrain_tool_preview_state.tool_coordinates = hit_coordinates.clone();
            terrain_tool_preview_state.tool_radius = radius.clone();
            terrain_tool_preview_state.tool_color = (0.6,0.6,0.9).into() ;


            regions_tool_preview_state.tool_coordinates = hit_coordinates.clone();
            regions_tool_preview_state.tool_radius = radius.clone();
            regions_tool_preview_state.tool_color = (0.6,0.6,0.9).into() ;




             if !mouse_input.pressed(MouseButton::Left) {
                return;
            }


            match tool_data.editing_tool {
                EditingTool::TerrainEditingTool(terrain_edit_tool) =>  {

                 let  terrain_brush_type = match &brush_type {
                    BrushType::SetExact => TerrainBrushType::SetExact,
                    BrushType::Smooth => TerrainBrushType::Smooth,
                    BrushType::Noise => TerrainBrushType::Noise,
                    BrushType::EyeDropper => TerrainBrushType::EyeDropper,
                };

                       edit_terrain_event_writer.send(EditTerrainEvent {
                            entity: intersection_entity.clone(),
                            tool: terrain_edit_tool,
                            brush_type: terrain_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });
                  

                },
               /* EditingTool::FoliageEditingTool(foliage_edit_tool) => {

                       let  foliage_brush_type = match &brush_type {
                            BrushType::SetExact => FoliageBrushType::SetExact,
                            BrushType::Smooth => FoliageBrushType::SetExact,
                            BrushType::Noise => FoliageBrushType::SetExact,
                            BrushType::EyeDropper => FoliageBrushType::EyeDropper,
                        };

                     edit_foliage_event_writer.send(EditFoliageEvent {   
                            entity: intersection_entity.clone(),
                            tool: foliage_edit_tool,
                            brush_type:foliage_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });
                      
                },*/
                 EditingTool::RegionsEditingTool(region_edit_tool) => {

                       let  regions_brush_type = match &brush_type {
                            BrushType::SetExact => RegionsBrushType::SetExact,
                            BrushType::Smooth => RegionsBrushType::SetExact,
                            BrushType::Noise => RegionsBrushType::SetExact,
                            BrushType::EyeDropper => RegionsBrushType::EyeDropper,
                        };

                     edit_regions_event_writer.send(EditRegionEvent {   
                            entity: intersection_entity.clone(),
                            tool: region_edit_tool,
                            brush_type:regions_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });
                      
                },

                 EditingTool::TilesEditingTool(tiles_edit_tool) => {

                    //the plugin handles this ..


                       /*let  regions_brush_type = match &brush_type {
                            BrushType::SetExact => RegionsBrushType::SetExact,
                            BrushType::Smooth => RegionsBrushType::SetExact,
                            BrushType::Noise => RegionsBrushType::SetExact,
                            BrushType::EyeDropper => RegionsBrushType::EyeDropper,
                        };

                     edit_regions_event_writer.send(EditRegionEvent {   
                            entity: intersection_entity.clone(),
                            tool: region_edit_tool,
                            brush_type:regions_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });*/
                      
                },
            }

         
        }
    }
}



fn handle_brush_events_from_terrain(
    mut evt_reader: EventReader<TerrainBrushEvent>,
    mut editor_tools_state: ResMut<EditorToolsState>,
){


    for evt in evt_reader.read(){

        info!("learned of evt {:?}", evt );
        match evt{
            TerrainBrushEvent::EyeDropTerrainHeight { height } => {

                editor_tools_state.color.r = * height ; 

            },
            TerrainBrushEvent::EyeDropSplatMap { r, g, b } => {

                editor_tools_state.color.r = * r as u16 ;
                editor_tools_state.color.g = * g as u16 ;
                editor_tools_state.color.b = * b as u16 ; 

            }
        }


    }
}