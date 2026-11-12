pub mod dto;

use actix_web::HttpResponse;

pub use dto::HealthResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}
