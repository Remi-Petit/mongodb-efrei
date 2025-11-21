use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(controller::health_check) 
    );
}