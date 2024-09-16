use actix_web::web;

pub mod user_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(user_routes::init));
}
