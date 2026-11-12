use actix_web::{http::StatusCode, test};
use linkshop::{build_app, handlers::health::health_check, testing::TestContext};

#[actix_web::test]
async fn health_check_returns_200() {
    // If PostgreSQL test database is available, test with the full app
    if let Some(ctx) = TestContext::try_new().await {
        let app = test::init_service(build_app(ctx.pool)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    } else {
        // Fallback unit test of health handler
        let resp = health_check().await;
        let status = resp.status();
        assert_eq!(status, StatusCode::OK);
    }
}
