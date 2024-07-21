use std::{any::Any, fs, path::PathBuf};
extern crate back;

use back::markdown::doc_header::DocHeader;

pub fn main() {
    let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ressources_dir.push("../../content/");
    println!("{:?}", ressources_dir);

    let paths = match fs::read_dir(ressources_dir) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    for path in paths {
        match path {
            Ok(p) => {
                p.metadata().unwrap().is_dir();
                let file_name = p.file_name();
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".md") {
                    let content = fs::read_to_string(p.path()).unwrap();
                    let (header, _) = DocHeader::parse_md(&content).unwrap();
                    println!("{:?}", header);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
