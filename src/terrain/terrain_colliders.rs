use std::io::Read;
use degen_toon_terrain::terrain::TerrainData;
use crate::doodads::doodad_colliders::CollisionLayer;
use degen_toon_terrain::chunk::Chunk;
use degen_toon_terrain::chunk::ChunkData;
use degen_toon_terrain::terrain_config::TerrainConfig;
use avian3d::prelude::*;
 
use bevy::prelude::*; 

pub fn terrain_colliders_plugin(app: &mut App){

/*
	app
		 
	 	  .add_systems(
            Update,
            (
                add_collider_children_to_chunks,
                load_colliders_for_terrain,
             
            )
                .chain()
                 
        )
	;


*/
}


#[derive(Component, Default)]
struct ChunkColliderMarker;

#[derive(Component, Default)]
struct ChunkNoCollision;

#[derive(Component)]
struct HasColliderChild;



fn add_collider_children_to_chunks(
    mut commands: Commands,

    chunk_query: Query<
        (Entity, &Chunk, &ChunkData),
        (Without<HasColliderChild>, Without<ChunkNoCollision>),
    >,
    //terrain_config_query: Query<(&TerrainConfig, &TerrainData)>,
) {
    for (chunk_entity, chunk, chunk_data) in chunk_query.iter() {
        let collider_child = commands
            .spawn((Transform::default(), Visibility::Hidden))
            .insert(Name::new("chunk collider marker"))
            .insert(ChunkColliderMarker)
            .id();

        let chunk_lod = chunk_data.get_lod_level();

        if chunk_lod >= 2 {
            info!("skip chunk collision generation for lod - too low ");

            if let Ok(mut cmd) = commands.get_entity(chunk_entity) {
                cmd.insert(ChunkNoCollision).add_child(collider_child);
            }

            continue;
        }

        if let Ok(mut cmd) = commands.get_entity(chunk_entity) {
            cmd.insert(HasColliderChild).add_child(collider_child);
        }
    }
}

// could make this more efficient !?  collision doesnt LOD ?
fn load_colliders_for_terrain(
    mut commands: Commands,

    chunk_query: Query<&Chunk>,
    // chunk_lod_query: Query<(&ChunkData)>,
    terrain_config_query: Query<(&TerrainConfig, &TerrainData)>,

    terrain_collider_query: Query<
        (Entity, &ChildOf),
        (With<ChunkColliderMarker>, Without<Collider>),
    >,

    //mut terrain_load_timer_resource: ResMut<TerrainLoadingTimerResource>,
) {
    let Some((terrain_config, terrain_data)) = terrain_config_query.single().ok() else {
        return;
    };

    for (entity, parent_chunk) in terrain_collider_query.iter() {
        // let _chunk_world_location = terrain_mesh_transform.translation();

        let chunk = chunk_query.get(parent_chunk.parent()).unwrap();

        let chunk_id = chunk.chunk_id;

        info!("loading terrain collider  ");

        //  terrain_collider_loading_resource.0.insert( entity.clone() ) ;

        let collision_folder_path = terrain_config
            .collider_data_folder_path
            .as_path()
            .to_str()
            .unwrap();

        let file_path = format!("assets/{}/{}.col", collision_folder_path, chunk_id);

        if let Ok(mut cmd) = commands.get_entity(entity) {
            cmd.remove::<ChunkColliderMarker>();
        }

        // Read the entire file into a string
        if let Some(mut file) = std:: fs::File::open(file_path).ok() {
            let mut contents = Vec::new();
            let read_successful = file.read_to_end(&mut contents);

            // Deserialize the JSON string back into a struct
            if let Some(collider) = bincode::deserialize::<Collider>(&contents).ok() {
                commands.entity(entity).insert(collider).insert((
                    RigidBody::Static,
                    Restitution::new(0.0001).with_combine_rule(CoefficientCombine::Multiply),
                    CollisionLayers::new(
                        [CollisionLayer::Terrain, CollisionLayer::CameraObstacle],
                        [CollisionLayer::Character],
                    ),
                ));
                info!("added terrain collider to {:?}", entity);

                //terrain_load_timer_resource.terrain_loading_timer.reset();
            }
        }
        // let collider: Collider = bincode::deserialize(&contents).unwrap();

        //   commands.entity(entity).insert((NotShadowCaster,NotShadowReceiver));  //this doesnt rly help
    }
}
