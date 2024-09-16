use crate::handlers::user_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(user_handlers::create_user))
            .route(web::get().to(user_handlers::get_users)),
    );
    cfg.service(web::resource("/ws/").route(web::get().to(user_handlers::ws_index)));
}
