
use degen_toon_water::DegenToonWaterPlugin;
 
use degen_toon_water::toonwater_material::{build_toon_water_material,  ToonWaterMaterial};
 
  

use bevy::{pbr::NotShadowCaster, prelude::*};

use bevy_mod_sysfail::sysfail;
 

//const WATER_HEIGHT: f32 = 5.0;

pub(crate) fn water_plugin(app: &mut App) {
    app
   
      .add_plugins(DegenToonWaterPlugin)
      .add_systems(Update, add_model_to_liquid_plane )

      
      ;

}


#[derive(Component)]
pub struct LiquidPlaneComponent {}



const WATER_SIZE:u32 = 256;

 

 #[sysfail]
pub(crate) fn add_model_to_liquid_plane(
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
     
 
)  {
    #[cfg(feature = "tracing")]
    let _span = info_span!("add_model_to_liquid_plane").entered();

    for (new_entity,liquid_plane_component) in added_liquid_plane_query.iter() {
      

      
  let water_height = 0.0;
  // Generate mesh for water.
  let mesh: Handle<Mesh> = meshes.add(
    Mesh::from( Plane3d{

        normal:  Direction3d::Y
    })
  );


   let base_color = Color::rgba(0.2,0.2,0.6,1.0);
    let emissive = Color::rgba(0.2,0.2,0.6,1.0);


  let toon_water_material_handle = toon_water_materials.add( 
         build_toon_water_material (
            base_color,
            emissive,
             None,  
             None,  
        ) );   

  let water_mesh =  commands.spawn((MaterialMeshBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material:  toon_water_material_handle,
            ..default()
        } ))
        .id();

    commands.entity(new_entity).add_child(
        water_mesh 
        );

    }

     
}
