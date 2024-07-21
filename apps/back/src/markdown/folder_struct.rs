use std::fs;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct File {
    pub name: String,
    pub content: String,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Folder {
    pub name: String,
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

#[derive(Debug)]
pub enum ProcessFolderStructError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
}

pub fn process_folder_struct(base_path: String) -> Result<Folder, ProcessFolderStructError> {
    let mut folder = Folder {
        name: base_path.to_string(),
        files: Vec::new(),
        folders: Vec::new(),
    };

    let paths = match fs::read_dir(base_path) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(ProcessFolderStructError::IoError(e));
        }
    };

    for path in paths {
        match path {
            Ok(p) => {
                let metadata = match p.metadata() {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(ProcessFolderStructError::IoError(e));
                    }
                };
                if metadata.is_dir() {
                    match process_folder_struct(p.path().to_str().unwrap().to_string()) {
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

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;
    const TEST_FOLDER: &str = "../../test_dataset/content";

    #[test]
    fn test_process_folder_struct() {
        let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        ressources_dir.push(TEST_FOLDER);
        let folder = process_folder_struct(ressources_dir.to_str().unwrap().to_string()).unwrap();
        assert_eq!(folder.name, ressources_dir.to_str().unwrap().to_string());
        assert_eq!(folder.files.len(), 1);
        assert_eq!(folder.folders.len(), 2);
        assert_eq!(folder.files[0].name, "index.md");
        assert_eq!(
            folder.folders[0].name,
            format!("{}/project", ressources_dir.to_str().unwrap())
        );
        assert_eq!(
            folder.folders[1].name,
            format!("{}/cicd", ressources_dir.to_str().unwrap())
        );
    }
    #[test]
    fn test_process_folder_path_invalid() {
        let folder = process_folder_struct("invalid_path".to_string());
        assert!(folder.is_err());
        assert!(matches!(
            folder.unwrap_err(),
            ProcessFolderStructError::IoError(_)
        ));
    }

    #[test]
    fn test_process_folder_empty_dir() {
        let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        ressources_dir.push("../../test_dataset/empty");
        fs::create_dir_all(ressources_dir.to_str().unwrap()).unwrap();
        let folder = process_folder_struct(ressources_dir.to_str().unwrap().to_string()).unwrap();
        assert_eq!(folder.name, ressources_dir.to_str().unwrap().to_string());
        assert_eq!(folder.files.len(), 0);
        assert_eq!(folder.folders.len(), 0);
    }
}
