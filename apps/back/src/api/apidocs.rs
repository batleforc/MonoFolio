use crate::{
    api::{
        media::get_media::{self},
        page::{
            get_page::{self},
            get_page_v2::{self},
        },
    },
    home_profile::{HomeContent, HomeHistory, HomeHistoryUrl, HomeUrl},
};

use super::blog::get_timeline;
use super::doc::get_doc_sidebar;
use super::home::get_home;
use markdown_struct::{
    blog_timeline::BlogTimeline,
    content_struct::{Page, PageShort},
    doc_header::{DocHeader, DocHeaderLink, DocHeaderSpec, DocHeaderWriter},
    doc_sidebar::DocCategory,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "MonoFolio",
        version = "0.1.0",
        description = "This is the API that serves the content of the MonoFolio website to the front end, the MonoFolio idea is to have the Portfolio Front and Backend in a single repository.",
        contact(
            name = "Batleforc",
            email = "maxleriche.60@gmail.com",
            url = "https://maxleriche.net"
        )
    ),
    tags(

        ( name = "Blog", description = "Return the content of the blog timeline and page to be displayed."),
        ( name = "Doc", description = "Return the content of the documentation page, sidebar and header."),
        ( name = "Home", description = "Endpoints that return the content meant for the home page."),
        ( name = "Media", description = "Return the assets that are meant to be dynamically loaded and not a part of the static build."),
        ( name = "Page", description = "Return the content of a page with a specific workflow turning shortPage into a full page.")
    ),
    servers(
        (url = "http://localhost:5437", description = "Local development server."),
        (url = "https://maxleriche.net", description = "Production server referencing a tagged version of the api."),
        (url = "https://beta.maxleriche.net", description = "Beta server referencing the main branch.")
    ),
    components(
        schemas(
            PageShort,
            Page,
            DocHeader,
            DocHeaderSpec,
            DocHeaderLink,
            DocHeaderWriter,
            BlogTimeline,
            HomeUrl,
            HomeHistoryUrl,
            HomeHistory,
            HomeContent,
            DocCategory,
        )
    ),
    paths(
        get_timeline::get_timeline,
        get_page::get_page,
        get_doc_sidebar::get_doc_sidebar,
        get_home::get_home,
        get_media::get_media,
        get_page_v2::get_page_v2
    )
)]
pub struct ApiDocs;

#[cfg(test)]
mod tests {
    use utoipa::OpenApi;

    use super::*;

    #[test]
    fn test_api_docs_openapi() {
        let openapi = ApiDocs::openapi();
        assert_eq!(openapi.info.version, "0.1.0");
        assert_eq!(openapi.info.title, "MonoFolio");
        assert_eq!(
            openapi.info.description,
            Some("This is the API that serves the content of the MonoFolio website to the front end, the MonoFolio idea is to have the Portfolio Front and Backend in a single repository.".to_string())
        );
    }
}
