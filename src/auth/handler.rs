use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::auth::schema::Login;
use crate::AppState;

use bcrypt::verify;
use tower_sessions::Session;

use super::schema::User;

pub async fn test_login_handler(
    session: Session,
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

    let json_response = serde_json::json!({
        "status": "success",
        "data": item
    });

    session.insert("logged_in",1).await.unwrap();
    session.insert("id",item.id).await.unwrap();
    session.insert("email",item.email).await.unwrap();
    session.insert("username",item.username).await.unwrap();
    session.insert("fullname",item.fullname).await.unwrap();
    session.insert("role",item.role).await.unwrap();
    session.insert("avatar",item.avatar).await.unwrap();
    session.insert("active",item.active).await.unwrap();

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn logout_handler(
    session: Session,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    session.flush().await.unwrap();

    let json_response = serde_json::json!({
        "status": "success"
    });

    Ok((StatusCode::OK, Json(json_response)))
}