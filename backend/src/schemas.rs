use serde::{Deserialize, Serialize};
use std::collections::HashMap;


pub fn load_schemas() -> color_eyre::Result<CategorySchemas> {
    let schema_content = include_str!("schemas/schemas.yml");

    let schemas: CategorySchemas = serde_yaml::from_str(&schema_content)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse schemas.yml: {}", e))?;

    Ok(schemas)
}

pub async fn sync_categories(pool: &sqlx::PgPool) -> color_eyre::Result<()> {
    let schemas = load_schemas()?;

    for category in schemas.categories {
        let schema_json = serde_json::to_value(&category.specifications)?;

        sqlx::query(
            r#"
            INSERT INTO categories (name, slug, description, specification_schema)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (slug) 
            DO UPDATE SET 
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                specification_schema = EXCLUDED.specification_schema,
                updated_at = NOW()
            "#,
        )
        .bind(&category.name)
        .bind(&category.slug)
        .bind(&category.description)
        .bind(&schema_json)
        .execute(pool)
        .await?;

        tracing::info!("Synced category: {} ({})", category.name, category.slug);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct CategorySchemas {
    pub categories: Vec<CategoryDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryDefinition {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub specifications: HashMap<String, SpecificationField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecificationField {
    #[serde(rename = "type")]
    pub field_type: String,
    pub label: String,
    pub unit: String,
    pub higher_is_better: Option<bool>,
}
