

 
//use crate::physics::CollisionLayer;
//use crate::level_instantiation::spawning::doodad::DoodadMeshMarker;
//use crate::level_instantiation::spawning::doodad::DoodadSceneReady;



use serde::Serialize;
use serde::Deserialize;
use avian3d::prelude::PhysicsLayer;
use avian3d::prelude::{CoefficientCombine, Restitution};
use bevy::render::mesh::VertexAttributeValues;

use bevy::{prelude::*, scene::SceneInstanceReady};

use bevy::ecs::relationship::DescendantIter;

use avian3d::prelude::{Collider,   CollisionLayers, RigidBody};

pub(crate) fn doodad_colliders_plugin(app: &mut App) {
    app 
         .add_event::<ConvertAllChildMeshesToColliders>()
        .add_event::<ConvertMeshToColliders>()
        //.add_observer(add_doodad_collider_markers)
        .add_observer(handle_convert_all_child_meshes_to_colliders)
        .add_observer(handle_convert_mesh_to_collider)

        
        ;
}



#[derive(Event, Clone)]
pub struct ConvertAllChildMeshesToColliders {
    pub trimesh: bool,
}

#[derive(Event, Clone)]
pub struct ConvertMeshToColliders {
   pub  trimesh: bool,
}

/*
pub(crate) fn add_doodad_collider_markers(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    doodad_query: Query<Entity, (Without<DoodadSceneReady>, With<DoodadMeshMarker>)>,
    //  mut scene_instance_evt_reader: EventReader<SceneInstanceReady>,
) {
    //for evt in scene_instance_evt_reader.read() {
    //let parent = evt.parent;
    let parent = trigger.target();

    if let Some(new_doodad_entity) = doodad_query.get(parent).ok() {
        commands
            .entity(new_doodad_entity)
            .insert(DoodadSceneReady::default());
    }
    // }
}*/


fn handle_convert_all_child_meshes_to_colliders(
    trigger: Trigger<ConvertAllChildMeshesToColliders>,

    children_query: Query<&Children>,

    transform_query: Query<(Entity, &Transform)>,
    mesh_query: Query<&Mesh3d>,

    meshes: Res<Assets<Mesh>>,

    mut commands: Commands,
) {
    let root_entity = trigger.target();

    let trimesh = &trigger.trimesh;

    for child in DescendantIter::new(&children_query, root_entity) {
        let Some(mesh_handle) = mesh_query.get(child).ok() else {
            continue;
        };

        commands.trigger_targets(ConvertMeshToColliders { trimesh: *trimesh }, child);
    }
}



fn get_mesh_positions(mesh: &Mesh) -> Vec<Vec3> {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        positions
            .iter()
            .map(|&[x, y, z]| Vec3::new(x, y, z))
            .collect()
    } else {
        Vec::new()
    }
}


fn handle_convert_mesh_to_collider(
    trigger: Trigger<ConvertMeshToColliders>,

    transform_query: Query<&Transform>,

    mesh_query: Query<&Mesh3d>,

    meshes: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mesh_entity = trigger.target();

    let trimesh = &trigger.trimesh;

    let Some(mesh_xform) = transform_query.get(mesh_entity).ok() else {
        return;
    };

    let Some(mesh_handle) = mesh_query.get(mesh_entity).ok() else {
        return;
    };

    let Some(mesh) = meshes.get(mesh_handle) else {
        return;
    };

    if !trimesh {
        let unscaled_mesh = &mesh.clone();

        //could be inefficient ?
        let Some(convex_collider) = Collider::convex_hull_from_mesh(unscaled_mesh).or({
            // dont know why i have to do this.. but OK lol
            let mesh_points = get_mesh_positions(mesh);

            Collider::convex_hull(mesh_points)
        }) else {
            warn!("could not gen convex hull ");
            return;
        };

        commands.entity(mesh_entity).insert((
            convex_collider,
            RigidBody::Static,
            Restitution::new(0.001).with_combine_rule(CoefficientCombine::Multiply),
            CollisionLayers::new(
                //remove terrain layer  ??
                [
                    CollisionLayer::Terrain,
                    CollisionLayer::Doodad,
                    CollisionLayer::CameraObstacle,
                    CollisionLayer::Interact, //causes lag? not too bad. w player interact sensor ?
                ],
                [CollisionLayer::Character, CollisionLayer::Sensor],
            ),
            Visibility::Hidden,
        ));

        // info!("auto-adding collider to doodad   ",);
    } else {
        // if trimesh ...

        info!("auto-adding trimesh collider to doodad   ",);

        let doodad_mesh_global_xform = GlobalTransform::default(); // for now ...

        let (scale_of_parent_doodad_mesh, _rotation_of_doodad_mesh, _) =
            doodad_mesh_global_xform.to_scale_rotation_translation();

        //first rotate, then scale
        let scaled_mesh = &mesh.clone().scaled_by(scale_of_parent_doodad_mesh.into());

        let Some(trimesh_collider) = Collider::trimesh_from_mesh(scaled_mesh) else {
            return;
        };

        commands.entity(mesh_entity).insert((
            trimesh_collider,
            RigidBody::Static,
            Restitution::new(0.001).with_combine_rule(CoefficientCombine::Multiply),
            CollisionLayers::new(
                //remove terrain layer  ??
                [
                    CollisionLayer::Terrain,
                    CollisionLayer::Doodad,
                    CollisionLayer::CameraObstacle,
                    CollisionLayer::Interact,
                ],
                [CollisionLayer::Character, CollisionLayer::Sensor],
                //   Visibility::Hidden
            ),
        ));
    }
}



// in here for now 

#[derive(
    PhysicsLayer, Eq, Hash, PartialEq, Clone, Debug, Default, Serialize, Deserialize, Reflect,
)]
pub enum CollisionLayer {
    Player,
    Character,
    #[default]
    Doodad,
    Terrain,
    CameraObstacle,
    Sensor,
    Interact,
    PhysicsVolume,
}
