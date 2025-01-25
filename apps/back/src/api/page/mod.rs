use actix_web::Scope;

pub mod get_page;

pub mod get_page_v2;

pub fn init_page_api() -> Scope {
    Scope::new("/page").service(get_page::get_page)
}

pub fn init_page_v2_api() -> Scope {
    Scope::new("/page").service(get_page_v2::get_page_v2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{web, App};
    use markdown_struct::{
        content_struct::{Page, PageV2},
        doc_header::DocHeader,
        page_database::DbFolder,
    };

    #[actix_web::test]
    async fn test_init_page_api() {
        let mut db_folder = DbFolder::new("".to_string());
        db_folder
            .add_page(PageV2::new("test".to_string(), "".to_string(), DocHeader::default()).into());
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(db_folder.clone()))
                .service(Scope::new("/api").service(init_page_api())),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/page?path=test")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = actix_web::test::read_body(resp).await;
        assert_eq!(
            resp_body,
            serde_json::to_string(&Page::from(db_folder.get_page("test").unwrap())).unwrap()
        );
    }
}
