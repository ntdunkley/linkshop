use actix_web::{http::StatusCode, test};
use linkshop::{
    build_app, error::ErrorResponse, handlers::menu::MenuResponse, testing::TestContext,
};

#[actix_web::test]
async fn returns_business_and_items_for_valid_slug() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let business = ctx
        .create_business(
            "Tasty Bites",
            "tasty-bites",
            Some("Fresh homecooked meals"),
            true,
        )
        .await;

    ctx.create_item(
        business.id,
        "Chicken Curry",
        Some("Mild curry with rice"),
        950, // 950 pence = £9.50
        true,
        1,
    )
    .await;

    ctx.create_item(
        business.id,
        "Samosa",
        Some("Crispy vegetable samosa"),
        250,
        true,
        2,
    )
    .await;

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    let req = test::TestRequest::get().uri("/b/tasty-bites").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: MenuResponse = test::read_body_json(resp).await;
    assert_eq!(body.business.name, "Tasty Bites");
    assert_eq!(body.business.slug, "tasty-bites");
    assert_eq!(
        body.business.description,
        Some("Fresh homecooked meals".to_string())
    );
    assert_eq!(body.items.len(), 2);

    assert_eq!(body.items[0].name, "Chicken Curry");
    assert_eq!(body.items[0].price, 950);
    assert!(body.items[0].is_available);

    assert_eq!(body.items[1].name, "Samosa");
    assert_eq!(body.items[1].price, 250);
    assert!(body.items[1].is_available);
}

#[actix_web::test]
async fn returns_404_for_unknown_slug() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    let req = test::TestRequest::get()
        .uri("/b/non-existent-shop")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let body: ErrorResponse = test::read_body_json(resp).await;
    assert_eq!(body.error, "not_found");
    assert!(body.message.contains("non-existent-shop"));
}

#[actix_web::test]
async fn returns_404_for_reserved_slugs() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    for reserved in ["admin", "api", "login"] {
        let req = test::TestRequest::get()
            .uri(&format!("/b/{reserved}"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.error, "not_found");
    }
}

#[actix_web::test]
async fn does_not_return_unpublished_businesses() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let business = ctx
        .create_business("Draft Cafe", "draft-cafe", None, false)
        .await;

    ctx.create_item(business.id, "Latte", None, 350, true, 1)
        .await;

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    let req = test::TestRequest::get().uri("/b/draft-cafe").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let body: ErrorResponse = test::read_body_json(resp).await;
    assert_eq!(body.error, "not_found");
}

#[actix_web::test]
async fn only_returns_data_for_requested_business() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let business_a = ctx
        .create_business("Pizza Place", "pizza-place", None, true)
        .await;
    let business_b = ctx
        .create_business("Burger Joint", "burger-joint", None, true)
        .await;

    ctx.create_item(business_a.id, "Margherita Pizza", None, 1100, true, 1)
        .await;
    ctx.create_item(business_b.id, "Cheeseburger", None, 850, true, 1)
        .await;

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    let req = test::TestRequest::get().uri("/b/pizza-place").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: MenuResponse = test::read_body_json(resp).await;

    assert_eq!(body.business.name, "Pizza Place");
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].name, "Margherita Pizza");
    assert_eq!(body.items[0].price, 1100);
}

#[actix_web::test]
async fn returns_unavailable_items_flagged_as_unavailable() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let business = ctx
        .create_business("Sweet Bakery", "sweet-bakery", None, true)
        .await;

    ctx.create_item(business.id, "Croissant", None, 250, true, 1)
        .await;
    ctx.create_item(
        business.id,
        "Chocolate Eclair",
        None,
        350,
        false, // Currently out of stock / unavailable
        2,
    )
    .await;

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    let req = test::TestRequest::get().uri("/b/sweet-bakery").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: MenuResponse = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 2);

    let croissant = body
        .items
        .iter()
        .find(|i| i.name == "Croissant")
        .expect("Croissant exists");
    assert!(croissant.is_available);
    assert_eq!(croissant.price, 250);

    let eclair = body
        .items
        .iter()
        .find(|i| i.name == "Chocolate Eclair")
        .expect("Eclair exists");
    assert!(!eclair.is_available);
    assert_eq!(eclair.price, 350);
}

#[actix_web::test]
async fn supports_api_route_aliases() {
    let Some(ctx) = TestContext::try_new().await else {
        eprintln!("PostgreSQL not reachable; skipping integration test");
        return;
    };

    let _business = ctx
        .create_business("Tea Room", "tea-room", None, true)
        .await;

    let app = test::init_service(build_app(ctx.pool.clone())).await;

    for path in [
        "/b/tea-room",
        "/api/b/tea-room",
        "/api/businesses/tea-room",
        "/api/businesses/tea-room/menu",
    ] {
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
