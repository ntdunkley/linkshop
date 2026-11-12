use actix_web::{http::StatusCode, test};
use linkshop::{handlers::health::dto::HealthResponse, testing::TestContext};

#[actix_web::test]
async fn health_check_returns_200() {
    let ctx = TestContext::new().await;
    let app = test::init_service(ctx.app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: HealthResponse = test::read_body_json(resp).await;
    assert_eq!(body.status, "ok");
}
