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
