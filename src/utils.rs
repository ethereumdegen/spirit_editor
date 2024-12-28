


use std::io;
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




// This function copies all files and folders from `src_dir` to `dst_dir`
pub fn copy_dir_recursive(src_dir: &Path, dst_dir: &Path) -> io::Result<()> {
    // Ensure the destination directory exists
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    // Iterate through the entries in the source directory
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst_dir.join(file_name);

        if src_path.is_dir() {
            // If the entry is a directory, recursively copy it
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if src_path.is_file() {
            // If the entry is a file, copy it to the destination directory
            fs::copy(&src_path, &dst_path)?;
        }
    }

    println!("copying from {:?} to {:?}",src_dir,dst_dir);

    Ok(())
}


pub trait StringUtilsExt {
    fn ensure_ends_with(&self, suffix: &str) -> String;
}

impl StringUtilsExt for String {
    fn ensure_ends_with(&self, suffix: &str) -> String {
        if self.ends_with(suffix) {
            self.to_string()
        } else {
            format!("{}{}", self, suffix)
        }
    }
}
