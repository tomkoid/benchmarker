use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
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
