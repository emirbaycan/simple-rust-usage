    use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    image::handler::{
        create_image_handler, delete_image_handler, edit_image_handler, get_image_handler,
        image_list_handler, show_image_handler, update_all_images_handler, upload_image_handler,
    },
    AppState,
};

pub fn image_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/image", post(upload_image_handler))
        .route("/images", post(create_image_handler))
        .route("/images", get(image_list_handler))
        .route(
            "/images/:id",
            get(get_image_handler)
                .patch(edit_image_handler)
                .delete(delete_image_handler),
        )
        .with_state(app_state)
}

pub fn admin_image_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/update/all_images", get(update_all_images_handler))
        .with_state(app_state)
}

pub fn visitor_image_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/images/:path", get(show_image_handler))
        .with_state(app_state)
}