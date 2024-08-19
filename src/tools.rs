use spirit_edit_core::zones::ZoneResource;
use bevy_clay_tiles::tile_edit::ModifyTileTool;
use bevy_clay_tiles::tile_edit::TileEditingResource; 
use crate::ui::SubTool;
 
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
            
            update_clay_tiles_tool_state,
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

impl   EditingToolData {
    fn from_editor_tool_state(state: EditorToolsState) -> Option<Self> {
        let editing_tool = EditingTool::from_editor_tool_state(state.clone())?;

       Some(  Self {
            editing_tool,
            brush_radius: state.brush_radius as f32,
            brush_type: state.brush_type,
            brush_hardness: (state.brush_hardness as f32) / 100.0,
        } )
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


impl   EditingTool {
    fn from_editor_tool_state(state: EditorToolsState) -> Option<Self> {
 

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



                        let tiles_edit_tool_mode =  match sub_tool {
                            
    

                            Some(SubTool::BuildTileRectangle) => Some( TilesEditingTool::BuildTile( 
                               BuildTileTool::RectangleTileBuild  
                            ) ),

                                Some(SubTool::BuildTilePolygon) => Some( TilesEditingTool::BuildTile( 
                               BuildTileTool::PolygonTileBuild  
                            ) ),

                            Some(SubTool::ModifyTileHeight) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyTileHeight)  ),

                             Some(SubTool::ModifyTileBevel) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyTileBevel)  ),

                                Some(SubTool::ModifyTileType) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyTileType)  ),


                            _ => None 
                        };


                        tiles_edit_tool_mode.map(  |mode|  EditingTool::TilesEditingTool(mode)   )
 


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



fn update_clay_tiles_tool_state (
    
   // mut contexts: EguiContexts,

     editor_tools_state: Res<EditorToolsState>,
     mut tile_edit_resource: ResMut<TileEditingResource>,

     zone_resource: Res<ZoneResource> ,
) {
     
    let mut selected_tile_tool = None ; 
    

        let tile_layer_height = editor_tools_state.color.r  as u32;


        let tile_type = editor_tools_state.color.g as usize;

        let build_mesh_height = editor_tools_state.color.b as f32; 




      if let Some(tool_data)  = EditingToolData::from_editor_tool_state (editor_tools_state.clone())    {


        
         match tool_data.editing_tool {
                EditingTool::TilesEditingTool(edit_tool) =>  {
                    selected_tile_tool = Some(edit_tool) ;
                }


                _ => {}

            }
      };


     tile_edit_resource.set_build_layer_height(  tile_layer_height );

     tile_edit_resource.set_build_tile_type(  tile_type );

     tile_edit_resource.set_selected_tool ( selected_tile_tool  );
 
     tile_edit_resource.set_build_mesh_height( build_mesh_height );
 
     if let Some(primary_zone) = &zone_resource.primary_zone {
         tile_edit_resource.set_new_tile_parent_entity ( Some(*primary_zone) );


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

    let Some(tool_data)  = EditingToolData::from_editor_tool_state (editor_tools_state.clone())  else {

        return 
    };

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