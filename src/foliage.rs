

use bevy_foliage_paint::foliage_chunk;
use bevy_mesh_terrain::chunk::ChunkHeightMapResource;
use bevy_foliage_paint::foliage_chunk::FoliageChunkYOffsetData;
use bevy_foliage_paint::foliage_chunk::FoliageChunkYOffsetTexture;
use bevy_foliage_paint::density_map::DensityMap;
use bevy_foliage_paint::foliage_chunk::FoliageChunkDensityTexture;
use bevy_foliage_paint::density_map::DensityMapU8;
use bevy_foliage_paint::foliage_chunk::FoliageChunkDensityData;
use bevy_foliage_paint::foliage_chunk::FoliageChunk;
use bevy::prelude::*;


pub struct FoliagePlugin ;  


impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
             .add_systems(Update, add_data_for_foliage_chunks)
           

            ;
    }
}


#[derive(Component)]
pub struct FoliageChunkNeedsRebuild ;   // from height or density edit .. ? 

  
fn add_data_for_foliage_chunks (   

    mut commands:Commands,
  

   // sample_textures_res: Res<SampleTexturesResource>, 

  //  image_assets: Res<Assets<Image>>,


    chunks_query: Query< 
       (Entity,&FoliageChunk)   , 
      Or<(Without<FoliageChunkYOffsetData> , With<FoliageChunkNeedsRebuild> )>    
    > ,


      chunk_height_maps: Res<ChunkHeightMapResource>,


    ){


      for (chunk_entity,foliage_chunk) in chunks_query.iter() {

        let chunk_id = &foliage_chunk.chunk_id;

           if let Some(height_map_data) = &chunk_height_maps.chunk_height_maps.get(chunk_id)
            {
               let raw_height_data = &height_map_data.0;



               info!("add y offset data to foliage chunk");

                commands.entity(chunk_entity).insert(
                    /*
                    FoliageChunkDensityTexture {

                    }
                    */

                    //make an enum type for HeightMapU8 and HeightMapU16 
                    FoliageChunkYOffsetData {
                        y_offset_map_data:  raw_height_data.clone()


                    } 

                );


            }



        //get this from the density images folder .. or a data resource maybe 
      //  let density_map = image_assets.get( sample_textures_res.sample_density_map.clone() );  
        
        //get this from the terrain heightmap stuff
      //  let y_offset_map = image_assets.get( sample_textures_res.sample_y_offset_map.clone() );  



   //if let Some(density_map) = density_map {
           //   if let Some(y_offset_map) = y_offset_map {



                

             //   let dimensions:Vec2  = Vec2::new(256.0,256.0);

             //  let density_map_data = DensityMapU8::load_from_image( density_map  ).unwrap();
              //  let y_offset_map_data = DensityMapU8::load_from_image( y_offset_map  ).unwrap();

               /* commands.entity(chunk_entity).insert( 

                    FoliageChunkDensityData {
                        density_map_data: *density_map_data


                    }

                );


                 commands.entity(chunk_entity).insert(     
               
                    FoliageChunkDensityTexture::default() 
 
                );*/


               
/*
                commands.entity(chunk_entity).insert(     
               
                    FoliageChunkYOffsetTexture::default() 
 
                );
*/
            

 
        }

}