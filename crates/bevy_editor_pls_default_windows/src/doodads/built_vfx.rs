use bevy::prelude::*;

use bevy::utils::HashMap;
use bevy_magic_fx::magic_fx_variant::MagicFxVariant;


#[derive(Resource, Default)]
 pub  struct BuiltVfxResource {


  pub	magic_fx_variants: HashMap<String, MagicFxVariant>  	

}
