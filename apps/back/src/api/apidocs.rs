use super::blog::get_timeline;
use markdown_struct::{
    blog_timeline::BlogTimeline,
    content_struct::{Page, PageShort},
    doc_header::{DocHeader, DocHeaderLink, DocHeaderSpec},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "MonoFolio",
        version = "0.1.0",
        description = "API documentation for MonoFolio"
    ),
    tags(
        ( name = "Blog", description = "Blog related endpoints"),
        ( name = "Doc", description = "Doc related endpoints"),
        ( name = "Media", description = "Media related endpoints")
    ),
    components(
        schemas(
            PageShort,
            Page,
            DocHeader,
            DocHeaderSpec,
            DocHeaderLink,
            BlogTimeline,
        )
    ),
    paths(
        get_timeline::get_timeline,
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
            Some("API documentation for MonoFolio".to_string())
        );
    }
}
