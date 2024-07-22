use std::fs;
use std::path::Path;

/*

cargo run --bin generate_doodad_manifests

*/


fn generate_model_definitions(file_names: Vec<String>) -> Vec<String> {
    file_names.iter().map(|file_name| {
        let name = file_name.replace(".glb", "");
        let model_path = format!("models/doodads/polystyle_dungeon/{}", file_name);
        format!(
            "\"{}\": (\n    name: \"{}\",\n    model: GltfModel(\"{}\"),\n    tags: Some([\"dungeon\" ]),\n),\n",
            name, name, model_path
        )
    }).collect::<Vec<String>>() //.join("\n")
}



fn main() {

	let folder_path = "assets/models/doodads/polystyle_dungeon";

    let file_names_in_folder: Vec<String> = fs::read_dir(folder_path)
        .expect("Could not read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("glb") {
                path.file_name().and_then(|name| name.to_str().map(String::from))
            } else {
                None
            }
        })
        .collect();

    let model_definitions = generate_model_definitions(file_names_in_folder);

    for definition in model_definitions {
        println!("{}", definition);
    }

}