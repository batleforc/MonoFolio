use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::{content_struct::PageV2, page_database::DbFolder};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::IntoParams;

/// Querry to get a page content.
#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct QuerryPage {
    /// Path to the markdown page.
    pub path: String,
}

/// Get a page content in V2
///
/// Fetch page's content by page path in DbFolder, return page in the V2 format.
#[utoipa::path(
    tag = "Page",
    operation_id = "get_page_v2",
    path = "/api/v2/page",
    responses(
        (status = 200, description = "Content of a full page if found.", body = PageV2),
        (status = 404, description = "Page not found or path invalid."),
        (status = 500, description = "Internal server error."),
    ),
    params(
        QuerryPage,
    )
)]
#[get("")]
#[instrument(name = "get_page_v2")]
pub async fn get_page_v2(
    db_folder: web::Data<DbFolder>,
    info: web::Query<QuerryPage>,
) -> impl Responder {
    if info.path.is_empty() {
        return HttpResponse::NotFound().finish();
    }
    if let Some(page) = db_folder.get_page_in_sub_folder_by_path(info.path.clone().to_lowercase()) {
        return HttpResponse::Ok().json(page);
    } else {
        return HttpResponse::NotFound().finish();
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, Scope};

    use super::*;

    #[actix_web::test]
    async fn test_get_page_not_found() {
        let db_folder = DbFolder::new("".to_string());
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(db_folder.clone()))
                .service(Scope::new("/api/page").service(get_page_v2)),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/page?path=notfound")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[test]
    fn test_querry_page_serialize_deserialize() {
        let querry = QuerryPage {
            path: "test".to_string(),
        };
        let json = serde_json::to_string(&querry).unwrap();
        assert_eq!(json, "{\"path\":\"test\"}");
        let de_querry: QuerryPage = serde_json::from_str(&json).unwrap();
        assert_eq!(de_querry.path, "test");
    }
}
