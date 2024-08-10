use actix_web::{get, web, HttpResponse, Responder};
use tracing::instrument;

use crate::homeprofil::HomeContent;

/// Get home content
///
/// Get home page content
#[utoipa::path(
    tag = "Home",
    operation_id = "get_home",
    path = "/api/home",
    responses(
        (status = 200, description = "Home Content", body = HomeContent),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
#[instrument(name = "get_home")]
pub async fn get_home(home_content: web::Data<HomeContent>) -> impl Responder {
    HttpResponse::Ok().json(home_content)
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
                    HomeContent,
                )
            ),
            paths(
                get_home,
            )
        )]
        struct ApiDocs;

        let openapi = ApiDocs::openapi();
        let api = openapi.paths.paths.get("/api/home").unwrap();
        let ope = api.operations.first_key_value().unwrap();
        assert!(ope.0.eq(&PathItemType::Get));
        assert!(ope.1.operation_id.eq(&Some("get_home".to_string())));
    }
}
