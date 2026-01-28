use axum::extract::State;

use crate::{
    compare::build_comparison_table,
    models::{
        Category, CompareQuery, ComparisonResponse, CreateProduct, Product, ProductResponse,
        ProductsResponse, SearchQuery,
    },
    routes::AppState,
};

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use bigdecimal::BigDecimal;

pub async fn list_all_products(
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

pub async fn get_product(
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

pub async fn create_product(
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

pub async fn search_products(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<ProductsResponse>, StatusCode> {
    // Build base query
    let mut sql = String::from(
        r#"
        SELECT DISTINCT p.id, p.category_id, p.name, p.manufacturer, p.model, p.specifications, p.price
        FROM products p
        LEFT JOIN categories c ON p.category_id = c.id
        WHERE 1=1
        "#,
    );

    let search_pattern = query.q.as_ref().map(|q| format!("%{}%", q));
    let manufacturer_pattern = query.manufacturer.as_ref().map(|m| format!("%{}%", m));
    let min_price_decimal = query
        .min_price
        .map(|p| BigDecimal::from(p as i64) / BigDecimal::from(100));
    let max_price_decimal = query
        .max_price
        .map(|p| BigDecimal::from(p as i64) / BigDecimal::from(100));

    // Track parameter count
    let mut param_count = 0;

    // Add search conditions dynamically with correct parameter numbers
    if search_pattern.is_some() {
        param_count += 1;
        sql.push_str(&format!(
            " AND (p.name ILIKE ${} OR p.manufacturer ILIKE ${} OR p.model ILIKE ${})",
            param_count, param_count, param_count
        ));
    }
    if query.category.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND c.slug = ${}", param_count));
    }
    if manufacturer_pattern.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND p.manufacturer ILIKE ${}", param_count));
    }
    if min_price_decimal.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND p.price >= ${}", param_count));
    }
    if max_price_decimal.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND p.price <= ${}", param_count));
    }

    sql.push_str(" ORDER BY p.name");

    // Build and bind query in the correct order
    let mut db_query = sqlx::query_as::<_, Product>(&sql);

    if let Some(ref pattern) = search_pattern {
        db_query = db_query.bind(pattern);
    }
    if let Some(ref category) = query.category {
        db_query = db_query.bind(category);
    }
    if let Some(ref pattern) = manufacturer_pattern {
        db_query = db_query.bind(pattern);
    }
    if let Some(ref min_price) = min_price_decimal {
        db_query = db_query.bind(min_price);
    }
    if let Some(ref max_price) = max_price_decimal {
        db_query = db_query.bind(max_price);
    }

    let products = db_query.fetch_all(&state.db).await.map_err(|e| {
        tracing::error!("Failed to search products: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ProductsResponse { products }))
}

pub async fn compare_products(
    State(state): State<AppState>,
    Query(query): Query<CompareQuery>,
) -> Result<Json<ComparisonResponse>, StatusCode> {
    // Parse product IDs from comma-separated string
    let product_ids: Vec<i32> = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if product_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Fetch products
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, category_id, name, manufacturer, model, specifications, price
        FROM products
        WHERE id = ANY($1)
        ORDER BY id
        "#,
    )
    .bind(&product_ids)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch products for comparison: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if products.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Ensure all products are from the same category
    let category_id = products[0].category_id;
    if !products.iter().all(|p| p.category_id == category_id) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Fetch category with schema
    let category = sqlx::query_as::<_, Category>(
        "SELECT id, name, slug, description, specification_schema FROM categories WHERE id = $1",
    )
    .bind(category_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch category: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Build comparison table
    let comparison_table = build_comparison_table(&category, &products);

    Ok(Json(ComparisonResponse {
        category,
        products,
        comparison_table,
    }))
}
