use std::collections::HashMap;

use super::{
    content_struct::Page,
    doc_header::{DocHeader, DocHeaderParseError},
    folder_struct::Folder,
};
use serde::{Deserialize, Serialize};

// Mise en place d'une "base de donnée" contenant toute les pages
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DbFolder {
    pub name: String,
    pub pages: Vec<Page>,
    pub sub_folders: HashMap<String, DbFolder>,
}

#[derive(Debug)]
pub enum DbFolderGenerationError {
    MetadataParseError(DocHeaderParseError),
}

impl DbFolder {
    pub fn new(name: String) -> Self {
        DbFolder {
            name,
            pages: Vec::new(),
            sub_folders: HashMap::new(),
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn add_sub_folder(&mut self, folder: DbFolder) {
        self.sub_folders.insert(folder.name.clone(), folder);
    }

    pub fn get_page(&self, name: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.name == name)
    }

    pub fn get_page_in_sub_folder_by_path(&self, path: String) -> Option<&Page> {
        let path = path.split('/').collect::<Vec<&str>>();
        if path.len() == 1 {
            return self.get_page(path[0]);
        }
        if let Some(sub_folder) = self.sub_folders.get(path[0]) {
            let mut path = path;
            path.remove(0);
            return sub_folder.get_page_in_sub_folder_by_path(path.join("/"));
        }
        None
    }

    pub fn generate_database(folder_struct: Folder) -> Result<DbFolder, DbFolderGenerationError> {
        let mut db_folder = DbFolder::new(
            folder_struct
                .name
                .clone()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
        );
        for folder in folder_struct.folders {
            match DbFolder::generate_database(folder) {
                Ok(sub_db_folder) => {
                    db_folder.add_sub_folder(sub_db_folder);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        for file in folder_struct.files {
            if file.name.ends_with(".md") {
                let (metadata, content) = match DocHeader::parse_md(&file.content) {
                    Ok((metadata, content)) => (metadata, content),
                    Err(e) => return Err(DbFolderGenerationError::MetadataParseError(e)),
                };
                db_folder.add_page(Page::new(file.name.replace(".md", ""), content, metadata));
            }
        }
        Ok(db_folder)
    }
}

#[cfg(test)]
mod tests {
    use crate::{doc_header::DocHeader, folder_struct::File};

    use super::*;
    use chrono::DateTime;

    pub fn get_test_doc_header() -> DocHeader {
        DocHeader {
            title: "Test Title".to_string(),
            date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                .unwrap()
                .into(),
            description: None,
            writter: Default::default(),
            weight: 0,
            spec: Default::default(),
            tags: Vec::new(),
            techno: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn get_test_page() -> Page {
        Page::new(
            "Test Page".to_string(),
            "Test Content".to_string(),
            get_test_doc_header(),
        )
    }

    pub fn get_test_db_folder() -> DbFolder {
        let mut folder = DbFolder::new("Test Folder".to_string());
        folder.add_page(get_test_page());
        folder
    }

    #[test]
    fn test_db_folder() {
        let folder = get_test_db_folder();
        assert_eq!(folder.name, "Test Folder");
        assert_eq!(folder.pages.len(), 1);
        assert_eq!(folder.sub_folders.len(), 0);
    }

    #[test]
    fn test_db_folder_serialize_deserialize() {
        let folder = get_test_db_folder();
        let serialized = serde_json::to_string(&folder).unwrap();
        let deserialized: DbFolder = serde_json::from_str(&serialized).unwrap();
        assert_eq!(folder, deserialized);
    }

    #[test]
    fn test_db_folder_add_sub_folder() {
        let mut folder = get_test_db_folder();
        let sub_folder = DbFolder::new("Sub Folder".to_string());
        folder.add_sub_folder(sub_folder);
        assert_eq!(folder.sub_folders.len(), 1);
    }

    #[test]
    fn test_db_folder_get_page() {
        let folder = get_test_db_folder();
        let page = folder.get_page("Test Page").unwrap();
        assert_eq!(page.name, "Test Page");
    }

    pub fn get_folder_struct() -> Folder {
        Folder {
            name: "test_folder".to_string(),
            files: vec![],
            folders: vec![],
        }
    }

    pub fn get_test_folder_with_no_md_file() -> Folder {
        let mut folder = get_folder_struct();
        folder.files.push(File {
            name: "test_file.txt".to_string(),
            content: "Test content".to_string(),
        });
        folder
    }

    pub fn get_test_folder_with_invalid_files() -> Folder {
        let mut folder = get_folder_struct();
        folder.files.push(File {
            name: "test_file.md".to_string(),
            content: "Test content".to_string(),
        });
        folder
    }

    pub fn get_test_sub_folder_invalid_files() -> Folder {
        let mut folder = get_folder_struct();
        folder.folders.push(get_test_folder_with_invalid_files());
        folder
    }

    pub fn get_test_folder_with_empty_folders() -> Folder {
        let mut folder = get_folder_struct();
        folder.folders.push(Folder {
            name: "test_sub_folder".to_string(),
            files: vec![],
            folders: vec![],
        });
        folder
    }

    pub fn get_page_valid() -> File {
        File {
            name: "test_file.md".to_string(),
            content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content".to_string(),
        }
    }

    pub fn get_folder_valid() -> Folder {
        let mut folder = get_folder_struct();
        folder.files.push(get_page_valid());
        folder
    }

    pub fn get_folder_with_sub_folder() -> Folder {
        let mut folder = get_folder_struct();
        folder.folders.push(get_folder_valid());
        folder
    }

    #[test]
    fn test_db_folder_generate_database() {
        let folder = get_folder_struct();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        assert_eq!(db_folder.name, "test_folder");
        assert_eq!(db_folder.pages.len(), 0);
        assert_eq!(db_folder.sub_folders.len(), 0);
    }

    #[test]
    fn test_db_folder_generate_database_with_empty_folders() {
        let folder = get_test_folder_with_empty_folders();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        assert_eq!(db_folder.name, "test_folder");
        assert_eq!(db_folder.pages.len(), 0);
        assert_eq!(db_folder.sub_folders.len(), 1);
    }

    #[test]
    fn test_db_folder_generate_database_with_invalid_files() {
        let folder = get_test_folder_with_invalid_files();
        let db_folder = DbFolder::generate_database(folder);
        assert!(db_folder.is_err());
    }

    #[test]
    fn test_db_folder_generate_database_with_invalid_files_in_sub_folder() {
        let folder = get_test_sub_folder_invalid_files();
        let db_folder = DbFolder::generate_database(folder);
        assert!(db_folder.is_err());
    }

    #[test]
    fn test_db_folder_generate_database_with_no_md_file() {
        let folder = get_test_folder_with_no_md_file();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        assert_eq!(db_folder.name, "test_folder");
        assert_eq!(db_folder.pages.len(), 0);
        assert_eq!(db_folder.sub_folders.len(), 0);
    }

    #[test]
    fn test_db_folder_generate_database_with_files() {
        let folder = get_folder_valid();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        assert_eq!(db_folder.name, "test_folder");
        assert_eq!(db_folder.pages.len(), 1);
        assert_eq!(db_folder.sub_folders.len(), 0);
    }

    #[test]
    fn test_db_folder_generate_database_with_sub_folder() {
        let folder = get_folder_with_sub_folder();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        assert_eq!(db_folder.name, "test_folder");
        assert_eq!(db_folder.pages.len(), 0);
        assert_eq!(db_folder.sub_folders.len(), 1);
    }

    #[test]
    fn test_db_folder_get_page_by_path() {
        let folder = get_folder_valid();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        let page = db_folder.get_page_in_sub_folder_by_path("test_file".to_string());
        assert!(page.is_some());
        assert_eq!(page.unwrap().name, "test_file");
    }
    #[test]
    fn test_db_folder_get_page_by_path_does_not_exist() {
        let folder = get_folder_struct();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        let page = db_folder.get_page_in_sub_folder_by_path("not_file".to_string());
        assert!(page.is_none());
    }
    #[test]
    fn test_db_folder_get_page_in_sub_folder_by_path_does_not_exist() {
        let folder = get_folder_struct();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        let page = db_folder.get_page_in_sub_folder_by_path("test_folder/test_file".to_string());
        assert!(page.is_none());
    }
    #[test]
    fn test_db_folder_get_page_in_sub_folder_by_path() {
        let folder = get_folder_with_sub_folder();
        let db_folder = DbFolder::generate_database(folder).unwrap();
        let page = db_folder.get_page_in_sub_folder_by_path("test_folder/test_file".to_string());
        assert!(page.is_some());
        assert_eq!(page.unwrap().name, "test_file");
    }
}
