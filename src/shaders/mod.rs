
use bevy::prelude::*; 

pub mod fixed_space_uv; 
pub mod fixed_space_uv_material;

pub fn shaders_plugin(app: &mut App) {
    app
      .add_plugins(fixed_space_uv::fixed_space_uv_plugin)
        .add_plugins(fixed_space_uv_material::fixed_space_uv_material_plugin)
       ;
}
