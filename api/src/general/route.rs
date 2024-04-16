use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    general::handler::{health_checker_handler, update_translation_file},
    AppState,
};

pub fn general_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/update/translation_files",
            get(update_translation_file),
        )
        .with_state(app_state)
}
