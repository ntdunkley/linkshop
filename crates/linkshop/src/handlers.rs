pub mod health;
pub mod menu;

use actix_web::web;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health::health_check))
        .route("/b/{slug}", web::get().to(menu::get_menu))
        .route("/api/b/{slug}", web::get().to(menu::get_menu))
        .route("/api/businesses/{slug}", web::get().to(menu::get_menu))
        .route("/api/businesses/{slug}/menu", web::get().to(menu::get_menu));
}
