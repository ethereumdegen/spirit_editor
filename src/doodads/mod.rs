
use bevy::prelude::*;

pub(crate) mod doodad;
pub (crate) mod doodad_placement_preview;
pub(crate) mod prefabs;
pub(crate) mod doodad_placement ;
pub(crate) mod doodad_colliders; 




pub(crate) fn doodads_plugin(app: &mut App) {
    app 
         .add_plugins( doodad_colliders::doodad_colliders_plugin )
        	
        	  .add_plugins(doodad::doodad_plugin)
        .add_plugins(doodad_placement_preview::doodad_placement_plugin  )
            .add_plugins(doodad_placement::doodad_placement_plugin)
        .add_plugins(prefabs::prefabs_plugin )    


        ;
}