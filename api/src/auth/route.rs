use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
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
    if let Ok(Some(logged_in)) = session.get::<usize>("logged_in").await {
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

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/test_login", get(test_login_handler))
        .route("/api/auth/logout", get(logout_handler))
        .with_state(app_state)
}
