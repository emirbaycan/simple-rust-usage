use serde_json::json;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::general::schema::{FilterOptions, Table};
use crate::project::{ model::ProjectModel, schema::{ CreateProjectSchema, UpdateProjectSchema } };
use crate::AppState;

pub async fn project_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    
    let query_result = sqlx
        ::query_as!(
            Table,
            "SELECT count(id) as count FROM projects"
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
            ProjectModel,
            "SELECT * FROM projects ORDER by created_at LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
        .fetch_all(&data.db).await
        .map_err(|e| {
            let error_response =
                serde_json::json!({
                "status": "fail",
                "message": format!("Something bad happened while fetching all items: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response =
        serde_json::json!({
        "status": "success",
        "count": count,
        "items": query_result
    });
    Ok(Json(json_response))
}

pub async fn create_project_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateProjectSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

        // Convert Vec<String> to JSON string
        let imgs_slice: Vec<String> = body.imgs.iter().map(|s| s.clone()).collect();
        let stacks_slice: Vec<String> = body.stacks.iter().map(|s| s.clone()).collect();
    
    let query_result = sqlx
        ::query_as!(
            ProjectModel,
            "INSERT INTO projects (title,description,imgs,demo,git,stacks) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            body.title.to_string(),
            body.description.to_string(),
            &imgs_slice[..],
            body.demo.to_string(),
            body.git.to_string(),
            &stacks_slice[..],
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

pub async fn get_project_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(ProjectModel, "SELECT * FROM projects WHERE id = $1", id)
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

pub async fn edit_project_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateProjectSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(ProjectModel, "SELECT * FROM projects WHERE id = $1", id)
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
            ProjectModel,
            "UPDATE projects SET title = $1, description = $2, imgs = $3, demo = $4, git = $5, stacks = $6, updated_at = $7 WHERE id = $8 RETURNING *",
            body.title.to_owned().unwrap_or(item.title),
            body.description.to_owned().unwrap_or(item.description),
            &body.imgs.to_owned().unwrap_or(item.imgs),
            body.demo.to_owned().unwrap_or(item.demo),
            body.git.to_owned().unwrap_or(item.git),
            &body.stacks.to_owned().unwrap_or(item.stacks),
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

pub async fn delete_project_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx
        ::query!("DELETE FROM projects WHERE id = $1", id)
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
