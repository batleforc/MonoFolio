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
