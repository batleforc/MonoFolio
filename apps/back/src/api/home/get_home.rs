use actix_web::{get, web, HttpResponse, Responder};
use tracing::instrument;

use crate::home_profile::HomeContent;

/// Get home content
///
/// Get the content meant for the home page.
#[utoipa::path(
    tag = "Home",
    operation_id = "get_home",
    path = "/api/home",
    responses(
        (status = 200, description = "Content of the home page.", body = HomeContent),
        (status = 500, description = "Internal server error."),
    )
)]
#[get("")]
#[instrument(name = "get_home")]
pub async fn get_home(home_content: web::Data<HomeContent>) -> impl Responder {
    HttpResponse::Ok().json(home_content)
}
