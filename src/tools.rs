 
use spirit_edit_core::placement::PlacementResource;
use bevy_clay_tiles::tile_edit::ModifyTileTool;
use bevy_clay_tiles::tile_edit::TileEditingResource; 
use crate::editor_state::EditorStateResource;
use crate::ui::SubTool;


use bevy::picking::backend::ray::RayMap;
 
use bevy_clay_tiles::tile_edit::BuildTileTool;
use bevy::prelude::*;

use crate::editor_pls::bevy_pls_editor_is_active;
use crate::ui::{BrushType, EditorToolsState, ToolMode};
use degen_toon_terrain::edit::{ BrushType as TerrainBrushType,  EditingTool as TerrainEditingTool , TerrainBrushEvent};

use bevy_regions::edit::{
    BrushType as RegionsBrushType, 
    EditRegionEvent, 
    EditingTool as RegionsEditingTool,
     RegionBrushEvent};
 

use bevy_foliage_tool::edit::{
    BrushType as FoliageBrushType, 
    EditFoliageEvent, 
    EditingTool as FoliageEditingTool,
     FoliageBrushEvent}; 


use bevy_clay_tiles::tile_edit::{EditingTool as TilesEditingTool} ;

     
use degen_toon_terrain::terrain_config::TerrainConfig;
use degen_toon_terrain::{
    edit::{  EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    tool_preview::{ToolPreviewResource as TerrainToolPreviewResource},
    TerrainMeshPlugin,
};

use bevy_regions::tool_preview::{ToolPreviewResource as RegionsToolPreviewResource};

use bevy_egui::EguiContexts;

 

pub fn brush_tools_plugin(app: &mut App) {
    app


    .add_systems(
        Update,
        (
            
            
            update_brush_paint,
            handle_brush_events_from_terrain,




            ).chain().run_if(not(bevy_pls_editor_is_active)),
    )


    .add_systems(
        Update,
        (
            
            update_clay_tiles_tool_state,
            

            ).chain() ,
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
    TilesEditingTool(TilesEditingTool),
  //  PlaceDoodads, 
     FoliageEditingTool(FoliageEditingTool),

}



/*

make a system so when brush type changes, the editing tool will change ... to be one that is valid ! s

*/


impl   EditingTool {
    fn from_editor_tool_state(state: EditorToolsState) -> Option<Self> {
 

        match state.tool_mode {
                    

                   ToolMode::TerrainGen => {

                         let sub_tool = state.sub_tool; 
                       /* EditingTool::TerrainEditingTool( TerrainEditingTool::SetHeightMap {
                                            height: state.color.r,
                                        })*/

                                        // ???? 
                        match sub_tool{
                       //   Some(SubTool::TerrainGeneration) => Some(EditingTool::TerrainGenerationTool  ),
                          
                           _ => None 

                        }


                    },
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

                         /*  Some(SubTool::TerrainSplatUltra) => Some( EditingTool::TerrainEditingTool( 
                                TerrainEditingTool::SetSplatMapUltra {
                                texture_indices: state.layered_splatmap_data.texture_indices.clone(),
                            //    texture_strengths: state.layered_splatmap_data.texture_strengths.clone(),  
                            }) ),*/

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


                                Some(SubTool::BuildTileLinear) => Some( TilesEditingTool::BuildTile( 
                               BuildTileTool::LinearTileBuild  
                            ) ),

                            Some(SubTool::ModifyTileDragSides) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyDragSides)  ),

                            Some(SubTool::ModifyTileDragVertices) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyDragVertices)  ),

                            /*
                             Some(SubTool::ModifyTileBevel) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyTileBevel)  ),

                                Some(SubTool::ModifyTileType) => Some( TilesEditingTool::ModifyTile(
                                    ModifyTileTool::ModifyTileType)  ),*/


                            _ => None 
                        };


                        tiles_edit_tool_mode.map(  |mode|  EditingTool::TilesEditingTool(mode)   )
 


                    },
                     ToolMode::Regions => Some( EditingTool::RegionsEditingTool( RegionsEditingTool::SetRegionMap {
                        region_index: state.color.r as u8,
                    }) ),  

                     // ToolMode::Doodads =>  None ,  


                    ToolMode::Foliage =>  Some( EditingTool::FoliageEditingTool( 
                        FoliageEditingTool::SetFoliageDensity {
                          foliage_index: state.color.r as u8 ,
                          density: state.color.g as u8,
                    }) ),  
            } 
       
    }
}





 




 




