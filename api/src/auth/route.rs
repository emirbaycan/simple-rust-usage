use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    auth::handler::login_handler,
    AppState,
};

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_handler))
        .with_state(app_state)
}
