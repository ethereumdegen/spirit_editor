 
use bevy_clay_tiles::tile_types_config::ClayTilesTypesConfigResource;
use crate::terrain::terrain_manifest::{TerrainManifestResource,TerrainManifest};
use bevy::prelude::*;

use bevy_egui::EguiContexts;
use bevy_egui::{egui };

use bevy_mesh_terrain::edit::{BrushType as TerrainBrushType, TerrainCommandEvent};
use bevy_regions::edit::{BrushType as RegionsBrushType, RegionCommandEvent};
use spirit_edit_core::zones::ZoneEvent;
use bevy_foliage_tool::edit::{BrushType as FoliageBrushType, FoliageCommandEvent};

use std::fmt::{self, Display, Formatter};

use crate::editor_pls::bevy_pls_editor_is_active;

pub fn editor_ui_plugin(app: &mut App) {
    app
        .init_resource::<EditorToolsState>()
       // .add_plugins(EguiPlugin)  // only add this if it hasnt been added 
        .add_systems(Update, editor_tools_ui.run_if(not(bevy_pls_editor_is_active))) 

         //.add_systems(Update, force_update_tool_mode )

         ;
}

#[derive(Default, Resource, Clone)]
pub struct LinearPixelColor {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

#[derive(Default, Resource, Clone)]
pub struct EditorToolsState {
    pub tool_mode: ToolMode,    
    pub sub_tool: Option<SubTool>,
    pub brush_type: BrushType,
    pub brush_radius: u32,
    pub brush_hardness: u32,
    pub color: LinearPixelColor, //brush mode
}


#[derive(Clone ,PartialEq)]
pub enum BrushType {
 
    SetExact,
    ClearAll,
    Smooth,
    Noise,
    EyeDropper
}

impl BrushType{

    pub fn to_string(&self) -> String{

        match self {

            BrushType::SetExact  => "Set Exact".into(),
            BrushType::ClearAll => "Clear All".into(),
             BrushType::Smooth  => "Smooth".into(),
             BrushType::Noise  => "Noise".into(),
             BrushType::EyeDropper  => "Eyedropper".into(),

            

        }

    }
}

impl Default for BrushType {
    fn default() -> Self {
      BrushType::SetExact 
    }
} 

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub enum ToolMode {
    #[default]
    //Height,
    //Splat,
    Terrain,
    Foliage, 
    Regions,
    Tiles,
    //Doodads,
}



#[derive(Clone,Debug,PartialEq,Eq)]
pub enum SubTool {

    TerrainHeight,
    TerrainSplat, 

    BuildTileRectangle,
    BuildTileLinear,
    BuildTilePolygon,
    ModifyTileDragSides,
    ModifyTileDragVertices,
 //   ModifyTileType, 

}


impl SubTool{

