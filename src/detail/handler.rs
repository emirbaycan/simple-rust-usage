use serde_json::json;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::general::schema::FilterOptions;
use crate::detail::{ model::DetailModel, schema::{ CreateDetailSchema, UpdateDetailSchema } };
use crate::AppState;

pub async fn detail_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx
        ::query_as!(
            DetailModel,
            "SELECT * FROM details ORDER by created_at LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
        .fetch_all(&data.db).await;

    if query_result.is_err() {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response =
        serde_json::json!({
        "status": "success",
        "results": items.len(),
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_detail_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateDetailSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(
            DetailModel,
            "INSERT INTO details (description,about,position,company,img) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            body.description.to_string(),
            body.about.to_string(),
            body.position.to_string(),
            body.company.to_string(),
            body.img.to_string()
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response =
                json!({"status": "success","data": json!({
                "item": item
            })});

            return Ok((StatusCode::CREATED, Json(item_response)));
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                let error_response =
                    serde_json::json!({
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
pub async fn get_detail_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(DetailModel, "SELECT * FROM details WHERE id = $1", id)
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response =
                serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(_) => {
            let error_response =
                serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_detail_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateDetailSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(DetailModel, "SELECT * FROM details WHERE id = $1", id)
        .fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            DetailModel,
            "UPDATE details SET description = $1, about = $2, position = $3, company = $4, img = $5, updated_at = $6 WHERE id = $7 RETURNING *",
            body.description.to_owned().unwrap_or(item.description),
            body.about.to_owned().unwrap_or(item.about),
            body.position.to_owned().unwrap_or(item.position),
            body.company.to_owned().unwrap_or(item.company),
            body.img.to_owned().unwrap_or(item.img),
            now,
            id
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response =
                serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_detail_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx
        ::query!("DELETE FROM details WHERE id = $1", id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
