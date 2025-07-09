
use bevy::platform::collections::hash_map::HashMap; 
use degen_toon_water::DegenToonWaterPlugin;
 
use degen_toon_water::toonwater_material::{build_toon_water_material,  ToonWaterMaterial};
 
  

use bevy::{pbr::NotShadowCaster, prelude::*};

 
use serde::{Deserialize, Serialize};
 
 
pub(crate) fn liquid_plugin(app: &mut App) {
    app
   
      .add_plugins(DegenToonWaterPlugin)


      .init_resource::<LiquidManifest>()

      .add_systems(Startup, load_liquid_manifest)


      .add_systems(Update, add_model_to_liquid_plane )

      
      ;

}


 


#[derive(Component)]
pub struct LiquidPlaneComponent {
  pub liquid_type: String //"water" is default 
}



//const WATER_SIZE:u32 = 256;

 
 
  fn add_model_to_liquid_plane(
    mut commands: Commands,
    added_liquid_plane_query: Query<
        (Entity, &LiquidPlaneComponent),
        (
            Added<LiquidPlaneComponent>,
            With<GlobalTransform>,
        //    Without<Handle<Mesh>>, // ?? 
        ),
    >,
     

    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_water_materials: ResMut<Assets<ToonWaterMaterial>>,


    liquid_manifest_res : Res<LiquidManifest>

     
 
)  {
    

    for (new_entity,liquid_plane_component) in added_liquid_plane_query.iter() {
      
 

  let liquid_type = &liquid_plane_component.liquid_type ;

 
 //  let base_color = Color::rgba(0.2,0.2,0.6,1.0);
  // let emissive = Color::rgba(0.2,0.2,0.6,1.0);


  let mut liquid_material =  build_toon_water_material (  );


  if let Some( liquid_definition ) =  liquid_manifest_res.liquid_definitions.get(  liquid_type ) {

         liquid_definition.apply_to_liquid_material(&mut liquid_material);
  }


  

  let liquid_material_handle = toon_water_materials.add( 
    liquid_material
         );   

      let water_mesh =  commands.spawn( (

              Mesh3d( meshes.add(Plane3d::default().mesh().size(1.0, 1.0)) )  ,
              MeshMaterial3d( liquid_material_handle ),
            
         ) )
        .id();

    commands.entity(new_entity).add_child(
        water_mesh 
        );

    }

     
}





#[derive(Resource, Serialize,Deserialize,Clone,Debug, Default)]
pub struct LiquidManifest {

    pub liquid_definitions: HashMap< String, LiquidDefinition >

}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct LiquidDefinition {

    pub shallow_color: Option<LinearRgba> ,
    pub deep_color: Option<LinearRgba> ,
    pub foam_color: Option<LinearRgba> ,

    pub surface_noise_scroll: Option<Vec2>, //default is Vec2::new(0.1,0.1)

    pub depth_max_distance: Option<f32>, //default is 2.0 


}

impl LiquidDefinition {


 pub fn apply_to_liquid_material(&self, liquid_material: &mut ToonWaterMaterial ) {

         // let surface_noise_scroll_speed = Vec2::new(0.1,0.1);
         // water_mat.extension.custom_uniforms.surface_noise_scroll = surface_noise_scroll_speed;

  

            if let Some(shallow_color) = self.shallow_color {
                 liquid_material.extension.custom_uniforms.depth_gradient_shallow = shallow_color.into();
            }   

            if let Some(deep_color) = self.deep_color {
                 liquid_material.extension.custom_uniforms.depth_gradient_deep = deep_color.into();
            }  

            if let Some(foam_color) = self.foam_color {
                 liquid_material.extension.custom_uniforms.foam_color = foam_color.into();
            }   
 
            if let Some(surface_noise_scroll) = self.surface_noise_scroll {
                 liquid_material.extension.custom_uniforms.surface_noise_scroll = surface_noise_scroll.into();
            }   

            if let Some(depth_max_distance) = self.depth_max_distance {
                 liquid_material.extension.custom_uniforms.depth_max_distance = depth_max_distance.into();
            }   
 

    }
}


//this is a super nice way to read a manifest file ! no need to wait for bevy asset server 
fn load_liquid_manifest(

    mut liquid_manifest_res : ResMut<LiquidManifest> 
){

 //load the ron file 

  let ron_str = include_str!("../assets/liquid_manifest.liquidmanifest.ron");

    // Parse the .ron string into the Config struct
 
   *liquid_manifest_res = ron::de::from_str::<LiquidManifest>(ron_str).expect("Failed to parse RON file");
}