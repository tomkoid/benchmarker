use crate::models::*;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use bigdecimal::BigDecimal;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories))
        .route(
            "/categories/{slug}/products",
            get(list_products_by_category),
        )
        .route("/products", get(list_all_products).post(create_product))
        .route("/products/{id}", get(get_product))
}

async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<CategoriesResponse>, StatusCode> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, slug, description FROM categories ORDER BY name",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch categories: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CategoriesResponse { categories }))
}

async fn list_products_by_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ProductsResponse>, StatusCode> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT p.id, p.category_id, p.name, p.manufacturer, p.model, p.specifications, p.price
        FROM products p
        JOIN categories c ON p.category_id = c.id
        WHERE c.slug = $1
        ORDER BY p.name
        "#,
    )
    .bind(&slug)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch products for category {}: {}", slug, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ProductsResponse { products }))
}

async fn list_all_products(
    State(state): State<AppState>,
) -> Result<Json<ProductsResponse>, StatusCode> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, category_id, name, manufacturer, model, specifications, price
        FROM products
        ORDER BY name
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch all products: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ProductsResponse { products }))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductResponse>, StatusCode> {
    let product = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, category_id, name, manufacturer, model, specifications, price
        FROM products
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch product {}: {}", id, e);
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(ProductResponse { product }))
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<ProductResponse>), StatusCode> {
    let price = payload
        .price
        .map(|p| BigDecimal::from(p as i64) / BigDecimal::from(100));

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (category_id, name, manufacturer, model, specifications, price)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, category_id, name, manufacturer, model, specifications, price
        "#,
    )
    .bind(payload.category_id)
    .bind(&payload.name)
    .bind(&payload.manufacturer)
    .bind(&payload.model)
    .bind(&payload.specifications)
    .bind(price)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create product: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(ProductResponse { product })))
}
