
use bevy::prelude::*; 

//pub mod fixed_space_uv; 
//pub mod fixed_space_uv_material;
pub mod doodad_material;
pub mod material_affine_processor; 

pub mod hologram_material;

pub fn shaders_plugin(app: &mut App) {
    app
    //  .add_plugins(fixed_space_uv::fixed_space_uv_plugin)


       // .add_plugins(fixed_space_uv_material::fixed_space_uv_material_plugin)  // just use doodad_material w the bit flag 


           .add_plugins( doodad_material:: doodad_material_plugin )
             .add_plugins( hologram_material:: hologram_material_plugin )

       ;
}
