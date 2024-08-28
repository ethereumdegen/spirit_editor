

use bevy_foliage_tool::foliage_assets::FoliageAssetsResource;
use bevy_foliage_tool::foliage_assets::FoliageAssetsState;
use bevy_mesh_terrain::chunk::Chunk;
use bevy_mesh_terrain::chunk::CachedHeightmapData;
use crate::ui::ToolMode;
use crate::EditorToolsState;
 
use bevy::prelude::*;


pub struct FoliagePlugin ;  


impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app

            .add_systems(Startup, register_foliage_assets)

            // .add_systems(Update, add_data_for_foliage_chunks)

             //.add_systems(Update, mark_needs_rebuild_for_foliage_chunks) 

           //  .add_systems(Update, update_foliage_root_visibility )
           

            ;
    }
}


//#[derive(Component)]
//pub struct FoliageChunkNeedsRebuild ;   // from height or density edit .. ? 



fn register_foliage_assets(

    asset_server: Res <AssetServer>, 

    mut assets_resource: ResMut<FoliageAssetsResource>, 

    mut next_state: ResMut<NextState<FoliageAssetsState>>, 

) {


    let green_material: StandardMaterial = Color::srgb(0.4, 0.7, 0.6) .into();

    assets_resource.register_foliage_mesh("grass1", asset_server.load( "foliage/meshes/grass1.obj" ));

    assets_resource.register_foliage_material("standard_green", asset_server.add( green_material ));


    next_state.set( FoliageAssetsState::Loaded );
}

  /*
fn add_data_for_foliage_chunks (   

    mut commands:Commands,
   

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

               info!("add y offset data to foliage chunk");

                commands.entity(chunk_entity).insert(
                     

                    //make an enum type for HeightMapU8 and HeightMapU16 
                    FoliageChunkYOffsetData {
                        y_offset_map_data:  height_map_data.to_vec()


                    } 

                );
 
                 commands.entity(chunk_entity).remove::<FoliageChunkNeedsRebuild>();

 
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

fn mark_needs_rebuild_for_foliage_chunks(

    mut commands: Commands, 
    terrain_chunks_query: Query< &Chunk, Changed<CachedHeightmapData> >,


    foliage_chunks_query: Query< (Entity, &FoliageChunk) >

 ){

    for terrain_chunk in terrain_chunks_query.iter(){

        let chunk_id = terrain_chunk.chunk_id;

        for (foliage_chunk_entity, foliage_chunk) in foliage_chunks_query.iter(){

            if chunk_id == foliage_chunk.chunk_id {

                commands.entity(foliage_chunk_entity).insert( FoliageChunkNeedsRebuild );

            }

        }



    }

}


fn update_foliage_root_visibility (

   mut foliage_root_query: Query<&mut Visibility, With<FoliageData>>,
   editor_tools_state: Res<EditorToolsState>



  ){

    let Some( mut region_plane_vis ) = foliage_root_query.get_single_mut().ok() else {return};

    *region_plane_vis = match &editor_tools_state.tool_mode {
        
         ToolMode::Foliage => Visibility::Visible, 
        _ => Visibility::Hidden
    }

}*/