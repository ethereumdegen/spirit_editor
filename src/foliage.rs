

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


 


  
fn add_data_for_foliage_chunks (   

    mut commands:Commands,
  

   // sample_textures_res: Res<SampleTexturesResource>, 

    image_assets: Res<Assets<Image>>,


    chunks_query: Query< 
       Entity   , 
    (   With<FoliageChunk>, Without<FoliageChunkDensityData>   )
    > 

    ){


      for chunk_entity in chunks_query.iter() {


        //get this from the density images folder .. or a data resource maybe 
      //  let density_map = image_assets.get( sample_textures_res.sample_density_map.clone() );  
        
        //get this from the terrain heightmap stuff
      //  let y_offset_map = image_assets.get( sample_textures_res.sample_y_offset_map.clone() );  



        if let Some(density_map) = density_map {
              if let Some(y_offset_map) = y_offset_map {



                

             //   let dimensions:Vec2  = Vec2::new(256.0,256.0);

                let density_map_data = DensityMapU8::load_from_image( density_map  ).unwrap();
                let y_offset_map_data = DensityMapU8::load_from_image( y_offset_map  ).unwrap();

                commands.entity(chunk_entity).insert( 

                    FoliageChunkDensityData {
                        density_map_data: *density_map_data


                    }

                );


                 commands.entity(chunk_entity).insert(     
               
                    FoliageChunkDensityTexture::default() 
 
                );


                commands.entity(chunk_entity).insert(     
               
                    FoliageChunkYOffsetData {
                        y_offset_map_data: *y_offset_map_data


                    } 

                );

                commands.entity(chunk_entity).insert(     
               
                    FoliageChunkYOffsetTexture::default() 
 
                );

            }


            }
        }

}