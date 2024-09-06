use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{content_struct::PageShort, page_database::DbFolder};

/// Timeline of the blog space, contain the short representation of the pages.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct BlogTimeline {
    pub pages: HashMap<String, PageShort>,
}

impl Default for BlogTimeline {
    fn default() -> Self {
        BlogTimeline::new()
    }
}

impl BlogTimeline {
    pub fn new() -> Self {
        BlogTimeline {
            pages: HashMap::new(),
        }
    }

    pub fn generate_timeline_from_db(db_folder: DbFolder, path: String) -> BlogTimeline {
        let mut blog_timeline = BlogTimeline::new();
        for page in db_folder.pages {
            if page.metadata.spec.blog {
                let path: String = {
                    if path.is_empty() {
                        page.name.clone()
                    } else {
                        format!("{}/{}", path, page.name)
                    }
                };
                blog_timeline.add_page(page.to_short(path));
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
            let sub_blog_timeline =
                BlogTimeline::generate_timeline_from_db(sub_folder.clone(), path_sub_folder);
            blog_timeline.pages.extend(sub_blog_timeline.pages);
        }
        blog_timeline
    }

    pub fn add_page(&mut self, page: PageShort) {
        self.pages.insert(page.url_encode_name(), page);
    }

    pub fn get_page(&self, page_id: &str) -> Option<&PageShort> {
        self.pages.get(page_id)
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
            ..Default::default()
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
    fn test_blog_timeline_new() {
        let blog_timeline = BlogTimeline::new();
        assert_eq!(blog_timeline.pages.len(), 0);
        let blog_timeline = BlogTimeline::default();
        assert_eq!(blog_timeline.pages.len(), 0);
    }

    #[test]
    fn test_blog_deserialize() {
        let blog_timeline = BlogTimeline::new();
        let serialized = serde_json::to_string(&blog_timeline).unwrap();
        let deserialized: BlogTimeline = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blog_timeline, deserialized);
    }

    #[test]
    fn test_blog_timeline_add_page() {
        let mut blog_timeline = BlogTimeline::new();
        let page = Page::new(
            "test_page".to_string(),
            "test_content".to_string(),
            DocHeader {
                title: "Test Title".to_string(),
                date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                    .unwrap()
                    .into(),
                ..Default::default()
            },
        );
        blog_timeline.add_page(page.to_short("test_page".to_string()));
        assert_eq!(blog_timeline.pages.len(), 1);
    }

    #[test]
    fn test_blog_timeline_get_page() {
        let mut blog_timeline = BlogTimeline::new();
        let page = Page::new(
            "test_page".to_string(),
            "test_content".to_string(),
            DocHeader {
                title: "Test Title".to_string(),
                date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                    .unwrap()
                    .into(),
                ..Default::default()
            },
        );
        blog_timeline.add_page(page.to_short("test_page".to_string()));
        let page = blog_timeline.get_page("2024-07-16-22h51-Test_Title");
        assert!(page.is_some());
    }

    #[test]
    fn test_blog_timeline_generate_timeline_from_db() {
        let mut db_folder = DbFolder::new("Test Folder".to_string());
        let page = get_test_page();
        db_folder.add_page(page);
        let mut sub_folder = DbFolder::new("Sub Folder".to_string());
        let mut page = get_test_page();
        page.metadata.spec.blog = true;
        sub_folder.add_page(page);
        db_folder.add_sub_folder(sub_folder);
        let blog_timeline =
            BlogTimeline::generate_timeline_from_db(db_folder.clone(), "".to_string());
        assert_eq!(blog_timeline.pages.len(), 1);
        let short_page = blog_timeline
            .pages
            .get("2024-07-16-22h51-Test_Title")
            .unwrap();
        assert_eq!(short_page.name, "test_page");
        let full_page =
            db_folder.get_page_in_sub_folder_by_path(short_page.path.clone().to_lowercase());
        assert!(full_page.is_some());
        let full_page = full_page.unwrap();
        assert_eq!(full_page.name, short_page.name);
        assert_eq!(full_page.metadata, short_page.metadata);
    }

    #[test]
    fn test_blog_timeline_generate_timeline_from_db_2() {
        let mut db_folder = DbFolder::new("Test Folder".to_string());
        let mut page = get_test_page();
        page.metadata.spec.blog = true;
        db_folder.add_page(page);
        let mut sub_folder = DbFolder::new("Sub Folder".to_string());
        let page = get_test_page();
        sub_folder.add_page(page);
        db_folder.add_sub_folder(sub_folder);
        let blog_timeline = BlogTimeline::generate_timeline_from_db(db_folder, "".to_string());
        assert_eq!(blog_timeline.pages.len(), 1);
        let short_page = blog_timeline
            .pages
            .get("2024-07-16-22h51-Test_Title")
            .unwrap();
        assert_eq!(short_page.name, "test_page");
    }

    #[test]
    fn test_blog_timeline_generate_timeline_from_db_3() {
        let mut db_folder = DbFolder::new("Test Folder".to_string());
        let page = get_test_page();
        db_folder.add_page(page);
        let mut sub_folder = DbFolder::new("Sub Folder".to_string());
        let page = get_test_page();
        sub_folder.add_page(page);
        let mut sub_sub_folder = DbFolder::new("Sub Sub Folder".to_string());
        let mut page = get_test_page();
        page.metadata.spec.blog = true;
        sub_sub_folder.add_page(page);
        sub_folder.add_sub_folder(sub_sub_folder);
        db_folder.add_sub_folder(sub_folder);
        let blog_timeline = BlogTimeline::generate_timeline_from_db(db_folder, "".to_string());
        assert_eq!(blog_timeline.pages.len(), 1);
        let short_page = blog_timeline
            .pages
            .get("2024-07-16-22h51-Test_Title")
            .unwrap();
        assert_eq!(short_page.name, "test_page");
    }

    #[test]
    fn test_blog_timeline_to_schema() {
        let schema = BlogTimeline::schema();
        assert_eq!(schema.0, "BlogTimeline");
    }
}
