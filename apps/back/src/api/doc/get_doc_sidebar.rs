use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::doc_sidebar::DocCategory;
use tracing::instrument;

/// Get doc sidebar
///
/// Get doc sidebar with minimal description of each article
#[utoipa::path(
    tag = "Doc",
    operation_id = "get_doc_sidebar",
    path = "/api/doc",
    responses(
        (status = 200, description = "Doc Sidebar", body = DocCategory),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
#[instrument(name = "get_doc_sidebar")]
pub async fn get_doc_sidebar(doc_sidebar: web::Data<DocCategory>) -> impl Responder {
    HttpResponse::Ok().json(doc_sidebar)
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
                    DocCategory,
                )
            ),
            paths(
                get_doc_sidebar,
            )
        )]
        struct ApiDocs;

        let openapi = ApiDocs::openapi();
        let api = openapi.paths.paths.get("/api/doc").unwrap();
        let ope = api.operations.first_key_value().unwrap();
        assert!(ope.0.eq(&PathItemType::Get));
        assert!(ope.1.operation_id.eq(&Some("get_doc_sidebar".to_string())));
    }
}
