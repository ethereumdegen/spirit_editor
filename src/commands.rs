//use bevy_foliage_paint::edit::FoliageCommandEvent;
use spirit_edit_core::prefabs::PrefabEvent;
use spirit_edit_core::zones::ZoneEvent;
use bevy_foliage_tool::edit::FoliageCommandEvent;
use bevy::prelude::*;

use bevy_mesh_terrain::edit::EditingTool;
use bevy_mesh_terrain::{
    edit::{EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    TerrainMeshPlugin,
};

 
use bevy_regions::edit::RegionCommandEvent;

pub fn update_commands(
    key_input: Res<ButtonInput<KeyCode>>, //detect mouse click

   // mut edit_event_writer: EventWriter<EditTerrainEvent>,
   // mut command_event_writer: EventWriter<TerrainCommandEvent>,
   // mut region_command_event_writer: EventWriter<RegionCommandEvent>,

    mut commands: Commands,  
   // mut foliage_command_event_writer: EventWriter<FoliageCommandEvent>,
) {
    if key_input.pressed(KeyCode::ControlLeft) || key_input.pressed(KeyCode::ControlRight) {
        if key_input.just_pressed(KeyCode::KeyS) {
            println!("saving chunks !");

            commands.send_event(TerrainCommandEvent::SaveAllChunks(true, true, true));
            commands.send_event(RegionCommandEvent::SaveAll);

             commands.send_event(ZoneEvent::SaveAllZones);
             commands.send_event(FoliageCommandEvent::SaveAll );


             commands.send_event(PrefabEvent::SaveAllPrefabs);

           // foliage_command_event_writer.send(FoliageCommandEvent::SaveAll);
        }
    }
}
