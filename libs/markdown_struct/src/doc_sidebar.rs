use serde::{Deserialize, Serialize};

use super::{content_struct::PageShort, doc_header::DocHeaderParseError, page_database::DbFolder};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DocCategory {
    pub name: String,
    pub has_index: bool,
    pub sub_categories: Vec<DocCategory>,
    pub pages: Vec<PageShort>,
}

#[derive(Debug)]
pub enum DocCategoryGenerateError {
    MetadataParseError(DocHeaderParseError),
}

impl DocCategory {
    pub fn new(name: String) -> DocCategory {
        DocCategory {
            name,
            has_index: false,
            sub_categories: Vec::new(),
            pages: Vec::new(),
        }
    }

    pub fn generate_sidebar_from_db(db_folder: DbFolder, path: String) -> DocCategory {
        let mut doc_category = DocCategory::new(db_folder.name.clone());
        for page in db_folder.pages {
            if page.metadata.spec.blog {
                continue;
            }
            let path: String = {
                if path.is_empty() {
                    page.name.clone()
                } else {
                    format!("{}/{}", path, page.name)
                }
            };
            doc_category.pages.push(page.to_short(path));
            if page.name == "index" {
                doc_category.has_index = true;
            }
        }
        for (_, sub_folder) in db_folder.sub_folders {
            let path_sub_folder = {
                if path.is_empty() {
                    sub_folder.name.clone()
                } else {
                    format!("{}/{}", path, sub_folder.name)
                }
            };
            let sub_doc_category =
                DocCategory::generate_sidebar_from_db(sub_folder.clone(), path_sub_folder);
            doc_category.sub_categories.push(sub_doc_category);
        }
        doc_category
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{content_struct::Page, doc_header::DocHeader};
    use chrono::DateTime;

    pub fn get_test_doc_header() -> DocHeader {
        DocHeader {
            title: "Test Title".to_string(),
            date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                .unwrap()
                .into(),
            description: None,
            weight: 0,
            spec: Default::default(),
            tags: vec![],
            techno: vec![],
            links: vec![],
        }
    }

    pub fn get_test_page() -> Page {
        Page::new(
            "test_page".to_string(),
            "test_content".to_string(),
            get_test_doc_header(),
        )
    }

    #[test]
    fn test_doc_category_new() {
        let doc_category = DocCategory::new("test".to_string());
        assert_eq!(
            doc_category,
            DocCategory {
                name: "test".to_string(),
                has_index: false,
                sub_categories: Vec::new(),
                pages: Vec::new(),
            }
        );
        let serialised_doc_category = serde_json::to_string(&doc_category).unwrap();
        let deserialised_doc_category: DocCategory = serde_json::from_str(&serialised_doc_category)
            .expect("Failed to deserialise DocCategory");
        assert_eq!(doc_category, deserialised_doc_category);
    }

    #[test]
    fn test_doc_category_generate_sidebar_from_db() {
        let db_folder = DbFolder {
            name: "test".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        let doc_category = DocCategory::generate_sidebar_from_db(db_folder, "".to_string());
        assert_eq!(doc_category.name, "test".to_string());
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 1);
        assert_eq!(doc_category.pages[0].name, "test_page".to_string());
    }

    #[test]
    fn test_doc_category_generate_sidebar_from_db_only_blog() {
        let mut db_folder = DbFolder {
            name: "test".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        db_folder.pages[0].metadata.spec.blog = true;
        db_folder.pages[0].metadata.spec.doc = false;
        let doc_category = DocCategory::generate_sidebar_from_db(db_folder, "".to_string());
        assert_eq!(doc_category.name, "test".to_string());
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 0);
    }

    #[test]
    fn test_doc_category_generate_sidebar_from_db_sub_category_has_index() {
        let mut db_folder = DbFolder {
            name: "test".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        let mut sub_folder = DbFolder {
            name: "sub".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        sub_folder.pages[0].name = "index".to_string();
        db_folder.sub_folders.insert("sub".to_string(), sub_folder);
        let doc_category = DocCategory::generate_sidebar_from_db(db_folder, "".to_string());
        assert_eq!(doc_category.name, "test".to_string());
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 1);
        assert_eq!(doc_category.pages.len(), 1);
        assert_eq!(doc_category.sub_categories[0].name, "sub".to_string());
        assert_eq!(doc_category.sub_categories[0].has_index, true);
    }

    #[test]
    fn test_doc_category_generate_sidebar_from_db_sub_sub_category() {
        let mut db_folder = DbFolder {
            name: "test".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        let mut sub_folder = DbFolder {
            name: "sub".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        let sub_sub_folder = DbFolder {
            name: "sub_sub".to_string(),
            pages: vec![get_test_page()],
            sub_folders: Default::default(),
        };
        sub_folder
            .sub_folders
            .insert("sub_sub".to_string(), sub_sub_folder);
        db_folder.sub_folders.insert("sub".to_string(), sub_folder);
        let doc_category = DocCategory::generate_sidebar_from_db(db_folder.clone(), "".to_string());
        assert_eq!(doc_category.name, "test".to_string());
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 1);
        assert_eq!(doc_category.pages.len(), 1);
        assert_eq!(doc_category.sub_categories[0].name, "sub".to_string());
        assert_eq!(doc_category.sub_categories[0].has_index, false);
        assert_eq!(doc_category.sub_categories[0].sub_categories.len(), 1);
        assert_eq!(doc_category.sub_categories[0].pages.len(), 1);
        assert_eq!(
            doc_category.sub_categories[0].sub_categories[0].name,
            "sub_sub".to_string()
        );
        assert_eq!(
            doc_category.sub_categories[0].sub_categories[0].has_index,
            false
        );
        assert_eq!(
            doc_category.sub_categories[0].sub_categories[0]
                .sub_categories
                .len(),
            0
        );
        assert_eq!(
            doc_category.sub_categories[0].sub_categories[0].pages.len(),
            1
        );

        let full_page = db_folder.get_page_in_sub_folder_by_path(
            doc_category.sub_categories[0].sub_categories[0].pages[0]
                .path
                .clone(),
        );
        assert!(full_page.is_some());
        let full_page = full_page.unwrap();
        assert_eq!(
            full_page.name,
            doc_category.sub_categories[0].sub_categories[0].pages[0].name
        );
        assert_eq!(
            full_page.metadata,
            doc_category.sub_categories[0].sub_categories[0].pages[0].metadata
        );
    }
}
