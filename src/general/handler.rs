use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};

use crate::AppState;
use std::sync::Arc;
use std::collections::HashMap;
use sqlx::{prelude::*, Column}; // Import Column trait
use tokio::io::AsyncWriteExt;

async fn get_table(table: &str, data: Arc<AppState>) -> Value  {
    let excludes = ["id","created_at","updated_at"];

    let query_result = sqlx::query(&format!("SELECT * FROM {} ORDER BY created_at", table))
        .fetch_all(&data.db)
        .await
        .unwrap(); // Assuming fetch_all always returns a result
    
    let rows: Vec<HashMap<String, Value>> = query_result
        .iter()
        .map(|row| {
            let mut map = HashMap::new();
            for (idx, col) in row.columns().iter().enumerate() {
                if excludes.iter().any(|&ex| col.name() == ex) {
                    continue; // Skip excluded columns
                }
                let value: Option<String> = row.try_get(idx).unwrap_or_default();
                map.insert(col.name().to_string(), json!(value));
            }
            map
        })
        .collect();

    json!(rows)
}

pub async fn update_translation_file(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let file_path = "data/en.json";
    let mut file = match tokio::fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => tokio::fs::File::create(&file_path).await.unwrap()
    };

    let mut file_data = serde_json::json!({});
  
    file_data["jobs"] = get_table("jobs",data.clone()).await;
    file_data["projects"] = get_table("projects",data.clone()).await;
    file_data["details"] = get_table("details",data.clone()).await;
    file_data["testimonials"] = get_table("testimonials",data).await;

    let json_response = serde_json::to_string(&file_data).unwrap();

    file.write_all(json_response.as_bytes()).await.unwrap();

    Ok((StatusCode::OK, Json(json_response)))
}
