use std::path::PathBuf;

use back::markdown::{folder_struct::process_folder_struct, page_database::DbFolder};
extern crate back;

pub fn main() {
    let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ressources_dir.push("../../content/");
    let content_folder = match process_folder_struct(ressources_dir.to_str().unwrap().to_string()) {
        Ok(folder) => folder,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };

    let db_folder = match DbFolder::generate_database(content_folder) {
        Ok(folder) => folder,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };

    println!("{:?}", db_folder);
}
