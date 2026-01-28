use axum::extract::State;

use crate::{
    models::{CategoriesResponse, Category, Product, ProductsResponse},
    routes::AppState,
};

use axum::{Json, extract::Path, http::StatusCode};

pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<CategoriesResponse>, StatusCode> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, slug, description, specification_schema FROM categories ORDER BY name",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch categories: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CategoriesResponse { categories }))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Category>, StatusCode> {
    let category = sqlx::query_as::<_, Category>(
        "SELECT id, name, slug, description, specification_schema FROM categories WHERE slug = $1",
    )
    .bind(&slug)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch category {}: {}", slug, e);
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(category))
}

pub async fn list_products_by_category(
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
