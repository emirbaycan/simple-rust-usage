use serde_json::json;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::auth::schema::{Login, NewUser};
use crate::user::model::UserModel;
use crate::AppState;

use bcrypt::{hash, verify, DEFAULT_COST};
use tower_sessions::Session;

use super::schema::User;

pub async fn test_login_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    session.insert("logged_in",1).await.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });
    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn login_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Login>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email = body.email.unwrap();
    let password = body.password.unwrap();

    let query_result = sqlx::query_as!(
        User,
        "SELECT id,username,email,password,fullname,role,avatar,active FROM users WHERE email=$1",
        email
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User not exists",
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let password_hash = item.password.clone();
    let valid = verify(&password, &password_hash).unwrap();

    if !valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Passwords did not match",
        });
        return Err((StatusCode::NOT_ACCEPTABLE, Json(error_response)));
    }

    let user = serde_json::json!({
        "id":item.id,
        "email":item.email,
        "username":item.username,
        "fullname":item.fullname,
        "password":"",
        "role":item.role,
        "avatar":item.avatar,
        "active":item.active
    });

    let json_response = serde_json::json!({
        "status": "success",
        "data": item
    });

    session.insert("logged_in",1).await.unwrap();

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn logout_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    session.flush().await.unwrap();

    let json_response = serde_json::json!({
        "status": "success"
    });

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn register_handler(
    opts: Option<Query<Login>>,
    session: Session,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let email = opts.email.unwrap();
    let password = opts.password.unwrap();

    let query_result = sqlx::query_as!(NewUser, "SELECT id FROM users WHERE email=$1", email)
        .fetch_one(&data.db)
        .await;

    if !query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User already exists",
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query_result = sqlx
        ::query_as!(
            UserModel,
            "INSERT INTO users (username,password,email,fullname,role,avatar,notes,active) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            "",
            password,
            email,
            "",
            1,
            "",
            "",
            1
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let user = serde_json::json!({
                "id":item.id,
                "email":item.email,
                "username":item.username,
                "fullname":item.fullname,
                "password":"",
                "role":item.role,
                "avatar":item.avatar,
                "active":item.active
            });

            session.insert("user", user).await.unwrap();

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
