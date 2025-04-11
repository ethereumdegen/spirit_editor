use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterial;
use bevy_materialize::GenericMaterialError;
use bevy::{math::Affine2, prelude::*};
use bevy_materialize::prelude::*;

use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterialBase;


/*

sometimes this doesnt always work !? 
*/
pub fn materials_plugin(app:&mut App){


          app

          .register_type::<TextureSubsetDimensions>()   // critical ! 
            //.add_systems(Startup, register_foliage_assets)
            .add_systems(Update, (
                
             	update_materialize_properties

                ).chain()
               
           )

            ;
}

 

pub trait CustomMaterialPropertiesExt {
    const TEXTURE_SUBSET_DIMENSIONS: MaterialProperty<TextureSubsetDimensions> = MaterialProperty::new("texture_subset_dimensions", || TextureSubsetDimensions::default());


    const UV_SCALE_FACTOR: MaterialProperty<f32> = MaterialProperty::new("uv_scale_Factor", || 1.0 );
}
impl CustomMaterialPropertiesExt for GenericMaterial {}





#[derive(Clone,Debug,Reflect,Default )]
pub struct TextureSubsetDimensions {

    pub offset: IVec2 ,
    pub dimensions: IVec2,
    pub base_texture_dimensions: IVec2 
 

}

impl TextureSubsetDimensions {

    pub fn to_affine(&self , uv_scale: f32 ) ->  Affine2 {


                 let base_texture_dimensions = self. base_texture_dimensions;

                let scale = Vec2::new(  self.dimensions .x as f32  / base_texture_dimensions.x as f32   ,  
                 self.dimensions .y as f32  / base_texture_dimensions.y as f32  )  * Vec2::splat(uv_scale) ;

                Affine2 {
                    matrix2: Mat2::from_diagonal(scale),

                    translation: Vec2::new(  self.offset .x as f32  / base_texture_dimensions.x as f32  ,  
                     self.offset .y as f32  / base_texture_dimensions.y as f32   ) 
                }

    }

}



/*

Read custom properties from our materialize toml file and apply them 


Performs post processing on our  materialize materials !! this is critical due to how we are using spritesheet textures 

*/
fn update_materialize_properties(

    mut asset_load_events: EventReader< AssetEvent< GenericMaterial > >  ,
  
     generic_materials_ext: GenericMaterials, 

     mut standard_materials : ResMut<Assets<StandardMaterial>> ,
      mut fixed_uv_materials : ResMut<Assets< FixedSpaceUvMaterial >> 

 ) {

 
    for evt in asset_load_events.read() {

        match evt {

            AssetEvent::LoadedWithDependencies { id } => {
                 //  println!( "LoadedWithDependencies GenericMaterial 1 " );


                let Some(loaded_generic_material) =  generic_materials_ext.get( *id ) else {continue};
    
                     //  println!( "LoadedWithDependencies GenericMaterial 2" );

                    let uv_scale_factor  = loaded_generic_material.get_property(GenericMaterial::UV_SCALE_FACTOR) .unwrap_or( 1.0 ) ;

                    let tex_subset_dimensions: Result<TextureSubsetDimensions, GenericMaterialError> = loaded_generic_material
                                    .get_property(GenericMaterial::TEXTURE_SUBSET_DIMENSIONS);
    

                     let uv_affine_xform = match    tex_subset_dimensions.ok() {

                        Some( tex_subset_dimensions ) =>  tex_subset_dimensions.to_affine( uv_scale_factor  ), 
                        None => Affine2::from_scale(Vec2::splat(uv_scale_factor))  

                     };
 

                     let material = &loaded_generic_material.material;
                      
                    if let Some(  mat  ) = standard_materials.get_mut(  material.handle.id() .typed_unchecked()) {
                        println!("Successfully updated GenericMaterial uv_transform {:?}" , uv_affine_xform);
                        mat.uv_transform = uv_affine_xform;
                    } 


                    if let Some(  mat  ) = fixed_uv_materials.get_mut(  material.handle.id() .typed_unchecked()) {
                        println!("Successfully updated GenericMaterial uv_transform {:?}" , uv_affine_xform);
                        mat.base.uv_transform = uv_affine_xform;
                    } 
 
            },
            _ =>  {} 

        }

    }


     // You can also do materials.get(<asset id>) to get a view.
  

}