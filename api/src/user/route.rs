use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    auth::route::authenticate,
    user::handler::{
        create_user_handler, delete_user_handler, edit_user_handler, get_user_handler,
        user_list_handler,
    },
    AppState,
};

pub fn user_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/users", get(user_list_handler))
    .route("/api/users", post(create_user_handler))
    .route(
        "/api/users/:id",
        get(get_user_handler)
        .patch(edit_user_handler)
        .delete(delete_user_handler),
    )
    .with_state(app_state)
    .layer(middleware::from_fn(authenticate))
}
