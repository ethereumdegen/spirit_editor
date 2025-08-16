


use crate::AssetLoadState;
 
use crate::{asset_loading::TextureAssets, utils::StringUtilsExt};
use bevy::pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt};
use bevy::prelude::*; 

 // use bevy_contact_projective_decals::{DecalMaterialExtension,decal_mesh_quad,DecalMaterial};

use bevy::pbr::{NotShadowCaster,NotShadowReceiver};

use crate::{asset_loading::DecalAssets, decal_manifest::DecalManifest};

pub(crate) fn decals_plugin(app: &mut App) {
    app
    
      .register_type::< DecalComponent >()
      .add_systems(Update, build_decals .run_if(in_state(   AssetLoadState::Complete )  ) )

   	
      
      ;

}



#[derive(Component,Debug,Clone,Reflect)]
#[reflect(Component)]
pub struct DecalComponent { 
    pub decal_name: String
}
 




fn build_decals(
	mut commands:Commands, 
	decal_query: Query< (Entity, &DecalComponent), Added<DecalComponent> >,

	 mut meshes: ResMut<Assets<Mesh>>,
	
 	mut decal_standard_materials: ResMut<Assets<ForwardDecalMaterial<StandardMaterial>>>,


	 decal_manifest_handles: Res<DecalAssets>, 
	 decal_manifest_assets: Res<Assets<DecalManifest>>,

	 texture_asset_handles: Res< TextureAssets >

){

	for (decal_entity, decal_comp) in decal_query.iter() {

		



	 
		let decal_name = decal_comp.decal_name.clone().ensure_ends_with(".decal");


		let Some(decal_type_data) = decal_manifest_handles.decals
		.get(decal_name.as_str())
		.map(|h| decal_manifest_assets.get(h) ).flatten()  else {
			warn!("could not load decal type data: {} ", decal_name);
			continue;
		} ;


 		let diffuse_texture_name = &decal_type_data.diffuse_texture;
 		let base_color = &decal_type_data.base_color;

 		let emissive_texture_name = &decal_type_data.emissive_texture ;
 		let emissive_color = decal_type_data.emissive_color.clone().unwrap_or( LinearRgba::BLACK ) ;


 		let diffuse_texture_handle  = texture_asset_handles.decal_textures.get( diffuse_texture_name.as_str() ) ;

 		if diffuse_texture_handle.is_none(){

 			warn!("could not load decal texture: {} {}", decal_name, diffuse_texture_name);
 			continue;

 		}

 		let emissive_texture_handle = emissive_texture_name.as_ref()
 			.map(|n| texture_asset_handles.decal_textures.get( n.as_str() )  )
 				.flatten();

 
 		if let Ok(mut cmd) = commands.get_entity(decal_entity) {

	        cmd.insert((
	            //Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(4.0)),
	            
                 MeshMaterial3d(decal_standard_materials.add(ForwardDecalMaterial {
                    base: StandardMaterial {
                        
                        base_color_texture: diffuse_texture_handle.cloned(),
                        base_color: base_color.clone().into(),
                        alpha_mode: AlphaMode::Blend,
                        unlit: decal_type_data.unlit.clone(),
                        emissive: emissive_color.into(),
                        emissive_texture: emissive_texture_handle.cloned(),
                        ..default()

                    },
                    extension: ForwardDecalMaterialExt {
                        depth_fade_factor: 1.0,
                    },
                })),

                 ForwardDecal,

	             //Visibility::default(),
	             NotShadowCaster, 
	             NotShadowReceiver,
	 


        )) ;





		}


	}



}

/*


 commands.spawn(



        (
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(4.0)),
            MeshMaterial3d (

                    decal_materials.add(DecalMaterialExtension {
                        base: StandardMaterial {
                            base_color_texture: Some(asset_server.load("blast.png")),
                            base_color: Color::Srgba(Srgba::RED),
                            alpha_mode: AlphaMode::Blend,
                            ..default()
                        },
                        extension: DecalMaterial {
                            depth_fade_factor: 8.0,
                        },
                    })

               ),
             Mesh3d( meshes.add(decal_mesh_quad(Vec3::Y)) ),

             Visibility::default(),
             NotShadowCaster, 
             NotShadowReceiver,





        )

        );


*/