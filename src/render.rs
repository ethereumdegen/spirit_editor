use bevy::prelude::*;

use bevy::core_pipeline::core_3d::graph::Node3d ;

use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::render::view::NoFrustumCulling;

use bevy::utils::HashSet;

use bevy::scene::SceneInstanceReady;

 


pub(crate) fn rendering_plugin(app: &mut App) {
    app.register_type::<SceneBundleLink>()
        .register_type::<HeadModelSceneLink>()

      
        .add_observer(listen_for_scene_loaded)
        .add_systems(
            Update,
            (
                apply_cascaded_no_frustrum_culling,
                apply_cascaded_not_shadow_caster,
                apply_cascaded_not_shadow_receiver,
            )
                .chain(),
        );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SceneBundleLink(pub Entity); //for normal model, third person model..

#[derive(Component, Reflect)] // special !! only used for player to link their first person model.
#[reflect(Component)]
pub struct HeadModelSceneLink(pub HashSet<Entity>); //points at the face and CasualHead

#[derive(Component)]
struct SceneInstanceLoaded;

pub fn listen_for_scene_loaded(trigger: Trigger<SceneInstanceReady>, mut commands: Commands) {
    let parent = trigger.entity();

    commands.entity(parent).insert(SceneInstanceLoaded);
}

// attach this to the same entity that you are attaching a GLTF scene to and that gltf scene will NOT frustum cull
#[derive(Component)]
pub struct CascadedNoFrustumCulling;

fn apply_cascaded_no_frustrum_culling(
    mut commands: Commands,

    scene_instance_loaded_query: Query<Entity, Added<SceneInstanceLoaded>>,

    cascaded_query: Query<Entity, With<CascadedNoFrustumCulling>>,

    children_query: Query<&Children>,
    //parent_query: Query<&Parent>,
) {
    for entity in scene_instance_loaded_query.iter() {
        if cascaded_query.get(entity).ok().is_some() {
            for child in DescendantIter::new(&children_query, entity) {
                if let Some(mut cmd) = commands.get_entity(child) {
                    cmd.insert(NoFrustumCulling);
                }
            }
        }
    }
}

// attach this to the same entity that you are attaching a GLTF scene to and that gltf scene will NOT frustum cull
#[derive(Component)]
pub struct CascadedNotShadowCaster;

fn apply_cascaded_not_shadow_caster(
    mut commands: Commands,

    scene_instance_loaded_query: Query<Entity, Added<SceneInstanceLoaded>>,

    cascaded_query: Query<Entity, With<CascadedNotShadowCaster>>,

    children_query: Query<&Children>,
) {
    for entity in scene_instance_loaded_query.iter() {
        if cascaded_query.get(entity).ok().is_some() {
            for child in DescendantIter::new(&children_query, entity) {
                if let Some(mut cmd) = commands.get_entity(child) {
                    cmd.insert(NotShadowCaster);
                }
            }
        }
    }
}

// attach this to the same entity that you are attaching a GLTF scene to and that gltf scene will NOT frustum cull
#[derive(Component)]
pub struct CascadedNotShadowReceiver;

fn apply_cascaded_not_shadow_receiver(
    mut commands: Commands,

    scene_instance_loaded_query: Query<Entity, Added<SceneInstanceLoaded>>,

    cascaded_query: Query<Entity, With<CascadedNotShadowReceiver>>,

    children_query: Query<&Children>,
) {
    for entity in scene_instance_loaded_query.iter() {
        if cascaded_query.get(entity).ok().is_some() {
            for child in DescendantIter::new(&children_query, entity) {
                if let Some(mut cmd) = commands.get_entity(child) {
                    cmd.insert(NotShadowReceiver);
                }
            }
        }
    }
}
