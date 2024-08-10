use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::page_database::DbFolder;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QuerryPage {
    pub path: String,
}

/// Get a page content
///
/// fetch page's content by page path in DbFolder
#[utoipa::path(
    tag = "Page",
    operation_id = "get_page",
    path = "/api/page",
    responses(
        (status = 200, description = "Page Content", body = Page),
        (status = 404, description = "Page not found"),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("info" = QuerryPage, Query,  description = "Page path"),
    )
)]
#[get("")]
#[instrument(name = "get_page")]
pub async fn get_page(
    db_folder: web::Data<DbFolder>,
    info: web::Query<QuerryPage>,
) -> impl Responder {
    if info.path.is_empty() {
        return HttpResponse::NotFound().finish();
    }
    if let Some(page) = db_folder.get_page_in_sub_folder_by_path(info.path.clone()) {
        return HttpResponse::Ok().json(page);
    } else {
        return HttpResponse::NotFound().finish();
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, Scope};
    use markdown_struct::content_struct::Page;
    use utoipa::{openapi::PathItemType, OpenApi};

    use super::*;

    #[actix_web::test]
    async fn test_get_page_not_found() {
        let db_folder = DbFolder::new("".to_string());
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(db_folder.clone()))
                .service(Scope::new("/api/page").service(get_page)),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/page?path=notfound")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

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
                (name = "Page", description = "Page related endpoints"),
                (name = "Media", description = "Media related endpoints")
            ),
            components(
                schemas(
                    Page,
                )
            ),
            paths(
                get_page,
            )
        )]
        struct ApiDocs;

        let openapi = ApiDocs::openapi();
        let api = openapi.paths.paths.get("/api/page").unwrap();
        let ope = api.operations.first_key_value().unwrap();
        assert!(ope.0.eq(&PathItemType::Get));
        assert!(ope.1.operation_id.eq(&Some("get_page".to_string())));
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
