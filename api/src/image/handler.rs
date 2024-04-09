use futures::TryFutureExt;
use reqwest::Body;
use serde_json::json;
use tokio::io;
use tokio_util::codec::{BytesCodec, FramedRead};
use std::sync::Arc;

use axum::extract::Multipart;
use futures_util::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use axum::{
    extract::{Path, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::general::schema::{FilterOptions, Table};
use crate::image::{
    model::ImageModel,
    schema::{CreateImageSchema, UpdateImageSchema},
};
use crate::AppState;

pub async fn show_image_handler(
    Path(path): Path<String>,
    State(data): State<Arc<AppState>>
) ->
Result<impl IntoResponse, (StatusCode, String)>  {
    let file_path = format!("images/{}", path);

    let file = match tokio::fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, "Image not found".to_string())),
    };
    // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = reqwest::Body::wrap_stream(stream);

    
    Ok(Response::new(file_body))
}

pub async fn upload_image_handler(
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        // Get the name of the file
        let name = field.name().unwrap().to_string();

        let file_path = format!("images/{}.webp", name);
        let mut file = File::create(&file_path).unwrap();

        // Write the data from the field to the file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap();
        }

        let query_result = sqlx::query_as!(
            ImageModel,
            "INSERT INTO images (name) VALUES ($1) RETURNING *",
            name + ".webp",
        )
        .fetch_one(&data.db)
        .await;

        match query_result {
            Ok(mut item) => {
                let file_rename_result =
                    std::fs::rename(file_path, format!("images/{}.webp", item.id));

                let name = format!("{}.{}", item.id, "webp");                
                let query_result = sqlx::query_as!(
                    ImageModel,
                    "UPDATE images SET name=$1 WHERE id=$2",
                    name,
                    item.id,
                )
                .fetch_one(&data.db)
                .await;
                item.name = name;

                let item_response = json!({"status": "success","data": json!({
                    "item": item
                })});

                return Ok((StatusCode::CREATED, Json(item_response)));
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": "Note with that title already exists",
                    });
                    return Err((StatusCode::CONFLICT, Json(error_response)));
                }
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error","message": format!("{:?}", e)})),
                ));
            }
        }
    }
    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn edit_image_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ImageModel, "SELECT * FROM images WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        // Get the name of the file
        let name = field.name().unwrap().to_string();

        let file_path = format!("images/{}.webp", id);
        let mut file = File::create(&file_path).unwrap();

        // Write the data from the field to the file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap();
        }
    }

    let item = query_result.unwrap();

    let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "item": item
    })});

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn image_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx
        ::query_as!(
            Table,
            "SELECT count(id) as count FROM images"
        )
        .fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let count = item.count;

    let query_result = sqlx::query_as!(
        ImageModel,
        "SELECT * FROM images ORDER by created_at LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": count,
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn get_image_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ImageModel, "SELECT * FROM images WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn delete_image_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ImageModel, "SELECT * FROM images WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;
    match query_result {
        Ok(item) => {
            let file_name = item.name;
            let images_dir = PathBuf::from("images");

            match std::fs::remove_file(images_dir.join(file_name)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }

    let rows_affected = sqlx::query!("DELETE FROM images WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
