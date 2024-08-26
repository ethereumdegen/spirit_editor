use bevy_clay_tiles::clay_tile_block::ClayTileBlock;
use bevy::prelude::*; 






pub(crate) fn clay_tiles_plugin(app: &mut App) {
    app
    
     // .add_systems(Update, remove_invalid_clay_tiles)

   
      
      ;

}




fn remove_invalid_clay_tiles(  
	mut commands:Commands,


clay_tile_query: Query<(Entity, &ClayTileBlock) >
){

	for (clay_tile_entity, clay_tile_block) in clay_tile_query.iter(){

		if !clay_tile_block.is_complete(){

				commands.get_entity(clay_tile_entity).map(
					|cmd| { cmd.despawn_recursive(); }
				) ;
		}
 
	}


}