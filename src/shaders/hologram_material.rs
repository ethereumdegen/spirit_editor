use bevy_materialize::MaterializeAppExt; 
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension};
 

pub fn hologram_material_plugin(app: &mut App) {
    app
       
        .add_plugins(MaterialPlugin::<HologramMaterial>::default())
        .register_generic_material::<HologramMaterial>()
        .register_generic_material_shorthand::<HologramMaterial>("HologramMaterial");

     
}

pub type HologramMaterial = ExtendedMaterial<StandardMaterial, HologramMaterialBase>;

#[derive(Clone, ShaderType, Debug, Reflect)]
pub struct HologramMaterialUniforms {
  

    pub distortion_speed: Vec2, 
    pub scroll_repeats: Vec2, 
    pub scroll_speed: Vec2,
    pub distortion_amount: f32,
    pub distortion_cutoff: f32,
    pub depth_cutoff_offset: f32,
    pub animation_frame_dimension: Vec2, //if this is 1, we know we have a normal static texture.  Otherwise, we have an animated scrolly one 
    pub current_animation_frame_index: u32,

    pub uv_scale_factor: Vec2, 

    pub tint_color: LinearRgba ,
    pub fresnel_power: f32 ,

    pub use_masking_texture: u32,   //bool
    pub animate_masking_texture: u32 ,  //bool


}
impl Default for HologramMaterialUniforms {
    fn default() -> Self {
        Self {
             scroll_speed: Vec2::new(0.1,1.0),
             distortion_speed:Vec2::new(3.0,1.0),
             scroll_repeats:Vec2::new(12.0,3.0),

            
            distortion_amount: 0.03,
            distortion_cutoff: 1.0,

            uv_scale_factor: Vec2::new(1.0,1.0),
            
            depth_cutoff_offset: 0.0,
            animation_frame_dimension: Vec2::new(1.0,1.0),
         
            current_animation_frame_index: 0, 
            tint_color: Color::WHITE.into(),
            fresnel_power:  0.0 , //typically like 2.0 if used 

            use_masking_texture: 0,
            animate_masking_texture: 0,
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default )]
//#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct HologramMaterialBase {
   
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: HologramMaterialUniforms,

    #[texture(21)]
    #[sampler(22)]
    pub base_color_texture: Option<Handle<Image>>,

    // Adding masking texture
    #[texture(23)]
    #[sampler(24)]
    pub masking_texture: Option<Handle<Image>>,

}

/*
impl Default for HologramMaterialBase {
    fn default() -> Self {
        Self {
            custom_uniforms: HologramMaterialUniforms::default(),
            mask: Handle::default(), //this gets fixed in a system s

            highlight_color: Srgba::hex("ADBBB7").unwrap().into(),
            shadow_color: Srgba::hex("8E978D").unwrap().into(),
            rim_color: Srgba::hex("EEEEEE").unwrap().into(),
        }
    }
} */
 
 

impl MaterialExtension for HologramMaterialBase {
    fn fragment_shader() -> ShaderRef {
       
        "shaders/hologram.wgsl".into()
    }
}
 