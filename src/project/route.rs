use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    project::handler::{
        create_project_handler, delete_project_handler, edit_project_handler, get_project_handler,
        project_list_handler,
    },
    AppState,
};

pub fn project_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/projects", get(project_list_handler))
        .route("/projects", post(create_project_handler))
        .route(
            "/projects/:id",
            get(get_project_handler)
                .patch(edit_project_handler)
                .delete(delete_project_handler),
        )
        .with_state(app_state)
}
