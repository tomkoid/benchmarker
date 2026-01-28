use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub specification_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub specifications: Option<serde_json::Value>,
    pub price: Option<bigdecimal::BigDecimal>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub category_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub specifications: Option<serde_json::Value>,
    pub price: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ProductsResponse {
    pub products: Vec<Product>,
}

#[derive(Debug, Serialize)]
pub struct CategoriesResponse {
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub product: Product,
}

#[derive(Debug, Serialize)]
pub struct ComparisonResponse {
    pub category: Category,
    pub products: Vec<Product>,
    pub comparison_table: Vec<ComparisonRow>,
}

#[derive(Debug, Serialize)]
pub struct ComparisonRow {
    pub field: String,
    pub label: String,
    pub unit: String,
    pub values: Vec<ComparisonValue>,
}

#[derive(Debug, Serialize)]
pub struct ComparisonValue {
    pub product_id: i32,
    pub product_name: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CompareQuery {
    pub ids: String, // comma-separated product IDs
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,           // General search query
    pub category: Option<String>,     // Filter by category slug
    pub manufacturer: Option<String>, // Filter by manufacturer
    pub min_price: Option<f64>,      // Minimum price
    pub max_price: Option<f64>,      // Maximum price
}
