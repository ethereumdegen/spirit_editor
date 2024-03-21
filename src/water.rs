
use bevy::prelude::*;

use bevy_water::*;

const WATER_HEIGHT: f32 = 5.0;

pub(crate) fn water_plugin(app: &mut App) {
    app.insert_resource(WaterSettings {
        height: WATER_HEIGHT,
        ..default()
    })
    .add_plugins(WaterPlugin);
}