    pub fn to_string(&self) -> String{

        match self {

            Self::TerrainHeight  => "Terrain Height".into(),
            Self::TerrainSplat  => "Terrain Splat".into(),

            Self::BuildTileRectangle  => "Build: Rectangle".into(),
            Self::BuildTileLinear  => "Build: Linear".into(),
            Self::BuildTilePolygon  => "Build: Polygon".into(),

            Self::ModifyTileDragSides => "Modify: Drag Sides".into(),
            Self::ModifyTileDragVertices => "Modify: Drag Vertices".into(),

           /* Self::ModifyTileHeight  => "Modify: Height".into(),
            Self::ModifyTileBevel  => "Modify: Bevel".into(),
            Self::ModifyTileType  => "Modify: Tile Type".into(),*/
                
  
        }

    }
}




const TOOL_MODES: [ToolMode; 4] = [
ToolMode::Terrain,
//ToolMode::Height, 
//ToolMode::Splat, 
ToolMode::Foliage, 
ToolMode::Regions,
ToolMode::Tiles,
 //ToolMode::Doodads
];

const TERRAIN_SUBTOOLS : [SubTool; 2] = [
    SubTool::TerrainHeight,
    SubTool::TerrainSplat, 

];

const TILE_SUBTOOLS : [SubTool; 5] = [
    SubTool::BuildTileRectangle,
    SubTool::BuildTileLinear, 
    SubTool::BuildTilePolygon, 
    SubTool::ModifyTileDragSides,
    SubTool::ModifyTileDragVertices
];





const BRUSH_TYPES_HEIGHT: [ BrushType; 4] = [
BrushType::SetExact , 
BrushType::Smooth , 
BrushType::Noise , 
BrushType::EyeDropper
];


const BRUSH_TYPES_SPLAT: [ BrushType; 3] = [
BrushType::SetExact , 
 BrushType::ClearAll , 
 BrushType::EyeDropper
];
const BRUSH_TYPES_REGION: [BrushType; 2] = [
BrushType::SetExact ,   
BrushType::EyeDropper
];

//consider adding more stuff bc of bitmasking 
const BRUSH_TYPES_FOLIAGE: [BrushType; 2] = [
BrushType::SetExact ,   
BrushType::EyeDropper
];


impl Display for ToolMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            ToolMode::Terrain => "Terrain",
            ToolMode::Tiles => "Tiles",
            ToolMode::Foliage => "Foliage",
            ToolMode::Regions => "Regions",
          //    ToolMode::Doodads => "Doodads"
        };

        write!(f, "{}", label)
    }
}

