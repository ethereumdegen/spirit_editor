

use bevy_foliage_tool::foliage_scene::FoliageRoot;
use bevy_foliage_tool::foliage_chunk::FoliageChunk;
use bevy_foliage_tool::foliage_chunk::FoliageDataSource;
use bevy_foliage_tool::foliage_chunk::FoliageDimensionsData;
use bevy_foliage_tool::foliage_chunk::FoliageHeightMapData;
use degen_toon_terrain::chunk::Chunk;
 
use crate::doodads::doodad::SpawnDoodadEvent;
 
use bevy_foliage_tool::foliage_proto::FoliageProto;
use bevy_foliage_tool::foliage_material::FoliageMaterial;
use bevy_foliage_tool::foliage_material::FoliageMaterialExtension;
use degen_toon_terrain::terrain::TerrainData;
use degen_toon_terrain::terrain_config::TerrainConfig;
use degen_toon_terrain::chunk::ChunkHeightMapResource;
use bevy::platform::collections::hash_map::HashMap; 
use bevy_foliage_tool::foliage_assets::FoliageAssetsState;
 
//use bevy_foliage_tool::foliage_layer::FoliageBaseHeightMapU16;
//use bevy_foliage_tool::foliage_layer::FoliageLayer;
//use bevy_foliage_tool::foliage_layer::FoliageLayerNeedsRebuild;
//use degen_toon_terrain::chunk::CachedHeightmapData;
use degen_toon_terrain::terrain_loading_state;
use degen_toon_terrain::terrain_loading_state::TerrainLoadingState;
use crate::EditorToolsState;

use  bevy_foliage_tool::foliage_assets::FoliageMaterialHandle;
 
use bevy::prelude::*;


pub fn foliage_plugin(app:&mut App){


          app

             
            .add_systems(Update, (


             //   add_height_maps_to_foliage_layers,
                propogate_height_data_change_to_foliage,

               

             //   spawn_foliage_doodads,  //plugin should do this 

                ).chain()
                .run_if(in_state(TerrainLoadingState::Complete))
                )

            ;
}
 

//#[derive(Component)]
//pub struct FoliageChunkNeedsRebuild ;   // from height or density edit .. ? 


/*
fn register_foliage_assets(

    asset_server: Res <AssetServer>, 

    mut assets_resource: ResMut<FoliageAssetsResource>, 

    mut next_state: ResMut<NextState<FoliageAssetsState>>, 

) {


    let foliage_material = FoliageMaterialExtension {
        base: StandardMaterial { 
            base_color:  Color::srgb(0.13, 0.37, 0.11) .into() ,  // not needed ? 
            //double_sided: true ,
            cull_mode: None, 
             ..default()
         },


        ..default()

 
       
    };

    let mut green_material: StandardMaterial = Color::srgb(0.13, 0.37, 0.11) .into();
    green_material.unlit = true;
    green_material.double_sided = true ;
    //ideally, normals will point UP 


    assets_resource.register_foliage_mesh("grass1", asset_server.load( "foliage/meshes/grass1.obj" ));

    assets_resource.register_foliage_mesh("grass2", asset_server.load( "foliage/meshes/grass2.obj" ));


    assets_resource.register_foliage_material("standard_green",   FoliageMaterialHandle::Standard(   asset_server.add( green_material )  ));
     assets_resource.register_foliage_material("foliage_green",  FoliageMaterialHandle::Extended(   asset_server.add( foliage_material )  ) );


    next_state.set( FoliageAssetsState::Loaded );
}*/



fn spawn_foliage_doodads (

   // mut commands :Commands , 
    foliage_proto_query: Query< (Entity, &FoliageProto ), Added<FoliageProto> >,
     mut event_writer: EventWriter<SpawnDoodadEvent>,

){


    for (foliage_proto_entity, foliage_proto) in foliage_proto_query.iter() {


        let foliage_def = &foliage_proto.foliage_definition;
        let foliage_type_name = &foliage_def.name ; 



        info!("spawn foliage doodad" );
         event_writer.send(SpawnDoodadEvent {
                position: Vec3::default(),
                doodad_name: foliage_type_name.clone(),
                scale: None,
                rotation_euler: None,
                custom_props:None     ,
                  force_parent: Some(foliage_proto_entity),
                  auto_select: false,


                  is_foliage: true, 
                //clay_tile_block_data : None ,
      

            });

    }

}


