//use crate::dev::dev_editor::dev_editor_plugin;

use degen_toon_terrain::terrain::TerrainData;

use iyes_perf_ui::entries::PerfUiAllEntries;
use iyes_perf_ui::PerfUiPlugin;

use bevy::prelude::*;
use bevy::platform::collections::HashSet;

 
pub(crate) fn benchmarking_plugin(app: &mut App) {
    {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
           
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, spawn_perf_ui)
          


          ;
          
          
    }
}

fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}
 