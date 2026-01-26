use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

pub async fn ensure_database_exists(database_url: &str) -> color_eyre::Result<()> {
    // Parse the database URL to extract the database name
    let url_parts: Vec<&str> = database_url.rsplitn(2, '/').collect();
    if url_parts.len() != 2 {
        return Err(color_eyre::eyre::eyre!("Invalid DATABASE_URL format"));
    }

    let db_name = url_parts[0].split('?').next().unwrap();
    let base_url = url_parts[1];

    // Connect to the default postgres database
    let postgres_url = format!("{}/postgres", base_url);

    tracing::info!("Checking if database '{}' exists...", db_name);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&postgres_url)
        .await?;

    // Check if database exists
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
        .bind(db_name)
        .fetch_one(&pool)
        .await?
        .get(0);

    if !exists {
        tracing::info!("Database '{}' does not exist, creating...", db_name);
        // Note: Cannot use parameterized query for CREATE DATABASE
        let query = format!("CREATE DATABASE {}", db_name);
        sqlx::query(&query).execute(&pool).await?;
        tracing::info!("Database '{}' created successfully", db_name);
    } else {
        tracing::info!("Database '{}' already exists", db_name);
    }

    pool.close().await;
    Ok(())
}