fn editor_tools_ui(
    mut tools_state: ResMut<EditorToolsState>,

    mut command_event_writer: EventWriter<TerrainCommandEvent>,
    mut foliage_command_event_writer: EventWriter<FoliageCommandEvent>,
    mut region_command_event_writer: EventWriter<RegionCommandEvent>,
    mut zone_event_writer: EventWriter<ZoneEvent>,

    mut contexts: EguiContexts,

    terrain_manifest_res: Res<TerrainManifestResource>,
    terrain_manifest_asset: Res<Assets<TerrainManifest>>,


    clay_tiles_config_resource: Res<ClayTilesTypesConfigResource>, 
) {
    egui::Window::new("Editor Tools").show(contexts.ctx_mut(), |ui| {

      

        if ui.button("Save All   (Ctrl+S)").clicked() {
            command_event_writer.send(TerrainCommandEvent::SaveAllChunks(true, true, true));
            region_command_event_writer.send(RegionCommandEvent::SaveAll );
            zone_event_writer.send(ZoneEvent::SaveAllZones);
            foliage_command_event_writer.send(FoliageCommandEvent::SaveAll );
        }

       // if ui.button("Save Splat and Height").clicked() {
      //      command_event_writer.send(TerrainCommandEvent::SaveAllChunks(true, true, false));
      //  }

        ui.spacing();
        ui.separator();

        /*ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut tools_state.name)
                .labelled_by(name_label.id);
        });*/

        ui.heading("Tool Mode");
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.spacing();
            egui::ComboBox::new("tool_mode", "")
                .selected_text(tools_state.tool_mode.to_string())
                .show_ui(ui, |ui| {
                    for tool_mode in TOOL_MODES.into_iter() {
                        if ui
                            .selectable_label(
                                tools_state.tool_mode == tool_mode,
                                tool_mode.to_string(),
                            )
                            .clicked()
                        {
                            tools_state.tool_mode = tool_mode;
                            tools_state.sub_tool = None; //reset .. for now 
                        }
                    }
                });
        });
        ui.spacing();
        ui.separator();



        match tools_state.tool_mode {
            ToolMode::Terrain => {
                   ui.spacing();
                 ui.separator();

             
                  ui.heading("Sub Tool");
                
                let subtool_name = match &tools_state.sub_tool {

                    Some(st) => st.to_string(),
                    None => "None".to_string()
                };

                  egui::ComboBox::new("Terrain Tool", "")
                    .selected_text(subtool_name)
                    .show_ui(ui, |ui| {
                        for sub_tool in TERRAIN_SUBTOOLS.into_iter() {
                            if ui
                                .selectable_label(
                                     Some(sub_tool.clone()) ==  tools_state.sub_tool  ,
                                    sub_tool.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.sub_tool = Some(sub_tool);
                            }
                        }
                    });


                    ui.spacing();
                        ui.separator();


            if let Some(sub_tool) = &tools_state.sub_tool {
                match  sub_tool {

                    SubTool::TerrainSplat => {

                          egui::ComboBox::new("brush_type", "")
                                .selected_text(tools_state.brush_type.to_string())
                                .show_ui(ui, |ui| {
                                    for brush_type in BRUSH_TYPES_SPLAT.into_iter() {
                                        if ui
                                            .selectable_label(
                                                tools_state.brush_type == brush_type,
                                                brush_type.to_string(),
                                            )
                                            .clicked()
                                        {
                                            tools_state.brush_type = brush_type;
                                        }
                                    }
                                });


                            let terrain_index_A = tools_state.color.r.clone();
                           // let terrain_index_B = tools_state.color.g.clone();

                            let terrain_manifest:Option<&TerrainManifest> =  terrain_manifest_res.manifest.as_ref().map(|m| terrain_manifest_asset.get( m )).flatten();
             

                            
                               ui.spacing();
                          ui.add(egui::Slider::new(&mut tools_state.brush_radius, 0..=100).text("Brush Radius"));
                            ui.spacing();
                            ui.add(egui::Slider::new(&mut tools_state.brush_hardness, 0..=100).text("Brush Hardness"));
                            ui.spacing();
                            

             
                              ui.spacing_mut().slider_width = 255.0;
             
                            ui.add(

                                egui::Slider::new(&mut tools_state.color.r, 0..=255)
                                    .text("Texture Index (R_Channel")
                                    .step_by(1.0)
                                    .drag_value_speed(0.1)

                                    ,
                            );

                            if let Some(terrain_def) = terrain_manifest.map(|m| m.get_terrain_type(terrain_index_A) ).flatten() {
                                 ui.label( terrain_def.name.clone() );
                            }
                             ui.spacing_mut().slider_width = 255.0;
                            ui.add(

                                egui::Slider::new(&mut tools_state.color.g, 0..=255)
                                    .text("Texture Strength (G_Channel")
                                     .step_by(1.0)
                                    .drag_value_speed(0.1)
                                    ,
                            );
                                
                                /*
                            if let Some(terrain_def) = terrain_manifest.map(|m| m.get_terrain_type(terrain_index_B) ).flatten() {
                                 ui.label( terrain_def.name.clone() );
                            }*/
                              ui.spacing_mut().slider_width = 255.0;
                            ui.add(
                                egui::Slider::new(&mut tools_state.color.b, 0..=3)
                                    .text("Layer (B_Channel")
                                     .step_by(1.0)
                                    .drag_value_speed(0.1)

                                    ,
                            );

 

                    }

                    SubTool::TerrainHeight => {


                        egui::ComboBox::new("brush_type", "")
                            .selected_text(tools_state.brush_type.to_string())
                            .show_ui(ui, |ui| {
                                for brush_type in BRUSH_TYPES_HEIGHT.into_iter() {
                                    if ui
                                        .selectable_label(
                                            tools_state.brush_type == brush_type,
                                            brush_type.to_string(),
                                        )
                                        .clicked()
                                    {
                                        tools_state.brush_type = brush_type;
                                    }
                                }
                            });
         


                            ui.spacing();
                            ui.add(egui::Slider::new(&mut tools_state.brush_radius, 0..=100).text("Brush Radius"));
                            ui.spacing();
                            ui.add(egui::Slider::new(&mut tools_state.brush_hardness, 0..=100).text("Brush Hardness"));
                            ui.spacing();

                          ui.spacing_mut().slider_width = 300.0;
         
                        ui.add(
                            egui::Slider::new(&mut tools_state.color.r, 0..=65535)
                                .text("Height (R_Channel)")
                               //  .step_by(1.0)
                               // .drag_value_speed(1.0)
                                ,
                        );



                              

                    }


                    _ => {}
                }
              }
               
            }
            ToolMode::Tiles => {



                  ui.heading("Sub Tool");
                

                let subtool_name = match &tools_state.sub_tool {

                    Some(st) => st.to_string(),
                    None => "None".to_string()
                };

                  egui::ComboBox::new("Tiles Tool", "")
                    .selected_text(subtool_name)
                    .show_ui(ui, |ui| {
                        for sub_tool in TILE_SUBTOOLS.into_iter() {
                            if ui
                                .selectable_label(
                                     Some(sub_tool.clone()) ==  tools_state.sub_tool  ,
                                    sub_tool.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.sub_tool = Some(sub_tool);
                            }
                        }
                    });



                let show_tile_build_sliders = match &tools_state.sub_tool {

                     Some(SubTool::BuildTileRectangle) => true,
                     Some(SubTool::BuildTilePolygon) => true,
                     Some(SubTool::BuildTileLinear) => true,
                     _ => false 


                };
 

                if show_tile_build_sliders {





                   ui.add(
                            egui::Slider::new(&mut tools_state.color.r, 0..=1024)
                                .text("Tile Height Offset")
                               //  .step_by(1.0)
                               // .drag_value_speed(1.0)
                                ,
                        );




                      let tile_index = tools_state.color.g.clone()  as usize;
                          
                  
                     let tile_data = clay_tiles_config_resource.tile_type_data.get(  &tile_index ) ;

                     if let Some ( tile_data ) = tile_data { 

                        ui.label( tile_data.name.clone()  ) ;

                     }


                    ui.add(
                            egui::Slider::new(&mut tools_state.color.g, 0..=64)
                                .text("Tile Type")
                               //  .step_by(1.0)
                               // .drag_value_speed(1.0)
                                ,
                        );

                     ui.add(
                            egui::Slider::new(&mut tools_state.color.b, 0..=256)
                                .text("Tile Mesh Height")
                               //  .step_by(1.0)
                               // .drag_value_speed(1.0)
                                ,
                        );


                }

                
            },

             /*ToolMode::Doodads => {
 
                  ui.heading("Placing Doodads (no tool)");

              },*/
            ToolMode::Foliage => {


                 egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_FOLIAGE.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });
    


                        ui.spacing();
                         ui.add(egui::Slider::new(&mut tools_state.brush_radius, 0..=100).text("Brush Radius"));
                         ui.spacing();
                         ui.add(egui::Slider::new(&mut tools_state.brush_hardness, 0..=100).text("Brush Hardness"));
                        ui.spacing();
                                
                       ui.separator();


                     ui.spacing_mut().slider_width = 256.0;  
 
                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=64)
                        .text("Foliage Index (R_Channel)")
                         .step_by(1.0)
                        .drag_value_speed(1.0)
                        ,
                );

                  ui.add(
                    egui::Slider::new(&mut tools_state.color.g, 0..=256)
                        .text("Foliage Density (G_Channel)")
                         .step_by(1.0)
                        .drag_value_speed(1.0)
                        ,
                );


            }  
            ToolMode::Regions => {


                 egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_REGION.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });
   
 
                      ui.spacing_mut().slider_width = 200.0;
 
                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=64)
                        .text("Region Index (R_Channel)")
                         .step_by(1.0)
                        .drag_value_speed(1.0),

                );


            }


        }
    });
}

/*
fn force_update_tool_mode(

    mut editor_tools_state: ResMut<EditorToolsState> ,

    pls_editor_resource: Res<bevy_editor_pls::editor::Editor> 

){

    if pls_editor_resource.active() {
        editor_tools_state.tool_mode = ToolMode::Doodads; 
    }

}*/