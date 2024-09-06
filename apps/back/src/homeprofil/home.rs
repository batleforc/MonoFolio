use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Content of the home page.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HomeContent {
    pub name: String,
    pub presentation: String,
    pub cover_title: Vec<String>,
    pub cv_url: String,
    pub url: Vec<HomeUrl>,
    pub history: Vec<HomeHistory>,
}

/// Url that should be present on the home page.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HomeUrl {
    pub url: String,
    pub name: String,
    pub primaire: bool,
    pub img_url: String,
}

/// History of the home page, contain a part of the history of Max Batleforc.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HomeHistory {
    pub title: String,
    pub lieux: String,
    pub date: String,
    pub weight: i32,
    pub img_url: String,
    pub description: String,
    pub url: Option<Vec<HomeHistoryUrl>>,
}

/// Url that should be present on the history.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HomeHistoryUrl {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum HomeLoadingError {
    FileNotFound(String),
    FileNotProperYaml(String),
}

pub fn load_home_content(base_path: &str) -> Result<HomeContent, HomeLoadingError> {
    let path = format!("{}/home.yaml", base_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(HomeLoadingError::FileNotFound(err.to_string())),
    };
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);
    let content: HomeContent = match serde_yaml::from_str(&buffer) {
        Ok(content) => content,
        Err(err) => return Err(HomeLoadingError::FileNotProperYaml(err.to_string())),
    };
    Ok(content)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_load_home_content() {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("../../test_dataset/content/");
        let content = load_home_content(path_buf.to_str().unwrap());
        println!("{:?}", content);
        assert!(content.is_ok());
        let content = content.unwrap();
        assert_eq!(content.name, "max truc");
        assert_eq!(content.presentation, "Aloha !\n");
        assert_eq!(
            content.cover_title,
            vec!["dev".to_string(), "ops".to_string()]
        );
    }

    #[test]
    fn test_load_home_content_file_not_found() {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("../../test_dataset/empty/");
        let content = load_home_content(path_buf.to_str().unwrap());
        assert!(content.is_err());
        let content = content.unwrap_err();
        assert_eq!(
            content,
            HomeLoadingError::FileNotFound("No such file or directory (os error 2)".to_string())
        );
    }

    #[test]
    fn test_load_home_content_file_not_proper_yaml() {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("../../test_dataset/bad_content/");
        let content = load_home_content(path_buf.to_str().unwrap());
        assert!(content.is_err());
    }

    #[test]
    fn test_home_serialize_deserialize() {
        let home = HomeContent {
            name: "max truc".to_string(),
            presentation: "Aloha !\n".to_string(),
            cover_title: vec!["dev".to_string(), "ops".to_string()],
            cv_url: "https://cv.com".to_string(),
            url: vec![HomeUrl {
                url: "https://cv.com".to_string(),
                name: "cv".to_string(),
                primaire: true,
                img_url: "https://cv.com".to_string(),
            }],
            history: vec![HomeHistory {
                title: "title".to_string(),
                lieux: "lieux".to_string(),
                date: "date".to_string(),
                weight: 1,
                img_url: "https://cv.com".to_string(),
                description: "description".to_string(),
                url: Some(vec![HomeHistoryUrl {
                    url: "https://cv.com".to_string(),
                    name: "cv".to_string(),
                }]),
            }],
        };
        let serialized = serde_yaml::to_string(&home).unwrap();
        let deserialized: HomeContent = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(home, deserialized);
    }

    #[test]
    fn test_home_toschema() {
        let schema_home = HomeContent::schema();
        let schema_home_url = HomeUrl::schema();
        let schema_home_history = HomeHistory::schema();
        let schema_home_history_url = HomeHistoryUrl::schema();
        assert_eq!(schema_home.0, "HomeContent".to_string());
        assert_eq!(schema_home_url.0, "HomeUrl".to_string());
        assert_eq!(schema_home_history.0, "HomeHistory".to_string());
        assert_eq!(schema_home_history_url.0, "HomeHistoryUrl".to_string());
    }
}
