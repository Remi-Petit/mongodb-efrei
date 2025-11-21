use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(controller::health_check)
            .service(controller::get_all_games)
            .service(controller::get_game)
            .service(controller::create_game)
            .service(controller::update_game)
            .service(controller::delete_game)
            .service(controller::get_stats)
            .service(controller::toggle_favorite)
    );
}