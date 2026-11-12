pub mod config;
pub mod domain;
pub mod error;
pub mod handlers;
pub mod repos;
pub mod telemetry;
pub mod testing;

use actix_web::{
    App,
    body::BoxBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use sqlx::PgPool;

use crate::repos::{PgBusinessRepository, PgItemRepository};

pub fn build_app(
    pool: PgPool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let business_repo = PgBusinessRepository::new(pool.clone());
    let item_repo = PgItemRepository::new(pool.clone());

    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(business_repo))
        .app_data(web::Data::new(item_repo))
        .configure(handlers::register_routes)
}
