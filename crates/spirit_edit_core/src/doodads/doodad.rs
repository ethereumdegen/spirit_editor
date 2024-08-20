use bevy::{prelude::*, utils::HashMap};

use super::{  doodad_manifest::{DoodadDefinition, RenderableType}};
 

//use bevy_mod_sysfail::*;

use bevy_mod_picking::prelude::*;
 


//use bevy_mod_picking::prelude::{PickRaycastTarget, PickableBundle};

use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    scene::SceneInstanceReady,
};


/*
#[derive(Default)]
pub(crate) struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedGltfAssets::default())
            .add_systems(Update, attach_models_to_doodads);
    }
}*/


/*
#[derive(Resource, Default)]
pub struct LoadedGltfAssets {
    pub gltf_models: HashMap<String, Handle<Gltf>>,
}

*/



#[derive(Component, Debug)]
pub struct DoodadComponent {
    pub definition: DoodadDefinition,
}

impl DoodadComponent {
    pub fn from_definition(definition: &DoodadDefinition) -> Self {
        Self {
            definition: definition.clone(),
        }
    }
}
