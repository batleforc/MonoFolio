use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DocHeaderSpec {
    #[serde(default = "false_default")]
    blog: bool,
    #[serde(default = "false_default")]
    project: bool,
    #[serde(default = "true_default")]
    doc: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocHeaderLink {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocHeader {
    title: String,
    date: String,
    description: String,
    #[serde(default = "default_weight")]
    weight: i32,
    #[serde(default = "empty_doc_header_spec")]
    spec: DocHeaderSpec,
    #[serde(default = "empty_default")]
    tags: Vec<String>,
    #[serde(default = "empty_default")]
    techno: Vec<String>,
    #[serde(default = "empty_vec_doc_header_link")]
    links: Vec<DocHeaderLink>,
}
