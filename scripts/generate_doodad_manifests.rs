use std::path::PathBuf;
use std::fs;
use std::path::Path;

/*

cargo run --bin generate_doodad_manifests

*/



const SUB_FOLDER :&str = "academy";
const TAGS :&str = " [\"academy\"   ] ";



// This function now takes full paths and the base folder to handle paths correctly.
fn generate_model_definitions(base_folder: &str, file_paths: Vec<PathBuf>) -> Vec<String> {
    /*file_paths.iter().map(|file_path| {
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        let relative_path = file_path.strip_prefix(base_folder).unwrap().to_str().unwrap();
        let full_path = format!("models/doodads/{}", relative_path);
        format!(
            "\"{}\": (\n    name: \"{}\",\n    model: GltfModel(\"{}\"),\n    tags: Some({}),\n),\n",
            name, name, full_path, TAGS
        )
    }).collect()*/


    let mut definitions: Vec<String> = file_paths.iter().map(|file_path| {
        let name = file_path.file_stem().unwrap().to_str().unwrap();
        let relative_path = file_path.strip_prefix(base_folder).unwrap().to_str().unwrap();
        let full_path = format!("models/doodads/{}", relative_path);
        format!(
            "\"{}\": (\n    name: \"{}\",\n    model: GltfModel(\"{}\"),\n    material_replacement_set: Some(\"academy\"),  \n  tags: Some({}),\n),\n",
            name, name, full_path, TAGS
        )
    }).collect();

    // Sort the definitions alphabetically
    definitions.sort();

    definitions
}

fn main() {
    let base_folder = "artifacts/game_assets/models/doodads";
    let folder_path = format!("{}/{}", base_folder, SUB_FOLDER);
    let mut file_paths_in_folder = Vec::new();



    walk_dir(Path::new(&folder_path), "glb", &mut file_paths_in_folder);

    // Sort file paths alphabetically before generating model definitions
    file_paths_in_folder.sort_by(|a, b| {
        a.file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .cmp(b.file_stem().unwrap().to_str().unwrap())
    });

    let model_definitions = generate_model_definitions(base_folder, file_paths_in_folder);

    for definition in model_definitions {
        println!("{}", definition);
    }


    /*

    walk_dir(Path::new(&folder_path), "glb", &mut file_paths_in_folder);

    // Now pass the base_folder to generate_model_definitions
    let model_definitions = generate_model_definitions(base_folder, file_paths_in_folder);

    for definition in model_definitions {
        println!("{}", definition);
    }*/
}

pub fn walk_dir(folder_path: &Path, ext: &str, file_paths_array: &mut Vec<PathBuf>) {
    if folder_path.is_dir() {
        for entry in fs::read_dir(folder_path).expect("Could not read directory") {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path, ext, file_paths_array); // Recurse into subdirectory
            } else if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some(ext) {
                file_paths_array.push(path);
            }
        }
    }
}



/*
fn generate_model_definitions(file_names: Vec<String>) -> Vec<String> {
    file_names.iter().map(|file_name| {
        let name = file_name.replace(".glb", "");
        let model_path = format!("models/doodads/{}/{}", SUB_FOLDER, file_name);
        format!(
            "\"{}\": (\n    name: \"{}\",\n    model: GltfModel(\"{}\"),\n    tags: Some({}),\n),\n",
            name, name, model_path, TAGS
        )
    }).collect::<Vec<String>>() //.join("\n")
}



fn main() {

     let folder_path = format!("artifacts/game_assets/models/doodads/{}", SUB_FOLDER);
    let mut file_paths_in_folder = Vec::new();

    walk_dir( &folder_path, "glb", &mut file_paths_in_folder);


    let file_names_in_folder = ;

    let model_definitions = generate_model_definitions(file_paths_in_folder);

    for definition in model_definitions {
        println!("{}", definition);
    }



}



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
*/