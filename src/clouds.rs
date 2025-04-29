/*


just for fun ... 

*/




use degen_toon_clouds::plane3d_cloud_material::Plane3dCloudMaterial;
use degen_toon_clouds::plane3d_cloud_material::build_plane_3d_cloud_material;
use degen_toon_clouds::DegenToonCloudsPlugin;
use crate::AssetLoadState;
 
use crate::{asset_loading::TextureAssets, utils::StringUtilsExt};
use bevy::prelude::*; 
 

use bevy::pbr::{NotShadowCaster,NotShadowReceiver};

use crate::{asset_loading::DecalAssets, decal_manifest::DecalManifest};

pub(crate) fn clouds_plugin(app: &mut App) {
    app
    
      	.add_plugins(DegenToonCloudsPlugin)
      .add_systems(Update, spawn_planar_clouds 
      	.run_if( in_state(   AssetLoadState::Complete )
      	.and( not( any_with_component::<PlanarClouds3dComponent> ) )  ) )

   	
      
      ;

}



#[derive(Component,Debug,Clone,Reflect)]
#[reflect(Component)]
pub struct PlanarClouds3dComponent  ;
 


 fn spawn_planar_clouds(

 	mut commands :Commands ,
 	mut meshes: ResMut<Assets<Mesh>>,
    

 	 mut toon_cloud_materials: ResMut<Assets<Plane3dCloudMaterial>>,

 	) {



 	   let   mut cloud_material = build_plane_3d_cloud_material (  );


 	   cloud_material.extension.custom_uniforms.foam_color = LinearRgba::new(1.0,1.0,1.0,0.2);
 

  
       let planar_cloud_material_handle = toon_cloud_materials.add( cloud_material );
 


 	commands.spawn((

 		 Mesh3d(   meshes.add(Plane3d::default().mesh().size(1.0, 1.0))  ), 
         MeshMaterial3d( planar_cloud_material_handle ) ,
         Transform::from_xyz(0.0,200.0,0.0).with_scale( (5000.0,1.0,5000.0).into()  ),

 		PlanarClouds3dComponent

 		)); 





 }