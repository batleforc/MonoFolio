use actix_web::Scope;

pub mod get_media;

pub fn init_media_api() -> Scope {
    Scope::new("/media").service(get_media::get_media)
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;
    use actix_web::{http::StatusCode, web, App, Scope};

    #[actix_web::test]
    async fn test_get_media() {
        let config = Config {
            port: 5437,
            env: "test".to_string(),
            content_path: "../../test_dataset/content".to_string(),
            tracing: vec![],
        };

        let app = actix_web::test::init_service(
            App::new().service(
                Scope::new("/api")
                    .service(init_media_api())
                    .app_data(web::Data::new(config)),
            ),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/media?path=test.png")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
