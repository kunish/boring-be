use crate::api;
use actix_web::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(api::retrieve)
        .service(api::create)
        .service(api::update_title)
        .service(api::update_body)
        .service(api::publish)
        .service(api::unpublish)
        .service(api::delete);
}
