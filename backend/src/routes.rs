use crate::models::*;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use bigdecimal::BigDecimal;
use serde_json::json;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories))
        .route("/categories/{slug}", get(get_category))
        .route("/categories/{slug}/products", get(list_products_by_category))
        .route("/products", get(list_all_products).post(create_product))
        .route("/products/search", get(search_products))
        .route("/products/compare", get(compare_products))
        .route("/products/{id}", get(get_product))
}

async fn list_categories(
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

async fn get_category(
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

async fn search_products(
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
    let min_price_decimal = query.min_price.map(|p| BigDecimal::from(p as i64) / BigDecimal::from(100));
    let max_price_decimal = query.max_price.map(|p| BigDecimal::from(p as i64) / BigDecimal::from(100));

    // Add search conditions
    if query.q.is_some() {
        sql.push_str(" AND (p.name ILIKE $1 OR p.manufacturer ILIKE $1 OR p.model ILIKE $1)");
    }
    if query.category.is_some() {
        sql.push_str(" AND c.slug = $2");
    }
    if query.manufacturer.is_some() {
        sql.push_str(" AND p.manufacturer ILIKE $3");
    }
    if query.min_price.is_some() {
        sql.push_str(" AND p.price >= $4");
    }
    if query.max_price.is_some() {
        sql.push_str(" AND p.price <= $5");
    }

    sql.push_str(" ORDER BY p.name");

    // Build and bind query
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

    let products = db_query
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to search products: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ProductsResponse { products }))
}

async fn compare_products(
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

fn build_comparison_table(category: &Category, products: &[Product]) -> Vec<ComparisonRow> {
    let mut rows = Vec::new();

    // Add basic info rows
    rows.push(ComparisonRow {
        field: "name".to_string(),
        label: "Product Name".to_string(),
        unit: "".to_string(),
        values: products
            .iter()
            .map(|p| ComparisonValue {
                product_id: p.id,
                product_name: p.name.clone(),
                value: json!(p.name.clone()),
            })
            .collect(),
    });

    rows.push(ComparisonRow {
        field: "manufacturer".to_string(),
        label: "Manufacturer".to_string(),
        unit: "".to_string(),
        values: products
            .iter()
            .map(|p| ComparisonValue {
                product_id: p.id,
                product_name: p.name.clone(),
                value: json!(p.manufacturer.clone().unwrap_or_default()),
            })
            .collect(),
    });

    rows.push(ComparisonRow {
        field: "price".to_string(),
        label: "Price".to_string(),
        unit: "$".to_string(),
        values: products
            .iter()
            .map(|p| ComparisonValue {
                product_id: p.id,
                product_name: p.name.clone(),
                value: json!(p.price.clone().map(|pr| pr.to_string()).unwrap_or_default()),
            })
            .collect(),
    });

    // Add specification rows based on schema
    if let Some(schema) = &category.specification_schema {
        if let Some(schema_obj) = schema.as_object() {
            for (field_name, field_def) in schema_obj {
                if let Some(field_obj) = field_def.as_object() {
                    let label = field_obj
                        .get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or(field_name)
                        .to_string();
                    let unit = field_obj
                        .get("unit")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let values = products
                        .iter()
                        .map(|p| {
                            let value = p
                                .specifications
                                .as_ref()
                                .and_then(|specs| specs.get(field_name))
                                .cloned()
                                .unwrap_or(json!(null));

                            ComparisonValue {
                                product_id: p.id,
                                product_name: p.name.clone(),
                                value,
                            }
                        })
                        .collect();

                    rows.push(ComparisonRow {
                        field: field_name.clone(),
                        label,
                        unit,
                        values,
                    });
                }
            }
        }
    }

    rows
}
