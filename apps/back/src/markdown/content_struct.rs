use serde::{Deserialize, Serialize};

use super::doc_header::DocHeader;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Page {
    pub name: String,
    pub content: String,
    pub metadata: DocHeader,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PageShort {
    pub name: String,
    pub metadata: DocHeader,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlogTimeline {
    pub pages: Vec<(String, Page)>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectList {
    pub pages: Vec<Page>,
}
