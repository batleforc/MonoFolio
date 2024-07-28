use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    content_struct::Page,
    doc_header::{DocHeader, DocHeaderParseError},
    folder_struct::Folder,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlogTimeline {
    pub pages: HashMap<String, Page>,
}

#[derive(Debug)]
pub enum BlogTimelineGenerationError {
    MetadataParseError(DocHeaderParseError),
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

    pub fn generate_timeline(
        folder_struct: Folder,
    ) -> Result<BlogTimeline, BlogTimelineGenerationError> {
        let mut blog_timeline = BlogTimeline::new();
        for folder in folder_struct.folders {
            match BlogTimeline::generate_timeline(folder) {
                Ok(mut sub_blog_timeline) => {
                    blog_timeline.pages.extend(sub_blog_timeline.pages.drain());
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
                    Err(e) => return Err(BlogTimelineGenerationError::MetadataParseError(e)),
                };
                if !metadata.spec.blog {
                    continue;
                }
                blog_timeline.add_page(Page::new(file.name, content, metadata));
            }
        }
        Ok(blog_timeline)
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.insert(page.url_encode_name(), page);
    }

    pub fn get_page(&self, page_id: &str) -> Option<&Page> {
        self.pages.get(page_id)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::markdown::folder_struct::File;

    use super::*;

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

    pub fn get_test_folder_with_files_no_blog() -> Folder {
        let mut folder = get_folder_struct();
        folder.files.push(File {
            name: "test_file.md".to_string(),
            content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content".to_string(),
        });
        folder
    }

    pub fn get_test_folder_with_files_blog() -> Folder {
        let mut folder = get_folder_struct();
        folder.files.push(File {
            name: "test_file.md".to_string(),
            content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\nspec:\n  blog: true\n---\nTest content".to_string(),
        });
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

    #[test]
    fn test_blog_timeline_new() {
        let blog_timeline = BlogTimeline::new();
        assert_eq!(blog_timeline.pages.len(), 0);
    }

    #[test]
    fn test_blog_timeline_invalid_files() {
        let folder = get_test_folder_with_invalid_files();
        let blog_timeline = BlogTimeline::generate_timeline(folder);
        assert!(blog_timeline.is_err());
    }

    #[test]
    fn test_blog_generate_timeline_with_empty_folder() {
        let folder = get_folder_struct();
        let blog_timeline = BlogTimeline::generate_timeline(folder).unwrap();
        assert_eq!(blog_timeline.pages.len(), 0);
    }

    #[test]
    fn test_blog_timeline_with_invalid_sub_folder() {
        let folder = get_test_sub_folder_invalid_files();
        let blog_timeline = BlogTimeline::generate_timeline(folder);
        assert!(blog_timeline.is_err());
    }
    #[test]
    fn test_blog_generate_timeline_with_files() {
        let folder = get_test_folder_with_files_blog();
        let blog_timeline = BlogTimeline::generate_timeline(folder).unwrap();
        assert_eq!(blog_timeline.pages.len(), 1);
    }

    #[test]
    fn test_blog_timeline_no_md_file() {
        let folder = get_test_folder_with_no_md_file();
        let blog_timeline = BlogTimeline::generate_timeline(folder).unwrap();
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
    fn test_blog_generate_timeline_with_files_no_blog() {
        let folder = get_test_folder_with_files_no_blog();
        let blog_timeline = BlogTimeline::generate_timeline(folder).unwrap();
        assert_eq!(blog_timeline.pages.len(), 0);
    }

    #[test]
    fn test_blog_generate_timeline_with_empty_folders() {
        let folder = get_test_folder_with_empty_folders();
        let blog_timeline = BlogTimeline::generate_timeline(folder).unwrap();
        assert_eq!(blog_timeline.pages.len(), 0);
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
                description: None,
                weight: 0,
                spec: Default::default(),
                tags: vec![],
                techno: vec![],
                links: vec![],
            },
        );
        blog_timeline.add_page(page);
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
                description: None,
                weight: 0,
                spec: Default::default(),
                tags: vec![],
                techno: vec![],
                links: vec![],
            },
        );
        blog_timeline.add_page(page);
        let page = blog_timeline.get_page("2024-07-16-22h51-Test_Title");
        assert!(page.is_some());
    }
}
