

use serde::Serialize;
use serde::Deserialize;
use bevy::prelude::*;


#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect, PartialEq, Eq )]
pub enum GraphicsRenderLayer {
    #[default]
    Standard3d = 0,
  //UserInterface = 1, // this isnt really true ? 
     PostProcessingEffect = 2, 

}

impl From<GraphicsRenderLayer> for usize {
    fn from(layer: GraphicsRenderLayer) -> Self {
        layer as usize
    }
}

