

use degen_toon_terrain::chunk::TerrainImageDataNeedsReload;
use degen_toon_terrain::terrain::TerrainImageDataLoadStatus;
use degen_toon_terrain::heightmap::HeightMap;
use degen_toon_terrain::hypersplat::ChunkSplatDataRaw;
use degen_toon_terrain::chunk::ChunkHeightMapResource;
use degen_toon_terrain::terrain::TerrainData;
use degen_toon_terrain::terrain_config::TerrainConfig;
use degen_toon_terrain::chunk::Chunk;
use degen_toon_terrain::chunk::ChunkData;
use noise::NoiseFn;
use noise::Perlin;
use bevy::prelude::*; 

pub fn terrain_generation_plugin(app: &mut App){


	app
		.add_event::<GenerateTerrainEvent>()

		.add_observer(  generate_terrain )
	 	
	;

}

#[derive(Event)]
pub struct GenerateTerrainEvent ; 






fn generate_terrain(


	_trigger: Trigger<GenerateTerrainEvent>,

	terrain_query: Query<(&TerrainData, &TerrainConfig)>,
	chunk_query: Query<(Entity, &Chunk, & ChunkData, &ChunkSplatDataRaw, &Parent, &Children)>,

    mut chunk_height_maps: ResMut<ChunkHeightMapResource>,

    mut commands : Commands , 

) {


    info!( "generating terrain ! ! " );

	for (chunk_entity, chunk, chunk_data, chunk_splat_data, parent_terrain_entity, chunk_children) in chunk_query.iter() {


		    let terrain_entity_id = parent_terrain_entity.get();
		    if terrain_query.get(terrain_entity_id).is_ok() == false {
	                continue;
	            }

	        let (terrain_data, terrain_config) = terrain_query.get(terrain_entity_id).unwrap();
	       

	         let chunk_id = chunk.chunk_id ;

	      //   let file_name = format!("{}.png", chunk.chunk_id);
           //   let asset_folder_path = PathBuf::from("assets");



            let terrain_dimensions = &terrain_config.terrain_dimensions;
            let chunk_rows = &terrain_config.chunk_rows ;



               let   chunk_dimensions = [terrain_dimensions.x as u32 / chunk_rows, 
               terrain_dimensions.y as u32  / chunk_rows];//compute me from terrain config


				  // Generate noise map for the current chunk
            let width = chunk_dimensions[0] as usize;
            let height = chunk_dimensions[1] as usize;

            let offset_x =  chunk_id / chunk_rows;
            let offset_y = chunk_id % chunk_rows;

            let horizontal_scale = 1.0 / terrain_dimensions.x ;

            let seed = 0 ;

            let octaves = 1;

            let persistence = 0.5 ;

            let amplitude_scale = 1.0 ; 


            let height_noise_map = generate_terrain_noise_map(
                width,
                height,
                offset_x as f64,
                offset_y as f64 ,
                horizontal_scale.into(),
                amplitude_scale,
                seed,
                octaves, 
                persistence
            );

        	chunk_height_maps.chunk_height_maps.insert( chunk.chunk_id,  height_noise_map   );


            commands.trigger_targets(   TerrainImageDataNeedsReload  , chunk_entity  );

            
              



	}


	//   pub type HeightMapU16 = Vec<Vec<u16>>;

 	




}

/*

pub struct GenerateTerrain {
    pub width: usize,
    pub height: usize,
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
    pub seed: u32,
}
*/

pub fn generate_terrain_noise_map(


    width: usize,
    height: usize,
    offset_x: f64,
    offset_y: f64,
    horizontal_scale: f64,
    amplitude_scale: f64,
    seed: u32,
    octaves: usize,
    persistence: f64,
) -> Vec<Vec<u16>> {
    // Create a seeded noise generator
    let perlin = Perlin::new(seed);

    // Initialize the height map
    let mut noise_map: Vec<Vec<u16>> = vec![vec![0; width]; height];

    // Generate heights
    for y in 0..height {
        for x in 0..width {
            // Map x and y to world coordinates with offset and scale
            let world_x = (x as f64 ) + offset_x * width as f64;
            let world_y = (y as f64 ) + offset_y * height as f64;

            // Log the input coordinates to debug
            info!("world_x: {}, world_y: {}", world_x, world_y);

            // Compute fractal noise (FBM)
            let mut noise_value = 0.0;
            let mut amplitude = amplitude_scale;
            let mut frequency = 1.0;
            let mut max_amplitude = 0.0;

            for _ in 0..octaves {
                noise_value += perlin.get([world_x  * horizontal_scale, world_y * horizontal_scale ]) * amplitude;
                max_amplitude += amplitude;
                amplitude *= persistence;
                frequency *= 2.0;
            }

            // Normalize the noise value to range [-1.0, 1.0]
            noise_value /= max_amplitude;

            // Map noise value (-1.0 to 1.0) to u16 (0 to 65535)
            let height = ((noise_value + 1.0) / 2.0 * u16::MAX as f64) as u16;

            // Log noise values for debugging
            info!("noise_value: {:?}, height: {}", noise_value, height);

            // Store the height
            noise_map[y][x] = height;
        }
    }

    noise_map
}
