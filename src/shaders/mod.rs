
use bevy::prelude::*; 

pub mod rock_magic; 
pub mod magic_rock_material;

pub fn shaders_plugin(app: &mut App) {
    app
      .add_plugins(rock_magic::rock_magic_plugin)
        .add_plugins(magic_rock_material::magic_rock_material_plugin)
       ;
}
