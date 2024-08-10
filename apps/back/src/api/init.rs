use actix_web::{
    dev::Service, get, http::header, web, App, HttpResponse, HttpServer, Responder, Scope,
};
use markdown_struct::{
    blog_timeline::BlogTimeline, doc_sidebar::DocCategory, page_database::DbFolder,
};
use tracing::info;
use tracing_actix_web::{RequestId, TracingLogger};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{apidocs::ApiDocs, blog::init_blog_api, doc::init_doc_api, page::init_page_api};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Monofolio!")
}

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
            .service(
                Scope::new("/api")
                    .service(init_blog_api())
                    .service(init_page_api())
                    .service(init_doc_api())
                    .service(hello)
                    .wrap_fn(|mut req, srv| {
                        let request_id_asc = req.extract::<RequestId>();
                        let fut = srv.call(req);
                        async move {
                            let mut res = fut.await?;
                            let request_id: RequestId = request_id_asc.await.unwrap();
                            let request_id_str = format!("{}", request_id);
                            let headers = res.headers_mut();
                            headers.insert(
                                header::HeaderName::from_static("x-request-id"),
                                header::HeaderValue::from_str(request_id_str.as_str()).unwrap(),
                            );
                            Ok(res)
                        }
                    })
                    .wrap(TracingLogger::default()),
            )
    })
    .bind(("0.0.0.0", port))?
    .bind(("::1", port))?
    .run()
    .await
}
