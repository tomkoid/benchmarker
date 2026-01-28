use serde_json::json;

use crate::models::{Category, ComparisonRow, ComparisonValue, Product};

pub fn build_comparison_table(category: &Category, products: &[Product]) -> Vec<ComparisonRow> {
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
