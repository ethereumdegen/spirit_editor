use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension};

/*  pub(crate) const CHARACTER_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(7_529_326_912_151_597_124);

*/

pub fn magic_rock_material_plugin(app: &mut App) {
    app
        //.init_asset::<CharacterMaterial>()
        .add_plugins(MaterialPlugin::<
            //NEED THIS
            MagicRockMaterial,
        >::default());

    /* load_internal_asset!(
        app,
        CHARACTER_MATERIAL_SHADER_HANDLE,
        "shaders/character.wgsl",
        Shader::from_wgsl
    );
    */
}

pub type MagicRockMaterial = ExtendedMaterial<StandardMaterial, MagicRockMaterialBase>;

pub fn build_magic_rock_material(original_material: StandardMaterial) -> MagicRockMaterial {
    ExtendedMaterial {
        base: original_material, //from blender
        extension: MagicRockMaterialBase::default(),
    }
}

//pub type AnimatedMaterialExtension = ExtendedMaterial<StandardMaterial, AnimatedMaterial>;
//pub type CharacterMaterialBundle = MaterialMeshBundle<CharacterMaterial >;

#[derive(Clone, ShaderType, Debug)]
pub struct MagicRockMaterialUniforms {
    pub tint_color: LinearRgba,

   
}
impl Default for MagicRockMaterialUniforms {
    fn default() -> Self {

        
        Self {
            tint_color: Color::WHITE.into(),
             
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct MagicRockMaterialBase {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: MagicRockMaterialUniforms,



}

impl MagicRockMaterialBase {
    pub fn set_tint_alpha(&mut self, alpha: f32) {
        self.custom_uniforms.tint_color.alpha = alpha;
    }

    pub fn set_tint_rgb(&mut self, rgb: LinearRgba) {
        self.custom_uniforms.tint_color.red = rgb.red;
        self.custom_uniforms.tint_color.green = rgb.green;
        self.custom_uniforms.tint_color.blue = rgb.blue;  
    }
}

impl MaterialExtension for MagicRockMaterialBase {
    fn fragment_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/rock_magic.wgsl".into()
    }

  /*   fn prepass_fragment_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/character.wgsl".into()
    }*/

   /* fn deferred_fragment_shader() -> ShaderRef {
        //  CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/character.wgsl".into()
    }*/


/* fn vertex_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/character_2.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/character_2.wgsl".into()
    }*/



}
