

 
use degen_toon_terrain::{chunk::ChunkHeightMapResource, terrain::TerrainData, terrain_config::TerrainConfig};
use bevy::prelude::*;
use degen_toon_terrain::terrain_loading_state::TerrainLoadingState;


pub fn terrain_loading_plugin(app: &mut App){


	app

	 .add_systems(Update, 
	 	update_terrain_load_state.run_if( not(in_state(TerrainLoadingState::Complete)) )

	 	)
	;

}



fn update_terrain_load_state(


	terrain_data_query: Query< (&TerrainData, &TerrainConfig) > ,
 	chunk_height_maps_resource: Res<ChunkHeightMapResource>,

	mut next_state: ResMut<NextState<TerrainLoadingState>>,

){		

	let Some( (_terrain_data, terrain_config) ) = terrain_data_query.get_single().ok() else {return};

	let  chunk_rows = terrain_config.chunk_rows; 
	let expected_height_maps_count = chunk_rows * chunk_rows;


	let height_maps = &chunk_height_maps_resource.chunk_height_maps;

	let height_maps_count = height_maps.values().len(); 
	


	if height_maps_count >= expected_height_maps_count as usize{
			next_state.set(TerrainLoadingState::Complete)

	}
	

}