fn update_clay_tiles_tool_state (
    
   // mut contexts: EguiContexts,

     editor_tools_state: Res<EditorToolsState>,
     mut tile_edit_resource: ResMut<TileEditingResource>,
     
     editor_state_resource: Res<EditorStateResource>,

     pls_editor_resource: Res<bevy_editor_pls::editor::Editor> ,

    placement_resource: Res<PlacementResource> ,
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


      let mut tool_enabled = true;
      
    if  editor_state_resource.cursor_overlaps_gui {
       tool_enabled = false ;
    }

     if pls_editor_resource.active() {
        tool_enabled = false ;
    }





    tile_edit_resource.set_tool_enabled(tool_enabled);
 

     tile_edit_resource.set_build_layer_height(  tile_layer_height );

     tile_edit_resource.set_build_tile_type(  tile_type );

     tile_edit_resource.set_selected_tool ( selected_tile_tool  );
 
     tile_edit_resource.set_build_mesh_height( build_mesh_height );
 
     if let Some(primary_parent) = &placement_resource.placement_parent {
         tile_edit_resource.set_new_tile_parent_entity ( Some(*primary_parent) );


     }
 

 }



fn update_brush_paint(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    ray_map: Res<RayMap>,
    mut raycast: MeshRayCast,

    mut edit_terrain_event_writer: EventWriter<EditTerrainEvent>,
    mut edit_foliage_event_writer: EventWriter<EditFoliageEvent>,
    mut edit_regions_event_writer: EventWriter<EditRegionEvent>,
    // command_event_writer: EventWriter<TerrainCommandEvent>,
    editor_tools_state: Res<EditorToolsState>,

    mut terrain_tool_preview_state: ResMut<TerrainToolPreviewResource>,
    mut regions_tool_preview_state: ResMut<RegionsToolPreviewResource>,

    editor_state_resource: Res<EditorStateResource>,
 //   mut contexts: EguiContexts,
) {
     

    
    if  editor_state_resource.cursor_overlaps_gui {
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



     for (_, cursor_ray) in ray_map.iter() {
        if let Some((intersection_entity, intersection_data)) =
            raycast.cast_ray(*cursor_ray, &default()).first()
        {
            let hit_point = intersection_data.point;

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
                EditingTool::TerrainEditingTool(ref terrain_edit_tool) =>  {

                 let  terrain_brush_type = match &brush_type {
                    BrushType::SetExact => TerrainBrushType::SetExact,
                    BrushType::Smooth => TerrainBrushType::Smooth,
                    BrushType::Noise => TerrainBrushType::Noise,
                    BrushType::EyeDropper => TerrainBrushType::EyeDropper,
                     BrushType::ClearAll => TerrainBrushType::ClearAll,

                       BrushType::RaiseLower => TerrainBrushType::RaiseLower,

                         
                };

                       edit_terrain_event_writer.send(EditTerrainEvent {
                            entity: intersection_entity.clone(),
                            tool: terrain_edit_tool.clone(),
                            brush_type: terrain_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });
                  

                },
               EditingTool::FoliageEditingTool(ref foliage_edit_tool) => {

                       let  foliage_brush_type = match &brush_type {
                            BrushType::SetExact => Some(FoliageBrushType::SetExact),
                             BrushType::EyeDropper => Some(FoliageBrushType::EyeDropper),
                            _ => None 
                        };
                        info!("send edit foliage event ");

                    if let Some(foliage_brush_type) = foliage_brush_type {
                         edit_foliage_event_writer.send(EditFoliageEvent {   
                                entity: intersection_entity.clone(),
                                tool: foliage_edit_tool.clone(),
                                brush_type:foliage_brush_type,
                                brush_hardness,
                                coordinates: hit_coordinates,
                                radius,
                            });
                     }
                      
                }, 
                 EditingTool::RegionsEditingTool(ref region_edit_tool) => {

                       let  regions_brush_type = match &brush_type {
                            BrushType::SetExact => Some(RegionsBrushType::SetExact),
                            BrushType::EyeDropper => Some(RegionsBrushType::EyeDropper),
                            _ => None,
                            
                        };

                    if let Some(regions_brush_type) = regions_brush_type {
                        edit_regions_event_writer.send(EditRegionEvent {   
                            entity: intersection_entity.clone(),
                            tool: region_edit_tool.clone(),
                            brush_type:regions_brush_type,
                            brush_hardness,
                            coordinates: hit_coordinates,
                            radius,
                        });
                    }
                    
                      
                },

                 EditingTool::TilesEditingTool(ref tiles_edit_tool) => {

                    //the plugin handles this ..


                                           
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
            TerrainBrushEvent::EyeDropSplatMap { texture_indices } => {

                editor_tools_state.color.r = texture_indices[0] as u16;
                 editor_tools_state.color.g = texture_indices[1] as u16;
                  editor_tools_state.color.b = texture_indices[2] as u16;

             
                  info!("editor_tools_state {:?}", editor_tools_state.color );
            }
        }


    }
}
