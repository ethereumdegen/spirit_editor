use bevy::prelude::*;


pub fn materials_plugin(app:&mut App){


          app

            //.add_systems(Startup, register_foliage_assets)
            .add_systems(Update, (
                
             	inspect_gltfs
                ).chain()
               
           )

            ;
}

fn inspect_gltfs(

	mut ev_asset: EventReader<AssetEvent<Gltf>>,

	mut gltf_assets: Res<Assets<Gltf>>, 

	){



	  for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {


            	let Some(loaded_gltf) = gltf_assets.get( *id   ) else { continue } ; 

            	 


            }


            _ => {} 
          
        }
    }


}