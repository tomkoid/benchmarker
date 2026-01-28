use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct AboutResponse {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub async fn get_about() -> Json<AboutResponse> {
    Json(AboutResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Product Comparison API - Compare products across different categories with detailed specifications".to_string(),
    })
}
