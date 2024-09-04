

use std::path::Path;
use std::fs;

use spirit_edit_core::zones::zone_file::{ZoneFile, ZoneFileV2};
 



fn main() {

 

    let folder_path = "assets/zones";


    //let export_folder_path = "assets/zonesv2";



    let mut file_names_array : Vec<String> = Vec::new();

    walk_dir( folder_path, "ron" , &mut file_names_array) ;
 

    for file_name in file_names_array {

    	println!("file name {}", file_name);

    	let Some(zone_file) = ZoneFile::load_from_path( Path::new(&file_name) )  else {
    		eprintln!("could not parse {:?}", file_name);
    		continue;
    	};

    	let upgraded_zone_file = ZoneFileV2::from_zone_file(zone_file.clone());

    	let path_parts:Vec<&str> = file_name.split("/").collect();
    	let zone_name = path_parts.last().unwrap();



    	 let fixed_zone_name = match zone_name.ends_with( "zone.ron" ) || zone_name.ends_with( "zone" ){

            true => {

                  let   parts: Vec<&str> = zone_name.split('.').collect();
                 
                    parts.first().unwrap() .to_string()  
                   

              }, 
            false => zone_name.to_string()

        };


    	let zone_file_name = format!("assets/zonesv2/{}.zone.ron", fixed_zone_name);

    	println!("exporting file to {}",zone_file_name);
        let ron = ron::ser::to_string(&upgraded_zone_file).unwrap();
        let file_saved = std::fs::write(zone_file_name, ron);




    }

}


fn walk_dir( folder_path_name: &str , ext: &str, file_names_array: &mut Vec<String>)  {

	let folder_path = Path::new(folder_path_name);
 
	 if folder_path.is_dir() {
        for entry in fs::read_dir(folder_path).expect("Could not read directory") {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();

            if path.is_dir() {
                walk_dir(   path.to_str().unwrap()  , &ext, file_names_array ); // Recurse into subdirectory
            } else if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some( ext ) {
                if let Some(path_str) = path.to_str() {
                    file_names_array.push(String::from(path_str));
                }
            }
        }
    }
	 


}