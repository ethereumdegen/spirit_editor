use bevy_materialize::MaterializeAppExt;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension};

 

pub fn fixed_space_uv_material_plugin(app: &mut App) {
    app
        //.init_asset::<CharacterMaterial>()
        .add_plugins(MaterialPlugin::<
            //NEED THIS
            FixedSpaceUvMaterial,
        >::default())

           .add_plugins(MaterialPlugin::<
            //NEED THIS
         ExtendedMaterial<StandardMaterial, FixedSpaceUvMaterialSidesOnly>    ,
        >::default())


            .register_generic_material::< FixedSpaceUvMaterial >()
             .register_generic_material_shorthand::< FixedSpaceUvMaterial >("FixedSpaceUvMaterial")

            


        ;

   
}

pub type FixedSpaceUvMaterial = ExtendedMaterial<StandardMaterial, FixedSpaceUvMaterialBase>;
 
#[derive(Clone, ShaderType, Debug, Reflect)]
pub struct FixedSpaceUvMaterialUniforms {
    pub tint_color: LinearRgba,
    pub blank_top_bottom: u32,  //bool !! 

    pub uv_input_scale : f32 

   
}
impl Default for FixedSpaceUvMaterialUniforms {
    fn default() -> Self {


         

        
        Self {
            tint_color: Color::WHITE.into(),
            blank_top_bottom: 0, 
            uv_input_scale: 8.0 //default 
             
        }
    }
}

//#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default )]
pub struct FixedSpaceUvMaterialBase {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: FixedSpaceUvMaterialUniforms,



}

/*
impl FixedSpaceUvMaterialBase {
    pub fn set_tint_alpha(&mut self, alpha: f32) {
        self.custom_uniforms.tint_color.alpha = alpha;
    }

    pub fn set_tint_rgb(&mut self, rgb: LinearRgba) {
        self.custom_uniforms.tint_color.red = rgb.red;
        self.custom_uniforms.tint_color.green = rgb.green;
        self.custom_uniforms.tint_color.blue = rgb.blue;  
    }
}*/

impl MaterialExtension for FixedSpaceUvMaterialBase {
    fn fragment_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/fixed_space_uv.wgsl".into()
    }
 



}

// ---- 




#[derive(Asset, AsBindGroup, TypePath, Debug, Clone )]
pub struct FixedSpaceUvMaterialSidesOnly {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: FixedSpaceUvMaterialUniforms,
 
}

impl Default for FixedSpaceUvMaterialSidesOnly {



        fn default() -> Self { 


            
            Self {

                custom_uniforms: FixedSpaceUvMaterialUniforms {

                   
                    ..default() 
                }
            }

         }
}



impl MaterialExtension for FixedSpaceUvMaterialSidesOnly {
    fn fragment_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/fixed_space_uv.wgsl".into()
    }
}





// ----- 

/*
// Define an enum for the bit positions
#[repr(u32)]
#[derive(Clone,Copy)]
enum FixedSpaceConfigBits {
    BlankTopBottom = 0,       // Bit 0
    //AnimateMaskingTexture = 1,   // Bit 1
    // Add more bits as needed
}

// A helper function to construct the bitfield
fn build_config_bits(flags: &[(FixedSpaceConfigBits, bool)]) -> u32 {
    let mut config_bits = 0;

    for (bit, enabled) in flags {
        if *enabled {
            config_bits |= 1 << *bit as u32;
        }
    }

    config_bits
}*/