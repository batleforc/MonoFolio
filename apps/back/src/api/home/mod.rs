use actix_web::Scope;

pub mod get_home;

pub fn init_home_api() -> Scope {
    Scope::new("/home").service(get_home::get_home)
}

#[cfg(test)]
mod tests {
    use actix_web::{web, App, Scope};

    use crate::home_profile::{HomeContent, HomeHistory, HomeHistoryUrl, HomeUrl};

    use super::*;

    #[actix_web::test]
    async fn test_get_home() {
        let home_content = HomeContent {
            name: "max truc".to_string(),
            presentation: "Aloha !\n".to_string(),
            cover_title: vec!["dev".to_string(), "ops".to_string()],
            short_description: "short description".to_string(),
            cv_url: "https://cv.com".to_string(),
            url: vec![HomeUrl {
                url: "https://cv.com".to_string(),
                name: "cv".to_string(),
                primaire: true,
                img_url: "https://cv.com".to_string(),
            }],
            history: vec![HomeHistory {
                title: "title".to_string(),
                lieux: "lieux".to_string(),
                date: "date".to_string(),
                weight: 1,
                img_url: "https://cv.com".to_string(),
                ico_url: Some("https://cv.com".to_string()),
                description: "description".to_string(),
                url: Some(vec![HomeHistoryUrl {
                    url: "https://cv.com".to_string(),
                    name: "cv".to_string(),
                }]),
            }],
        };
        let app = actix_web::test::init_service(
            App::new()
                .app_data(web::Data::new(home_content.clone()))
                .service(Scope::new("/api").service(init_home_api())),
        )
        .await;

        let req = actix_web::test::TestRequest::get()
            .uri("/api/home")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = actix_web::test::read_body(resp).await;
        assert_eq!(resp_body, serde_json::to_string(&home_content).unwrap());
    }
}
