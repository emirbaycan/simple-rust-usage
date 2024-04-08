use serde_json::json;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::general::schema::{FilterOptions, Response};
use crate::job::{ model::JobModel, schema::{ CreateJobSchema, UpdateJobSchema } };
use crate::AppState;

pub async fn job_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx
        ::query_as!(
            Response,
            "SELECT count(id) as count FROM jobs"
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

    let query_result = sqlx
        ::query_as!(
            JobModel,
            "SELECT * FROM jobs ORDER by created_at LIMIT $1 OFFSET $2",
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
        "count": count,
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_job_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateJobSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(
            JobModel,
            "INSERT INTO jobs (company,title,date,description) VALUES ($1, $2, $3, $4) RETURNING *",
            body.company.to_string(),
            body.title.to_string(),
            body.date.to_string(),
            body.description.to_string()
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

pub async fn get_job_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(JobModel, "SELECT * FROM jobs WHERE id = $1", id)
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

pub async fn edit_job_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateJobSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(JobModel, "SELECT * FROM jobs WHERE id = $1", id)
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
            JobModel,
            "UPDATE jobs SET company = $1, title = $2, date = $3, description = $4, updated_at = $5 WHERE id = $6 RETURNING *",
            body.company.to_owned().unwrap_or(item.company),
            body.title.to_owned().unwrap_or(item.title),
            body.date.to_owned().unwrap_or(item.date),
            body.description.to_owned().unwrap_or(item.description),
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

pub async fn delete_job_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx
        ::query!("DELETE FROM jobs WHERE id = $1", id)
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
