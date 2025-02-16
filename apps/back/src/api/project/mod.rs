use actix_web::Scope;

pub mod get_project_list;

pub fn init_project_api() -> Scope {
    Scope::new("/projects").service(get_project_list::get_project_list)
}
