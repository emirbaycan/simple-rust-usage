use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    job::handler::{
        create_job_handler, delete_job_handler, edit_job_handler, get_job_handler, job_list_handler,
    },
    AppState,
};

pub fn job_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/jobs", get(job_list_handler))
        .route("/api/jobs", post(create_job_handler))
        .route(
            "/api/jobs/:id",
            get(get_job_handler)
                .patch(edit_job_handler)
                .delete(delete_job_handler),
        )
        .with_state(app_state)
}
