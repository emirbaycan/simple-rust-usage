use std::sync::Arc;

use axum::{
    extract::Request, middleware::Next, response::{IntoResponse, Response}, routing::{post}, Extension, Json, Router
};
use reqwest::StatusCode;
use tower_sessions::Session;

use crate::{auth::handler::login_handler, AppState};
use crate::auth::schema::User;

pub async fn authenticate(
    session: Session,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, impl IntoResponse)> { 
    // Check if the user is logged in
    if let Ok(Some(user)) = session.get::<User>("user").await {
        println!("User found in session: {:?}", user);
        // If logged in, continue processing the request
        let response = next.run(request).await;
        Ok(response)
    
    } else {
        // If session is missing 'user' key, return an unauthorized error response
        let error_response = serde_json::json!({
            "error": "Unauthorized"
        });
        Err((StatusCode::UNAUTHORIZED, Json(error_response)))
    }
}


pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_handler))
        .with_state(app_state)
}
