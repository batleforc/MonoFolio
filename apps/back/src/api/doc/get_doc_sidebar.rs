use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::doc_sidebar::DocCategory;
use tracing::instrument;

/// Get doc sidebar
///
/// Get doc sidebar with minimal description of each article.
#[utoipa::path(
    tag = "Doc",
    operation_id = "get_doc_sidebar",
    path = "/api/doc",
    responses(
        (status = 200, description = "Doc category sidebar.", body = DocCategory),
        (status = 500, description = "Internal server error."),
    )
)]
#[get("")]
#[instrument(name = "get_doc_sidebar")]
pub async fn get_doc_sidebar(doc_sidebar: web::Data<DocCategory>) -> impl Responder {
    HttpResponse::Ok().json(doc_sidebar)
}
