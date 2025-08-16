
use crate::post_processing::rendering::GraphicsRenderLayer;
use bevy_materialize::generic_material::{GenericMaterial, GenericMaterialApplied };
// use bevy_materialize::generic_material::GenericMaterialError;
use bevy::{ prelude::*};
use bevy_materialize::prelude::*;

use bevy::render::view::RenderLayers; 

//use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterialBase;
//use crate::shaders::fixed_space_uv_material::FixedSpaceUvMaterial;

use crate::shaders::doodad_material::{DoodadMaterial,DoodadMaterialBase};

 
pub fn materialize_properties_plugin(app:&mut App){


          app

        
          .register_type::< GraphicsRenderLayer >()
          
            .register_material_property(  GenericMaterial::RENDER_LAYER_PROPERTY )

            
          .add_systems(PostUpdate, (
                
         
                update_render_layer_from_material_property, 

                ).chain()
               
           )   

            ;
}


pub trait CustomMaterialPropertiesExt {
    const RENDER_LAYER_PROPERTY: MaterialProperty<GraphicsRenderLayer> = MaterialProperty::new("render_layer");
}
impl CustomMaterialPropertiesExt for GenericMaterial {}




 
pub fn update_render_layer_from_material_property(
    mut query: Query<(Entity, &GenericMaterial3d ), Changed<GenericMaterialApplied>>,
    generic_materials: Res<Assets<GenericMaterial>>,
    mut commands: Commands,
) {
 

    for (material_entity, generic_material_holder ) in &mut query {
        let Some(generic_material) = generic_materials.get(&generic_material_holder.0) else { continue };
        let Ok(render_layer_property) = generic_material.get_property(GenericMaterial::RENDER_LAYER_PROPERTY) else { continue };

         if let Ok(mut cmd) = commands.get_entity(material_entity){

            info!( "insert custom render layer ! " );
            cmd.insert  ( 

                RenderLayers::layer(  render_layer_property.clone() .into()

            ) ) ;
         }
    }
}



// ----

 


 
 /*

pub trait CustomMaterialPropertiesExt {
   // const TEXTURE_SUBSET_DIMENSIONS: MaterialProperty<TextureSubsetDimensions> = MaterialProperty::new("texture_subset_dimensions");


    const UV_SCALE_FACTOR: MaterialProperty<f32> = MaterialProperty::new("uv_scale_factor" );  //default is 1.0 ? 
}
impl CustomMaterialPropertiesExt for GenericMaterial {}

*/


/*
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

}*/



/*

Read custom properties from our materialize toml file and apply them 


Performs post processing on our  materialize materials !! this is critical due to how we are using spritesheet textures 


      this is buggy and weird  pretty often.. ? 

*/


/*
fn update_materialize_properties_when_applied(


    material_entity: Query< (Entity, &GenericMaterial3d ),  Or<( Changed<GenericMaterialApplied> , Changed<MeshMaterial3d<DoodadMaterial>> )> >,

     generic_material_assets: Res<Assets<GenericMaterial>>  , 



     mut standard_materials : ResMut<Assets< StandardMaterial >> ,
   //   mut fixed_uv_materials : ResMut<Assets< FixedSpaceUvMaterial >> ,  
       mut doodad_materials : ResMut<Assets< DoodadMaterial >> 


   //  mut commands: Commands 

    
){
    for (entity, generic_material_3d) in material_entity.iter(){


        let asset_id = generic_material_3d.id() ;




         let Some(loaded_generic_material) =  generic_material_assets.get(   asset_id ) else {continue};



                  let uv_scale_factor  = loaded_generic_material.get_property(GenericMaterial::UV_SCALE_FACTOR) .unwrap_or( &1.0 ) ;

                   // let tex_subset_dimensions: Result<TextureSubsetDimensions, GetPropertyError> = loaded_generic_material
                         //           .get_property(GenericMaterial::TEXTURE_SUBSET_DIMENSIONS).cloned();
    

                     let uv_affine_xform = match    tex_subset_dimensions.ok() {

                        Some( tex_subset_dimensions ) =>  tex_subset_dimensions.to_affine( *uv_scale_factor  ), 
                        None => Affine2::from_scale(Vec2::splat(*uv_scale_factor))  

                     };
 


                     let material_handle = &loaded_generic_material.handle;

                    

                    if let Some(  mat  ) = standard_materials.get_mut(  material_handle.id() .typed_unchecked()) {
                        println!("Successfully updated GenericMaterial uv_transform {:?}" , uv_affine_xform);
                        mat.uv_transform = uv_affine_xform;
                    } 


                  /*  if let Some(  mat  ) = fixed_uv_materials.get_mut(  material.handle.id() .typed_unchecked()) {
                        println!("Successfully updated GenericMaterial uv_transform {:?}" , uv_affine_xform);
                        mat.base.uv_transform = uv_affine_xform;
                    } */


                    if let Some(  mat  ) = doodad_materials.get_mut(  material_handle.id() .typed_unchecked()) {
                        println!("Successfully updated GenericMaterial uv_transform {:?}" , uv_affine_xform);
                        mat.base.uv_transform = uv_affine_xform;
                    } 


    }




 }*/


// a hack to add the mask image 
// can this be done a different way ?? 

/*
fn update_doodad_material(
       material_query: Query<   &GenericMaterial3d , Or<( Added<GenericMaterial3d> , Changed<GenericMaterial3d> )>>,

      generic_material_assets: Res<Assets<GenericMaterial>>  , 
 

     mut commands: Commands,  


      doodad_materials : Res <Assets< DoodadMaterial >> ,


      


){


     for  generic_material_3d in material_query.iter(){


                let asset_id = generic_material_3d.id() ;


                 let Some(loaded_generic_material) =  generic_material_assets.get(  asset_id ) else {continue};
  
                     let material_handle = &loaded_generic_material.handle;
  

                    if let Some(  _mat  ) = doodad_materials.get ( material_handle.id() .typed_unchecked()) {
                        

                        commands.queue( BuildMaterialMask( material_handle.id().typed_unchecked() )  );
                      

                    } 


    }



}

struct BuildMaterialMask( AssetId<DoodadMaterial> ) ;


impl Command for BuildMaterialMask {


fn apply(self, world: &mut  World) { 

   

        let Some(mut doodad_materials ) = world.remove_resource::<Assets<DoodadMaterial>>() else {return};

    

          if let Some(  mat  ) = doodad_materials.get_mut (  self. 0 ) {


               mat.extension.build_mask_from_world(  world  )


         } 

         world.insert_resource(   doodad_materials  );
     }
}*/