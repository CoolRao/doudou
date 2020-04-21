use actix_web::{web};

pub mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/message").route(web::post().to(handler::message_handler)))
        .service(web::resource("/login").route(web::post().to(handler::login_handler)))
        .service(web::resource("/index").route(web::post().to(handler::web_index)));
}