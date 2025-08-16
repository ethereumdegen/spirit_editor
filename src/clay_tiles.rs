 use bevy_material_wizard::material_overrides::MaterialOverrideComponent;
use bevy_clay_tiles::clay_tile_block::{ClayTileBlock,ClayTileMaterial};
use bevy::prelude::*; 






pub(crate) fn clay_tiles_plugin(app: &mut App) {
    app
    
      .add_systems(Update, remove_invalid_clay_tiles)
      .add_systems(Update, add_material_handles)

   	
      
      ;

}




fn remove_invalid_clay_tiles(  
	mut commands:Commands,
 
	clay_tile_query: Query<(Entity, &ClayTileBlock) >
){

	for (clay_tile_entity, clay_tile_block) in clay_tile_query.iter(){

		if !clay_tile_block.is_complete(){

				commands.get_entity(clay_tile_entity).map(
					|mut cmd| { cmd.despawn (); }
				) ;
		}
 
	}


}




fn add_material_handles(
    mut commands:Commands,

    block_query: Query<(Entity, &ClayTileMaterial), Added<ClayTileMaterial>>
){

    for (tile_entity, tile_material_comp) in block_query.iter(){

        let material_name = &tile_material_comp.material_name; 


        commands.get_entity(tile_entity).map( |mut cmd| { 

        	cmd.remove::<ClayTileMaterial>();
        	cmd.insert( MaterialOverrideComponent {
      	      material_override:  material_name.clone(),
      	      cascade: true , 
      		  } );

         } );

    }

}