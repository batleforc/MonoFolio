use actix_web::{web, App, HttpServer};
use markdown_struct::{
    blog_timeline::BlogTimeline, doc_sidebar::DocCategory, page_database::DbFolder,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::apidocs::ApiDocs;

pub async fn init_api(
    port: u16,
    db_folder: DbFolder,
    blog_timeline: BlogTimeline,
    doc_sidebar: DocCategory,
) -> Result<(), std::io::Error> {
    println!("Initializing API...");
    let mut openapi = ApiDocs::openapi();
    openapi.info.version = env!("CARGO_PKG_VERSION").to_string();
    info!("Starting API server on port {}", port);
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let swagger_ui =
            SwaggerUi::new("/api/docs/{_:.*}").url("/api/docs/docs.json", openapi.clone());
        App::new()
            .app_data(web::Data::new(db_folder.clone()))
            .app_data(web::Data::new(blog_timeline.clone()))
            .app_data(web::Data::new(doc_sidebar.clone()))
            .wrap(cors)
            .service(swagger_ui)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
