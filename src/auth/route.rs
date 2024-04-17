use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;
use tower_sessions::Session;

use crate::{auth::handler::login_handler, AppState};

use super::handler::{logout_handler, test_login_handler};

pub async fn authenticate(
    session: Session,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, impl IntoResponse)> {
    // Check if the user is logged in
    if let Ok(Some(_logged_in)) = session.get::<usize>("logged_in").await {
        // If logged in, continue processing the request
        let response = next.run(request).await;
        Ok(response)
    } else {
        let error_response = serde_json::json!({
            "error": "Unauthorized",
        });
        Err((StatusCode::UNAUTHORIZED, Json(error_response)))
    }
}

pub async fn auth_admin(
    session: Session,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, impl IntoResponse)> {
    // Check if the user is logged in
    if let Ok(Some(_logged_in)) = session.get::<usize>("logged_in").await {
        
    } else {
        let error_response = serde_json::json!({
            "error": "Unauthorized",
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
    }

    if let Ok(Some(role)) = session.get::<usize>("role").await {
        if role != 3 {
            let error_response = serde_json::json!({
                "error": "Unauthorized",
            });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    } else {
        let error_response = serde_json::json!({
            "error": "Session expired relogin",
        });

        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
    }

    let response = next.run(request).await;
    Ok(response)
}

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/login", post(login_handler))
        .route("/auth/test_login", get(test_login_handler))
        .route("/auth/logout", get(logout_handler))
        .with_state(app_state)
}