fn propogate_height_data_change_to_foliage(

      mut commands:  Commands,
    
    foliage_root_query :Query<Entity,With<FoliageRoot>> , 
    foliage_chunk_query: Query< Entity,  With<  FoliageChunk  >   >, 


    terrain_chunk_query: Query< (Entity, &Chunk) >, 

    transform_query: Query< & Transform >,

    chunk_height_maps_resource: Res<ChunkHeightMapResource>,


) {


    let Ok(foliage_root_entity) = foliage_root_query.single() else { return };

     if ! chunk_height_maps_resource.is_changed() {
        return ;
    }



    // delte all foliage chunks and recreate them 


    for (foliage_chunk) in foliage_chunk_query.iter(){


        if let Ok(mut cmd) = commands.get_entity(foliage_chunk){

            cmd.despawn(); 

        }


    }


    for (chunk_entity, chunk) in terrain_chunk_query.iter() {


        let chunk_id = chunk.chunk_id; 

        let Some(chunk_heightmap_data) = chunk_height_maps_resource.chunk_height_maps.get(&chunk_id) else {continue};


        let Some(transform) = transform_query.get(chunk_entity) .ok() else {continue};

        let chunk_translation = transform.translation.clone(); 

        commands.spawn(  (

                Name::new( format!("Foliage Chunk {}", chunk_id) ),
                FoliageChunk { chunk_id:  chunk_id },

                FoliageHeightMapData ( chunk_heightmap_data.to_vec() )  ,

                FoliageDimensionsData ( IVec2 { x: 128, y: 128 } ),
 
                FoliageDataSource ( chunk_entity.clone() ),
                
                Visibility::default(), 

                Transform::from_translation(  chunk_translation  )

            )  ).set_parent( foliage_root_entity );

    }







   /* for foliage_chunk_parent_entity  in foliage_chunk_query.iter(){

            if let Some(mut cmd) = commands.get_entity(foliage_chunk_parent_entity){
                cmd.remove::<FoliageChunkHeightData>();
            }   


    }*/
 

}
 




/*

To rebuild foliage layer , just remove the old foliageBaseHeightMap ? 
*/
/*
fn add_height_maps_to_foliage_layers(  
     mut commands:  Commands,
  
     terrain_chunk_query: Query< (Entity, &Chunk ) , Without< FoliageChunkHeightData  >>,

 

    chunk_height_maps_resource: Res<ChunkHeightMapResource>,
 
    //terrain_data_query: Query< (&TerrainData, &TerrainConfig) > ,
    
    //terrain_loading_state: Res<State<TerrainLoadingState>>

){  





    for (chunk_entity, chunk) in terrain_chunk_query.iter(){


        commands.spawn(


            (

                )

            );
 

    }
*/
    /*

    let Some( (_terrain_data, terrain_config) ) = terrain_data_query.get_single().ok() else {return};


    let terrain_width = terrain_config.terrain_dimensions.x as usize;
    let chunk_rows = terrain_config.chunk_rows as usize;

    for (foliage_layer_entity,foliage_layer) in foliage_layer_query.iter(){

       //  let dimensions = foliage_layer.dimensions.clone();


        let combined_height_map: Vec<Vec<u16>> =  get_combined_heightmap_data(
         &chunk_height_maps_resource.chunk_height_maps,
         terrain_width,
         chunk_rows

          );

        /*if combined_height_map.is_empty() {
            warn!("no chunk height data to provide to foliage system");
            continue
        }; */

        let base_height_comp = FoliageBaseHeightMapU16 (  combined_height_map   );

       
        commands.entity(foliage_layer_entity).try_insert(
            base_height_comp
        ); 

         commands.entity(foliage_layer_entity).try_insert(
            FoliageLayerNeedsRebuild
        ); 


    }
 


}
 
    */

    // chunk_height_maps is a collection of 16 maps, each being 256x256 
    //the output should be one big map, at 1024x1024  
    /*
    fn get_combined_heightmap_data(
       chunk_height_maps: &HashMap<u32, Vec<Vec<u16>> > ,

       terrain_dimensions: usize,
       terrain_chunk_rows: usize, 

     ) -> Vec<Vec<u16>>{
        // Initialize a 1024x1024 heightmap filled with zeros
        let mut combined_heightmap = vec![vec![0u16; terrain_dimensions]; terrain_dimensions];

        let chunk_width = terrain_dimensions / terrain_chunk_rows;

        // Iterate over each chunk in the heightmap
        for (&chunk_index, heightmap) in chunk_height_maps.iter() {
            // Determine the starting x and y positions in the combined map
            let chunk_x = (chunk_index % terrain_chunk_rows as u32) * chunk_width as u32;
            let chunk_y = (chunk_index / terrain_chunk_rows as u32) * chunk_width as u32;

            // Place the 256x256 chunk into the appropriate position in the 1024x1024 map
            for (y, row) in heightmap.iter().enumerate() {
                for (x, &value) in row.iter().enumerate() {
                    combined_heightmap[chunk_y as usize + y][chunk_x as usize + x] = value;
                }
            }
        }

        combined_heightmap

    }
 */

// ------------------





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