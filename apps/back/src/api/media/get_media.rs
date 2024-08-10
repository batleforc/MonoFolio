use std::{fs, path::PathBuf};

use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QuerryMedia {
    pub path: String,
}

/// Get media from the media folder
///
/// Get media from the media folder
#[utoipa::path(
    tag = "Media",
    operation_id = "get_media",
    path = "/api/media",
    responses(
        (status = 200, description = "Media content"),
        (status = 404, description = "Page not found"),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("info" = QuerryMedia, Query,  description = "Page path"),
    )
)]
#[get("")]
#[instrument(name = "get_media")]
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
        None => HttpResponse::Ok().body(file),
    }
}

#[cfg(test)]
mod tests {
    use utoipa::{openapi::PathItemType, OpenApi};

    use super::*;

    #[test]
    fn test_get_timeline_openapi() {
        #[derive(utoipa::OpenApi)]
        #[openapi(
            info(
                title = "MonoFolio",
                version = "0.1.0",
                description = "API documentation for MonoFolio"
            ),
            tags(
                (name = "Blog", description = "Blog related endpoints"),
                (name = "Doc", description = "Doc related endpoints"),
                (name = "Media", description = "Media related endpoints")
            ),
            components(
                schemas(
                    QuerryMedia,
                )
            ),
            paths(
                get_media,
            )
        )]
        struct ApiDocs;

        let openapi = ApiDocs::openapi();
        let api = openapi.paths.paths.get("/api/media").unwrap();
        let ope = api.operations.first_key_value().unwrap();
        assert!(ope.0.eq(&PathItemType::Get));
        assert!(ope.1.operation_id.eq(&Some("get_media".to_string())));
    }

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
