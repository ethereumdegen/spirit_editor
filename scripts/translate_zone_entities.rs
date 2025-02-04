

use bevy::prelude::Vec3;
use std::path::Path;
use std::fs;

use spirit_edit_core::zones::zone_file::{ZoneEntityV2, ZoneFile, ZoneFileV2};
 



fn main() {

 

    let zone_path = "assets/zonesv2/vessara/city_vessara_core.zone.ron";


  


    	let Some(mut zone_file) = ZoneFileV2::load_from_path( Path::new(&zone_path) )  else {
    		eprintln!("could not parse {:?}", zone_path);
    		return;
    	};



        let delta_translation = Vec3::new(600.0,-0.0,600.0);




        for entity in zone_file.entities.iter_mut() {
                match entity {
                    ZoneEntityV2::Doodad { transform, .. } => {
                        transform.translation += delta_translation;
                    }
                    _ => {}
                }
            }

    
    	 

    	let zone_file_name = format!("assets/zonesv2/vessara/city_vessara_core_new.zone.ron" );

    	println!("exporting file to {}",zone_file_name);
        let ron = ron::ser::to_string(&zone_file).unwrap();
        let file_saved = std::fs::write(zone_file_name, ron);




   // }

}
 