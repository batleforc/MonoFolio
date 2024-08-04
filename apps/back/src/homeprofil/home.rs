use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HomeContent {
    pub name: String,
    pub presentation: String,
    pub cover_title: Vec<String>,
    pub cv_url: String,
    pub url: Vec<HomeUrl>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HomeUrl {
    pub url: String,
    pub name: String,
    pub primaire: bool,
    pub img_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HomeHistory {
    pub title: String,
    pub lieux: String,
    pub date: String,
    pub weight: i32,
    pub img_url: String,
    pub description: String,
    pub url: Option<Vec<HomeHistoryUrl>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HomeHistoryUrl {
    pub url: String,
    pub name: String,
}
