use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    detail::handler::{
        create_detail_handler, delete_detail_handler, detail_list_handler, edit_detail_handler,
        get_detail_handler,
    },
    AppState,
};

pub fn detail_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/details", get(detail_list_handler))
        .route("/api/details", post(create_detail_handler))
        .route(
            "/api/details/:id",
            get(get_detail_handler)
                .patch(edit_detail_handler)
                .delete(delete_detail_handler),
        ).with_state(app_state)
}
