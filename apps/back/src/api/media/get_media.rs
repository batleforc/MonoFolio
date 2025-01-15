use std::{fs, path::PathBuf};

use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::IntoParams;

use crate::config::Config;

/// Querry to get media from the media folder.
#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct QuerryMedia {
    /// Path to the media asset.
    pub path: String,
}

/// Get media from the media folder
///
/// Get the media asset from the media folder.
#[utoipa::path(
    tag = "Media",
    operation_id = "get_media",
    path = "/api/media",
    responses(
        (status = 200, description = "Media content if found."),
        (status = 404, description = "Page not found inside of the Media folder or path invalid."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        QuerryMedia,
    )
)]
#[get("")]
#[instrument(name = "get_media", skip(config))]
pub async fn get_media(info: web::Query<QuerryMedia>, config: web::Data<Config>) -> impl Responder {
    let info_parsed = info.path.replace("../", "");
    if info_parsed.starts_with("/") {
        return HttpResponse::NotFound().body("Page not found");
    }
    if info_parsed.contains(".htaccess") {
        return HttpResponse::new(StatusCode::IM_A_TEAPOT);
    }
    let mut path = PathBuf::from(&config.clone().content_path);
    path.push("../media");
    path.push(info_parsed);
    info!("Path: {:?}", path);
    if !path.exists() || !path.is_file() {
        return HttpResponse::NotFound().body("Page not found");
    }
    let sub_path = path.to_str().unwrap();
    let file = match fs::read(sub_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Internal server error"),
    };
    match infer::get(&file.clone()) {
        Some(kind) => HttpResponse::Ok()
            .content_type(kind.mime_type().to_string())
            .body(file),
        None => {
            let ext = path.extension().unwrap().to_str().unwrap();
            match ext {
                "js" => {
                    return HttpResponse::Ok()
                        .content_type("application/javascript")
                        .body(file);
                }
                _ => HttpResponse::Ok().body(file),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_querry_media() {
        let querry = QuerryMedia {
            path: "test.png".to_string(),
        };
        let json = serde_json::to_string(&querry).unwrap();
        let de_querry: QuerryMedia = serde_json::from_str(&json).unwrap();
        assert_eq!(querry.path, de_querry.path);
    }
}
