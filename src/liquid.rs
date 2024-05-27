
use bevy::utils::HashMap;
use degen_toon_water::DegenToonWaterPlugin;
 
use degen_toon_water::toonwater_material::{build_toon_water_material,  ToonWaterMaterial};
 
  

use bevy::{pbr::NotShadowCaster, prelude::*};

use bevy_mod_sysfail::sysfail;
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

 

 #[sysfail]
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
    #[cfg(feature = "tracing")]
    let _span = info_span!("add_model_to_liquid_plane").entered();

    for (new_entity,liquid_plane_component) in added_liquid_plane_query.iter() {
      
 

  let liquid_type = &liquid_plane_component.liquid_type ;


  let liquid_type =  liquid_manifest_res.liquid_definitions.get(  liquid_type ) ;

 //  let base_color = Color::rgba(0.2,0.2,0.6,1.0);
  // let emissive = Color::rgba(0.2,0.2,0.6,1.0);


  let mut liquid_material =  build_toon_water_material (  );


  if let Some(liquid_type) = liquid_type {

    //modify the liquid material params based on this 

    if let Some(shallow_color) = liquid_type.shallow_color {
         liquid_material.extension.custom_uniforms.depth_gradient_shallow = shallow_color;
    }   

    if let Some(deep_color) = liquid_type.deep_color {
         liquid_material.extension.custom_uniforms.depth_gradient_deep = deep_color;
    }  

     if let Some(foam_color) = liquid_type.foam_color {
         liquid_material.extension.custom_uniforms.foam_color = foam_color;
    }   

   
  }


  let liquid_material_handle = toon_water_materials.add( 
    liquid_material
         );   

  let water_mesh =  commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
            material:  liquid_material_handle,
            ..default()
        } )
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

    pub shallow_color: Option<Color> ,
     pub deep_color: Option<Color> ,
     pub foam_color: Option<Color> ,


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