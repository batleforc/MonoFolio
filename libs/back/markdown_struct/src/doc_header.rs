use chrono::{DateTime, Utc};
use markdown_header::parse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

fn true_default() -> bool {
    true
}
fn false_default() -> bool {
    false
}

fn empty_default() -> Vec<String> {
    Vec::new()
}

fn empty_vec_doc_header_link() -> Vec<DocHeaderLink> {
    Vec::new()
}

fn default_weight() -> i32 {
    0
}

fn empty_doc_header_spec() -> DocHeaderSpec {
    DocHeaderSpec {
        blog: false,
        project: false,
        doc: true,
    }
}

fn default_doc_header_writter() -> DocHeaderWritter {
    Default::default()
}

/// Specification of the header of a markdown file, include the information if a Header is a blog/project/doc.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct DocHeaderSpec {
    #[serde(default = "false_default")]
    pub blog: bool,
    #[serde(default = "false_default")]
    pub project: bool,
    #[serde(default = "true_default")]
    pub doc: bool,
}

impl Default for DocHeaderSpec {
    fn default() -> Self {
        DocHeaderSpec {
            blog: false,
            project: false,
            doc: true,
        }
    }
}

/// Link present in the header of a markdown file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct DocHeaderLink {
    pub name: String,
    pub url: String,
}

/// Writer present in the header of a markdown file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct DocHeaderWritter {
    pub name: String,
    pub url: String,
    pub avatar: String,
}

impl Default for DocHeaderWritter {
    fn default() -> Self {
        DocHeaderWritter {
            name: "Maxime".to_string(),
            url: "https://maxleriche.net".to_string(),
            avatar: "https://avatars.githubusercontent.com/u/24699592?s=80".to_string(),
        }
    }
}

/// Header of a markdown file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct DocHeader {
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    #[serde(default = "default_doc_header_writter")]
    pub writter: DocHeaderWritter,
    #[serde(default = "default_weight")]
    pub weight: i32,
    #[serde(default = "empty_doc_header_spec")]
    pub spec: DocHeaderSpec,
    #[serde(default = "empty_default")]
    pub tags: Vec<String>,
    #[serde(default = "empty_default")]
    pub techno: Vec<String>,
    #[serde(default = "empty_vec_doc_header_link")]
    pub links: Vec<DocHeaderLink>,
    pub image: Option<String>,
}

impl Default for DocHeader {
    fn default() -> Self {
        DocHeader {
            title: "".to_string(),
            date: Utc::now(),
            description: None,
            writter: DocHeaderWritter {
                name: "".to_string(),
                url: "".to_string(),
                avatar: "".to_string(),
            },
            weight: 0,
            spec: DocHeaderSpec::default(),
            tags: Vec::new(),
            techno: Vec::new(),
            links: Vec::new(),
            image: None,
        }
    }
}

#[derive(Debug)]
pub enum DocHeaderParseError {
    ParseError(markdown_header::Error),
    PaseHeaderError(serde_yaml::Error),
}

