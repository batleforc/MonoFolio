use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::content_struct::Page;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlogTimeline {
    pub pages: HashMap<String, Page>,
}

#[derive(Debug)]
pub enum BlogTimelineGenerationError {}

impl BlogTimeline {
    pub fn new() -> Self {
        BlogTimeline {
            pages: HashMap::new(),
        }
    }

    pub fn generate_timeline() -> Result<BlogTimeline, BlogTimelineGenerationError> {
        Ok(BlogTimeline::new())
    }

    pub fn add_page(&mut self, page: Page) {
        if self.pages.contains_key(&page.metadata.date) {
            panic!("Duplicate page date: {}", page.metadata.date);
        }
        self.pages.insert(page.metadata.date.to_string(), page);
    }

    pub fn get_page(&self, date: &str) -> Option<&Page> {
        self.pages.get(date)
    }
}
