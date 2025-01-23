use crate::mdast_to_schema::Node;

use super::doc_header::DocHeader;
use markdown::ParseOptions;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A page containing the content, metadata and name of the page.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct Page {
    pub name: String,
    pub content: String,
    pub metadata: DocHeader,
}

/// V2 of the page struct, containing the metadata, path and commonmark content.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct PageV2 {
    pub name: String,
    content: String,
    pub metadata: DocHeader,
    pub parsed_content: Node,
}

impl From<Page> for PageV2 {
    fn from(page: Page) -> Self {
        PageV2 {
            name: page.name,
            content: page.content.clone(),
            metadata: page.metadata,
            parsed_content: markdown::to_mdast(&page.content, &ParseOptions::gfm())
                .unwrap()
                .into(),
        }
    }
}

impl From<PageV2> for Page {
    fn from(page: PageV2) -> Self {
        Page {
            name: page.name,
            content: page.content,
            metadata: page.metadata,
        }
    }
}

impl From<&Page> for PageV2 {
    fn from(page: &Page) -> Self {
        PageV2 {
            name: page.name.clone(),
            content: page.content.clone(),
            metadata: page.metadata.clone(),
            parsed_content: markdown::to_mdast(&page.content, &ParseOptions::gfm())
                .unwrap()
                .into(),
        }
    }
}

impl From<&PageV2> for Page {
    fn from(page: &PageV2) -> Self {
        Page {
            name: page.name.clone(),
            content: page.content.clone(),
            metadata: page.metadata.clone(),
        }
    }
}

impl PageV2 {
    pub fn new(name: String, content: String, metadata: DocHeader) -> Self {
        PageV2 {
            name,
            content: content.clone(),
            metadata,
            parsed_content: markdown::to_mdast(&content, &ParseOptions::gfm())
                .unwrap()
                .into(),
        }
    }

    pub fn url_encode_name(&self) -> String {
        format!(
            "{}-{}",
            self.metadata.date.format("%Y-%m-%d-%Hh%M"),
            self.metadata.title
        )
        .replace(' ', "_")
    }

    pub fn to_short(&self, path: String) -> PageShort {
        PageShort {
            name: self.name.clone(),
            path,
            metadata: self.metadata.clone(),
        }
    }
}

impl Page {
    pub fn new(name: String, content: String, metadata: DocHeader) -> Self {
        Page {
            name,
            content,
            metadata,
        }
    }

    pub fn url_encode_name(&self) -> String {
        format!(
            "{}-{}",
            self.metadata.date.format("%Y-%m-%d-%Hh%M"),
            self.metadata.title
        )
        .replace(' ', "_")
    }

    pub fn to_short(&self, path: String) -> PageShort {
        PageShort {
            name: self.name.clone(),
            path,
            metadata: self.metadata.clone(),
        }
    }
}

impl TryInto<PageShort> for Page {
    type Error = ();

    fn try_into(self) -> Result<PageShort, Self::Error> {
        Ok(PageShort {
            name: self.name,
            path: "".to_string(),
            metadata: self.metadata,
        })
    }
}

/// Short representation of a page, containing only the metadata and the path to the page.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, ToSchema)]
pub struct PageShort {
    pub name: String,
    pub path: String,
    pub metadata: DocHeader,
}

impl PageShort {
    pub fn url_encode_name(&self) -> String {
        format!(
            "{}-{}",
            self.metadata.date.format("%Y-%m-%d-%Hh%M"),
            self.metadata.title
        )
        .replace(' ', "_")
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;
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
    fn test_page_new() {
        let page = get_test_page();
        assert_eq!(page.name, "test_page".to_string());
        assert_eq!(page.content, "test_content".to_string());
        assert_eq!(page.metadata, get_test_doc_header());
    }

    #[test]
    fn test_page_url_encode_name() {
        let page = get_test_page();
        assert_eq!(
            page.url_encode_name(),
            "2024-07-16-22h51-Test_Title".to_string()
        );
    }

    #[test]
    fn test_page_try_into_page_short() {
        let page = get_test_page();
        let page_short: PageShort = page.try_into().unwrap();
        assert_eq!(page_short.name, "test_page".to_string());
        assert_eq!(page_short.metadata, get_test_doc_header());
    }

    #[test]
    fn test_page_serialize() {
        let page = get_test_page();
        let serialized = serde_json::to_string(&page).unwrap();
        let deserialized: Page = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page, deserialized);
        let page_short: PageShort = page.try_into().unwrap();
        let serialized = serde_json::to_string(&page_short).unwrap();
        let deserialized: PageShort = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_short, deserialized);
    }
}
