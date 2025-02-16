use actix_web::{get, web, HttpResponse, Responder};
use markdown_struct::project_list::ProjectList;
use tracing::instrument;

/// Get project list
///
/// Get project list with minimal description of each article.
#[utoipa::path(
    tag = "Project",
    operation_id = "get_project_list",
    path = "/api/projects",
    responses(
        (status = 200, description = "Project list.", body = ProjectList),
        (status = 500, description = "Internal server error."),
    )
)]
#[get("")]
#[instrument(name = "get_project_list")]
pub async fn get_project_list(project_list: web::Data<ProjectList>) -> impl Responder {
    HttpResponse::Ok().json(project_list)
}
