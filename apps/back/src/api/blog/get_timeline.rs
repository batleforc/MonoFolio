use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::blog_timeline::BlogTimeline;
use tracing::instrument;

#[utoipa::path(
    tag = "Blog",
    operation_id = "get_timeline",
    path = "/api/blog",
    responses(
        (status = 200, description = "Blog Timeline", body = BlogTimeline),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
#[instrument(name = "get_timeline")]
pub async fn get_timeline(blog_timeline: web::Data<BlogTimeline>) -> impl Responder {
    HttpResponse::Ok().json(blog_timeline)
}
