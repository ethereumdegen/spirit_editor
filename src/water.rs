
use bevy::{pbr::NotShadowCaster, prelude::*};

use bevy_mod_sysfail::sysfail;
use bevy_water::{material::{StandardWaterMaterial, WaterMaterial}, *};

//const WATER_HEIGHT: f32 = 5.0;

pub(crate) fn water_plugin(app: &mut App) {
    app
    .insert_resource(WaterSettings {
     //   height: WATER_HEIGHT,

     
      amplitude: 0.2,
      clarity: 0.25,
      edge_scale: 0.1,
      edge_color: Color::rgba(1.0, 1.0, 1.0, 1.0),


        spawn_tiles: None, 
        ..default()
    })

    .add_plugins(WaterPlugin)
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
      settings: Res<WaterSettings>,

       mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardWaterMaterial>>,
 
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

  let water_mesh = commands
    .spawn(WaterBundle {
      name: Name::new("Water"),
      ..default()
    })
    .with_children(|parent| {
     // let grid_center = (WATER_SIZE * WATER_GRID_SIZE) as f32 / 2.0;
      
          // UV starts at (0,0) at the corner.
          let coord_offset = Vec2::new(0.0,0.0);
          // Water material.
          let material = materials.add(StandardWaterMaterial {
            base: StandardMaterial {
              base_color: settings.base_color,
              perceptual_roughness: 0.22,
              ..default()
            },
            extension: WaterMaterial {
              amplitude: settings.amplitude,
              clarity: settings.clarity,
              deep_color: settings.deep_color,
              shallow_color: settings.shallow_color,
              edge_color: settings.edge_color,
              edge_scale: settings.edge_scale,
              coord_offset,
              coord_scale: Vec2::new(WATER_SIZE as f32, WATER_SIZE as f32),
              ..default()
            }
          });

          parent.spawn((
            Name::new(format!("Liquid Plane")),
            MaterialMeshBundle {
                mesh,
                material,
               // transform: Transform::from_xyz(0.0,0.0,0.0),
                ..default()
              },

          

           // WaterTileBundle::new(mesh.clone(), material, water_height, coord_offset),
            NotShadowCaster,
          ));
        
    }).id();

    commands.entity(new_entity).add_child(
        water_mesh 
        );

    }

     
}
