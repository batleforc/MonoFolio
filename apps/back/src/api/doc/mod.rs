use actix_web::Scope;

pub mod get_doc_sidebar;

pub fn init_doc_api() -> Scope {
    Scope::new("/doc").service(get_doc_sidebar::get_doc_sidebar)
}

#[cfg(test)]
mod tests {
    use actix_web::{web, App, Scope};
    use markdown_struct::doc_sidebar::DocCategory;

    use super::*;

    #[actix_web::test]
    async fn test_get_doc_sidebar() {
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(DocCategory::default()))
                .service(Scope::new("/api").service(init_doc_api())),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/doc")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = actix_web::test::read_body(resp).await;
        assert_eq!(
            resp_body,
            "{\"name\":\"\",\"has_index\":false,\"sub_categories\":[],\"pages\":[]}"
        );
    }
}
