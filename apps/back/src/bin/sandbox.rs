use std::{fs, path::PathBuf};
extern crate back;

pub fn main() {
    let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ressources_dir.push("../../content/");
    println!("{:?}", ressources_dir);

    let folder = process_folder(ressources_dir.to_str().unwrap().to_string());
    match folder {
        Ok(f) => {
            println!("{:?}", f);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

fn process_folder(base_path: String) -> Result<Folder, String> {
    let mut folder = Folder {
        name: base_path.to_string(),
        files: Vec::new(),
        folders: Vec::new(),
    };

    let paths = match fs::read_dir(base_path) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(format!("Error: {:?}", e));
        }
    };

    for path in paths {
        match path {
            Ok(p) => {
                let metadata = match p.metadata() {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(format!("Error: {:?}", e));
                    }
                };
                if metadata.is_dir() {
                    match process_folder(p.path().to_str().unwrap().to_string()) {
                        Ok(sub_folder) => {
                            folder.folders.push(sub_folder);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    folder.files.push(File {
                        name: p.file_name().to_str().unwrap().to_string(),
                        content: fs::read_to_string(p.path()).unwrap(),
                    });
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(folder)
}
