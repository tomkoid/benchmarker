use axum::{Router, routing::get};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub mod about;
pub mod categories;
pub mod products;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/about", get(about::get_about))
        .route("/categories", get(categories::list_categories))
        .route("/categories/{slug}", get(categories::get_category))
        .route(
            "/categories/{slug}/products",
            get(categories::list_products_by_category),
        )
        .route(
            "/products",
            get(products::list_all_products).post(products::create_product),
        )
        .route("/products/search", get(products::search_products))
        .route("/products/compare", get(products::compare_products))
        .route("/products/{id}", get(products::get_product))
}
