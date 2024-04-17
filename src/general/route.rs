use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    general::handler::update_translation_file,
    AppState,
};

pub fn general_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/update/translation_files",
            get(update_translation_file),
        )
        .with_state(app_state)
}
