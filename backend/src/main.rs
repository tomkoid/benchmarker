use axum::{Json, Router, http::StatusCode, routing::get};
use sqlx::postgres::PgPoolOptions;

fn api_routes() -> Router {
    Router::new().route("/products", get(products))
}

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:xxx@localhost/skoda")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .nest("/api/v1", api_routes());
    // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn products() -> (StatusCode, Json<ProductsResponse>) {
    let products = ProductsResponse {
        products: vec![
            "Product 1".to_string(),
            "Product 2".to_string(),
            "Product 3".to_string(),
        ],
    };

    (StatusCode::OK, Json(products))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ProductsResponse {
    products: Vec<String>,
}
