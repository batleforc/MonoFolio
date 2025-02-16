use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use super::{content_struct::PageShort, page_database::DbFolder};

// List of projects, contain the short representation of the pages, mainly aimed at the Home screen.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct ProjectList {
    pub projects: HashMap<String, PageShort>,
}

impl Default for ProjectList {
    fn default() -> Self {
        ProjectList::new()
    }
}

impl ProjectList {
    pub fn new() -> Self {
        ProjectList {
            projects: HashMap::new(),
        }
    }

    pub fn generate_project_list_from_db(db_folder: DbFolder, path: String) -> ProjectList {
        let mut project_list = ProjectList::new();
        for page in db_folder.pages {
            if page.metadata.spec.project {
                let path: String = {
                    if path.is_empty() {
                        page.name.clone()
                    } else {
                        format!("{}/{}", path, page.name)
                    }
                };
                project_list.add_page(page.to_short(path));
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
            let sub_project_list =
                ProjectList::generate_project_list_from_db(sub_folder.clone(), path_sub_folder);
            project_list.projects.extend(sub_project_list.projects);
        }
        project_list
    }

    pub fn add_page(&mut self, page: PageShort) {
        self.projects.insert(page.url_encode_name(), page);
    }

    pub fn get_page(&self, page_id: &str) -> Option<&PageShort> {
        self.projects.get(page_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{content_struct::PageV2, doc_header::DocHeader};
    use chrono::DateTime;

    pub fn get_test_doc_header() -> DocHeader {
        DocHeader {
            title: "Test Title".to_string(),
            date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                .unwrap()
                .into(),
            ..Default::default()
        }
    }

    pub fn get_test_page() -> PageV2 {
        PageV2::new(
            "test_page".to_string(),
            "test_content".to_string(),
            get_test_doc_header(),
        )
    }

    #[test]
    fn test_project_list_new() {
        let project_list = ProjectList::new();
        assert_eq!(project_list.projects.len(), 0);
        let project_list = ProjectList::default();
        assert_eq!(project_list.projects.len(), 0);
    }

    #[test]
    fn test_project_deserialize() {
        let project_list = ProjectList::new();
        let serialized = serde_json::to_string(&project_list).unwrap();
        let deserialized: ProjectList = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project_list, deserialized);
    }

    #[test]
    fn test_project_add_page() {
        let mut project_list = ProjectList::new();
        let page = get_test_page().to_short("test_page".to_string());
        project_list.add_page(page.clone());
        assert_eq!(
            project_list.projects.get(&page.url_encode_name()),
            Some(&page)
        );
    }

    #[test]
    fn test_project_get_page() {
        let mut project_list = ProjectList::new();
        let page = get_test_page().to_short("test_page".to_string());
        project_list.add_page(page.clone());
        assert_eq!(project_list.get_page(&page.url_encode_name()), Some(&page));
    }
}