impl DocHeader {
    pub fn parse_md(content: &str) -> Result<(DocHeader, String), DocHeaderParseError> {
        let md_file = match parse(content) {
            Ok(md) => md,
            Err(e) => return Err(DocHeaderParseError::ParseError(e)),
        };
        match serde_yaml::from_str::<DocHeader>(md_file.front_matter()) {
            Ok(md) => Ok((md, md_file.content().to_string())),
            Err(e) => Err(DocHeaderParseError::PaseHeaderError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_md() {
        let content = r#"
---
date: 2024-07-16T22:51:00Z
title: "Portfolio"
description: "Portfolio"
spec:
  blog: true
  project: true
  doc: true
tags:
 - project
 - discovery
 - easteregg
techno:
  - vue
  - rust
  - actix
  - nx
links:
  - name: "Portfolio"
    url: "https://maxleriche.net"
---

THIS IS A TEST
        "#;
        let (header, content) = DocHeader::parse_md(content).unwrap();
        assert_eq!(content, "THIS IS A TEST");
        assert_eq!(header.title, "Portfolio");
        assert_eq!(
            header.date,
            DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z").unwrap()
        );
        assert!(header.description.is_some());
        assert_eq!(header.description.unwrap(), "Portfolio");
        assert_eq!(header.weight, 0);
        assert_eq!(header.spec.blog, true);
        assert_eq!(header.spec.project, true);
        assert_eq!(header.spec.doc, true);
        assert_eq!(header.tags.len(), 3);
        assert_eq!(header.techno.len(), 4);
        assert_eq!(header.links.len(), 1);
        assert_eq!(
            header.links,
            vec![DocHeaderLink {
                name: "Portfolio".to_string(),
                url: "https://maxleriche.net".to_string(),
            }]
        );
        assert_eq!(header.tags, vec!["project", "discovery", "easteregg"]);
        assert_eq!(header.techno, vec!["vue", "rust", "actix", "nx"]);
    }

    #[test]
    fn test_parse_md_no_description_and_all_spec_false() {
        let content = r#"
---
date: 2024-07-16T22:51:00Z
title: "Portfolio"
spec:
  blog: false
  project: false
  doc: false
tags:
 - project
 - easteregg
techno:
  - vue
  - rust
links:
  - name: "Portfolio"
    url: "https://maxleriche.net"
  - name: "Home"
    url: "https://maxleriche.net"
---

THIS IS A TEST
        "#;
        let (header, content) = DocHeader::parse_md(content).unwrap();
        assert_eq!(content, "THIS IS A TEST");
        assert_eq!(header.title, "Portfolio");
        assert_eq!(
            header.date,
            DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z").unwrap()
        );
        assert!(header.description.is_none());
        assert_eq!(header.weight, 0);
        assert_eq!(header.spec.blog, false);
        assert_eq!(header.spec.project, false);
        assert_eq!(header.spec.doc, false);
        assert_eq!(header.tags.len(), 2);
        assert_eq!(header.techno.len(), 2);
        assert_eq!(header.links.len(), 2);
        assert_eq!(
            header.links,
            vec![
                DocHeaderLink {
                    name: "Portfolio".to_string(),
                    url: "https://maxleriche.net".to_string(),
                },
                DocHeaderLink {
                    name: "Home".to_string(),
                    url: "https://maxleriche.net".to_string(),
                }
            ]
        );
        assert_eq!(header.tags, vec!["project", "easteregg"]);
        assert_eq!(header.techno, vec!["vue", "rust"]);
    }

    #[test]
    fn no_spec_minimal() {
        let content = r#"
---
date: 2024-07-16T22:51:00Z
title: "Portfolio"
---

THIS IS A TEST
        "#;
        let (header, content) = DocHeader::parse_md(content).unwrap();
        assert_eq!(content, "THIS IS A TEST");
        assert_eq!(header.title, "Portfolio");
        assert_eq!(
            header.date,
            DateTime::parse_from_rfc3339("2024-07-16T22:51:00Z").unwrap()
        );
        assert!(header.description.is_none());
        assert_eq!(header.weight, 0);
        assert_eq!(header.spec.blog, false);
        assert_eq!(header.spec.project, false);
        assert_eq!(header.spec.doc, true);
        assert_eq!(header.tags.len(), 0);
        assert_eq!(header.techno.len(), 0);
        assert_eq!(header.links.len(), 0);
    }

    #[test]
    fn not_a_markdown() {
        let content = r#""#;
        let result = DocHeader::parse_md(content);
        assert!(result.is_err());
        assert!(matches!(result, Err(DocHeaderParseError::ParseError(_))));
    }
    #[test]
    fn not_a_yaml() {
        let content = r#"
        ---
        truc truc
        ---
        # Hello
        "#;
        let result = DocHeader::parse_md(content);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(DocHeaderParseError::PaseHeaderError(_))
        ));
        // Test if the error is a ParseHeaderError
    }

    #[test]
    fn test_default() {
        assert_eq!(true_default(), true);
        assert_eq!(false_default(), false);
        assert_eq!(empty_vec_doc_header_link(), Vec::new());
        assert_eq!(default_weight(), 0);
        assert_eq!(
            empty_doc_header_spec(),
            DocHeaderSpec {
                blog: false,
                project: false,
                doc: true,
            }
        );
    }
}
