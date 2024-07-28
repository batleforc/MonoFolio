use serde::{Deserialize, Serialize};

use super::{
    content_struct::Page,
    doc_header::{DocHeader, DocHeaderParseError},
    folder_struct::Folder,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocCategory {
    pub name: String,
    pub has_index: bool,
    pub sub_categories: Vec<DocCategory>,
    pub pages: Vec<Page>,
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

    pub fn generate_sidebar(
        folder_struct: Folder,
    ) -> Result<DocCategory, DocCategoryGenerateError> {
        let mut doc_category = DocCategory::new(folder_struct.name);
        for folder in folder_struct.folders {
            match DocCategory::generate_sidebar(folder) {
                Ok(sub_category) => doc_category.sub_categories.push(sub_category),
                Err(e) => return Err(e),
            }
        }
        for file in folder_struct.files {
            if file.name.ends_with(".md") {
                let (metadata, content) = match DocHeader::parse_md(&file.content) {
                    Ok((metadata, content)) => (metadata, content),
                    Err(e) => return Err(DocCategoryGenerateError::MetadataParseError(e)),
                };
                if metadata.spec.blog {
                    continue;
                }
                doc_category.pages.push(Page {
                    name: file.name.to_string(),
                    content,
                    metadata,
                });
                if file.name == "index.md" {
                    doc_category.has_index = true;
                }
            }
        }
        Ok(doc_category)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::markdown::folder_struct::File;

    use super::*;

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
    }

    #[test]
    fn test_doc_category_generate_sidebar_markdown_metadata_error() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: vec![
                File {
                    name: "index.md".to_string(),
                    content: "# Test content".to_string(),
                },
                File {
                    name: "test.md".to_string(),
                    content: "# Test content".to_string(),
                },
            ],
            folders: Vec::new(),
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct);
        assert!(doc_category.is_err());
    }
    #[test]
    fn test_doc_category_basic() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: vec![
                File {
                    name: "index.md".to_string(),
                    content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                        .to_string(),
                },
                File {
                    name: "test.md".to_string(),
                    content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                        .to_string(),
                },
            ],
            folders: Vec::new(),
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct).unwrap();
        assert_eq!(doc_category.name, "test");
        assert_eq!(doc_category.has_index, true);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 2);
        assert_eq!(doc_category.pages[0].name, "index.md");
        assert_eq!(doc_category.pages[1].name, "test.md");
        assert_eq!(doc_category.pages[0].content, "Test content");
        assert_eq!(doc_category.pages[1].content, "Test content");
    }
    #[test]
    fn test_doc_category_blog_shouldnt_appear() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: vec![
                File {
                    name: "index.md".to_string(),
                    content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\nspec:\n  blog: true\n---\nTest content"
                        .to_string(),
                },
                File {
                    name: "test.md".to_string(),
                    content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\nspec:\n  blog: true\n---\nTest content"
                        .to_string(),
                },
            ],
            folders: Vec::new(),
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct).unwrap();
        assert_eq!(doc_category.name, "test");
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 0);
    }
    #[test]
    fn test_doc_category_sub_category() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: Vec::new(),
            folders: vec![
                Folder {
                    name: "sub".to_string(),
                    files: vec![
                        File {
                            name: "index.md".to_string(),
                            content:
                                "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                                    .to_string(),
                        },
                        File {
                            name: "test.md".to_string(),
                            content:
                                "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                                    .to_string(),
                        },
                    ],
                    folders: Vec::new(),
                },
                Folder {
                    name: "sub2".to_string(),
                    files: vec![
                        File {
                            name: "index.md".to_string(),
                            content:
                                "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                                    .to_string(),
                        },
                        File {
                            name: "test.md".to_string(),
                            content:
                                "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                                    .to_string(),
                        },
                    ],
                    folders: Vec::new(),
                },
            ],
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct).unwrap();
        assert_eq!(doc_category.name, "test");
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 2);
        assert_eq!(doc_category.pages.len(), 0);
        assert_eq!(doc_category.sub_categories[0].name, "sub");
        assert_eq!(doc_category.sub_categories[1].name, "sub2");
        assert_eq!(doc_category.sub_categories[0].has_index, true);
        assert_eq!(doc_category.sub_categories[1].has_index, true);
        assert_eq!(doc_category.sub_categories[0].pages.len(), 2);
        assert_eq!(doc_category.sub_categories[1].pages.len(), 2);
    }

    #[test]
    fn test_doc_category_invalid_sub_category() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: Vec::new(),
            folders: vec![Folder {
                name: "sub".to_string(),
                files: vec![
                    File {
                        name: "index.md".to_string(),
                        content: "Test content".to_string(),
                    },
                    File {
                        name: "test.md".to_string(),
                        content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                            .to_string(),
                    },
                ],
                folders: Vec::new(),
            }],
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct);
        assert!(doc_category.is_err());
    }
    #[test]
    fn test_doc_category_not_a_markdown_file() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: vec![
                File {
                    name: "index.md".to_string(),
                    content: "---\ntitle: Test\ndate: 2024-07-16T22:51:00Z\n---\nTest content"
                        .to_string(),
                },
                File {
                    name: "test.txt".to_string(),
                    content: "Test content".to_string(),
                },
            ],
            folders: Vec::new(),
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct).unwrap();
        assert_eq!(doc_category.name, "test");
        assert_eq!(doc_category.has_index, true);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 1);
        assert_eq!(doc_category.pages[0].name, "index.md");
        assert_eq!(doc_category.pages[0].content, "Test content");
    }
    #[test]
    fn test_doc_category_empty_folder() {
        let folder_struct = Folder {
            name: "test".to_string(),
            files: Vec::new(),
            folders: Vec::new(),
        };
        let doc_category = DocCategory::generate_sidebar(folder_struct).unwrap();
        assert_eq!(doc_category.name, "test");
        assert_eq!(doc_category.has_index, false);
        assert_eq!(doc_category.sub_categories.len(), 0);
        assert_eq!(doc_category.pages.len(), 0);
    }

    #[test]
    fn test_doc_category_deserialize() {
        let doc_category = DocCategory {
            name: "test".to_string(),
            has_index: true,
            sub_categories: vec![DocCategory {
                name: "sub".to_string(),
                has_index: true,
                sub_categories: Vec::new(),
                pages: vec![Page {
                    name: "index.md".to_string(),
                    content: "Test content".to_string(),
                    metadata: DocHeader {
                        title: "Test".to_string(),
                        date: DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z")
                            .unwrap()
                            .into(),
                        description: None,
                        weight: 0,
                        spec: Default::default(),
                        tags: Vec::new(),
                        techno: Vec::new(),
                        links: Vec::new(),
                    },
                }],
            }],
            pages: Vec::new(),
        };
        let serialized = serde_json::to_string(&doc_category).unwrap();
        let deserialized: DocCategory = serde_json::from_str(&serialized).unwrap();
        assert_eq!(doc_category, deserialized);
    }
}
