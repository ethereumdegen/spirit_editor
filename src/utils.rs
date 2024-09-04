


use std::path::Path;
use std::fs;


pub fn walk_dir( folder_path_name: &str , ext: &str, file_names_array: &mut Vec<String>)  {

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
