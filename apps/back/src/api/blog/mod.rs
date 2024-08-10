use actix_web::Scope;

pub mod get_timeline;

pub fn init_blog_api() -> Scope {
    Scope::new("/blog").service(get_timeline::get_timeline)
}

#[cfg(test)]
mod tests {
    use actix_web::{web, App, Scope};
    use markdown_struct::blog_timeline::BlogTimeline;

    use super::*;

    #[actix_web::test]
    async fn test_get_timeline() {
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(BlogTimeline::default()))
                .service(Scope::new("/api").service(init_blog_api())),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/blog")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = actix_web::test::read_body(resp).await;
        assert_eq!(resp_body, "{\"pages\":{}}");
    }
}
