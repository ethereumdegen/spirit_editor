
//use crate::materials::extension_material_link::BuildableUsingWorld;
//use crate::materials::shaders::cel_mask_texture::get_cel_mask_texture_embedded_path;
use std::path::Path;
use crate::AssetSourceId;
use crate::AssetPath;
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy_materialize::MaterializeAppExt;

use bevy::pbr::{ExtendedMaterial, MaterialExtension};

/*

 Supports cel shading and  UV fixation


*/

pub fn doodad_material_plugin(app: &mut App) {
    app
        //.init_asset::<CharacterMaterial>()
        .add_plugins(MaterialPlugin::<DoodadMaterial>::default())


      //  .register_generic_material::< DoodadMaterial >()
        .register_extended_generic_material::<StandardMaterial, DoodadMaterialBase >("DoodadMaterial" )
        //.register_generic_material_shorthand::< DoodadMaterial >("DoodadMaterial")



        ;

     //    embedded_asset!(app, omit_prefix, "files/bevy_pixel_light.png");
        embedded_asset!(app, "assets/cel_mask.png");


}

pub type DoodadMaterial = ExtendedMaterial<StandardMaterial, DoodadMaterialBase>;
 


#[derive(Clone, ShaderType, Debug, Reflect)]
pub struct DoodadMaterialUniforms {
    pub tint_color: LinearRgba,

    pub use_fixed_world_uv: u32,    
    pub blank_top_bottom : u32 , 


    pub uv_input_scale: f32 
    //  pub accelerations: Vec4,
}
impl Default for DoodadMaterialUniforms {
    fn default() -> Self {
        Self {
            tint_color: Color::WHITE.into(),

            use_fixed_world_uv: 0, // default
            blank_top_bottom: 0, 


            uv_input_scale: 8.0 
                                     // accelerations: Vec4::default(),
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone   )]
//#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct DoodadMaterialBase {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: DoodadMaterialUniforms,

    #[texture(100)]
    #[sampler(101)]
    pub mask: Handle<Image>,
    /// The parts of the model that are facing the light source and are not in shadow.
    #[uniform(102)]
    pub highlight_color: LinearRgba,
    /// The parts of the model that are not facing the light source and are in shadow.
    #[uniform(103)]
    pub shadow_color: LinearRgba,
    /// The color of the edge of the model, which gets a slight specular highlight to make the model pop.
    #[uniform(104)]
    pub rim_color: LinearRgba,
}

/*
impl Default for DoodadMaterialBase {




    fn default() -> Self { 

        Self{ 
            custom_uniforms: DoodadMaterialUniforms::default(),
            mask: Handle::default(),  //this gets fixed in a system s

            highlight_color : Srgba::hex("ADBBB7").unwrap().into(), 
             shadow_color : Srgba::hex("8E978D").unwrap().into(),
             rim_color: Srgba::hex("EEEEEE").unwrap().into() 


        }

     }
}*/
 

pub fn get_cel_mask_texture_embedded_path() -> &'static str {
   // "embedded://spirit_editor/shaders/assets/cel_mask.png"

   "shaders/cel_mask.png"   

      // "embedded://spirit_editor/assets/cel_mask.png"
}


impl DoodadMaterialBase {

    pub fn build_mask_from_world (&mut self, world: &World ){

         let mask_image = world.load_asset(get_cel_mask_texture_embedded_path());
         self.mask = mask_image; 




    }

}

 
impl FromWorld for DoodadMaterialBase {



        fn from_world(world: &mut  World) -> Self {  

               let mask_image = world.load_asset(get_cel_mask_texture_embedded_path());


                DoodadMaterialBase::build(mask_image)


                /*
    
                       Self{ 
                            custom_uniforms: DoodadMaterialUniforms::default(),
                            mask: Handle::default(),  //this gets fixed in a system s

                            highlight_color : Srgba::hex("ADBBB7").unwrap().into(), 
                             shadow_color : Srgba::hex("8E978D").unwrap().into(),
                             rim_color: Srgba::hex("EEEEEE").unwrap().into() 


                        }
                */
         }
}  
 
impl DoodadMaterialBase {
    fn build(mask_image: Handle<Image>) -> Self {
        let highlight_color = Srgba::hex("ADBBB7").unwrap();
        let shadow_color = Srgba::hex("8E978D").unwrap();
        let rim_color = Srgba::hex("EEEEEE").unwrap();

        Self {
            custom_uniforms: DoodadMaterialUniforms::default(),
            mask: mask_image,
            highlight_color: highlight_color.into(),
            shadow_color: shadow_color.into(),
            rim_color: rim_color.into(),
        }
    }
} 

impl DoodadMaterialBase {
    pub fn set_tint_alpha(&mut self, alpha: f32) {
        self.custom_uniforms.tint_color.alpha = alpha;
    }

    pub fn set_tint_rgb(&mut self, rgb: LinearRgba) {
        self.custom_uniforms.tint_color.red = rgb.red;
        self.custom_uniforms.tint_color.green = rgb.green;
        self.custom_uniforms.tint_color.blue = rgb.blue;
    }
}

impl MaterialExtension for DoodadMaterialBase {
    fn fragment_shader() -> ShaderRef {
        // CHARACTER_MATERIAL_SHADER_HANDLE.into()
        "shaders/doodad_cel.wgsl".into()
    }
}

/*
impl BuildableUsingWorld for DoodadMaterialBase {
    fn build_with_world(world: &mut World) -> Self {
        let mask_image = world.load_asset(get_cel_mask_texture_embedded_path());

        DoodadMaterialBase::build(mask_image)
    }
}
*/