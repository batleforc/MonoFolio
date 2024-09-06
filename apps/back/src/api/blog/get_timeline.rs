use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::blog_timeline::BlogTimeline;
use tracing::instrument;

/// Get the blog timeline
///
/// Get blog timeline with minimal description of each article.
#[utoipa::path(
    tag = "Blog",
    operation_id = "get_timeline",
    path = "/api/blog",
    responses(
        (status = 200, description = "Content of the blog timeline ordered by date.", body = BlogTimeline),
        (status = 500, description = "Internal server error."),
    )
)]
#[get("")]
#[instrument(name = "get_timeline")]
pub async fn get_timeline(blog_timeline: web::Data<BlogTimeline>) -> impl Responder {
    HttpResponse::Ok().json(blog_timeline)
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
                    BlogTimeline,
                )
            ),
            paths(
                get_timeline,
            )
        )]
        struct ApiDocs;

        let openapi = ApiDocs::openapi();
        let api = openapi.paths.paths.get("/api/blog").unwrap();
        let ope = api.operations.first_key_value().unwrap();
        assert!(ope.0.eq(&PathItemType::Get));
        assert!(ope.1.operation_id.eq(&Some("get_timeline".to_string())));
    }
}
