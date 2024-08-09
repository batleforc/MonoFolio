use actix_web::Scope;

pub mod get_timeline;

pub fn init_blog_api() -> Scope {
    Scope::new("/blog").service(get_timeline::get_timeline)
}
