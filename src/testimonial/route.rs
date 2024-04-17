use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    testimonial::handler::{
        create_testimonial_handler, delete_testimonial_handler, edit_testimonial_handler,
        get_testimonial_handler, testimonial_list_handler,
    },
    AppState,
};

pub fn testimonial_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/testimonials", get(testimonial_list_handler))
        .route("/testimonials", post(create_testimonial_handler))
        .route(
            "/testimonials/:id",
            get(get_testimonial_handler)
                .patch(edit_testimonial_handler)
                .delete(delete_testimonial_handler),
        )
        .with_state(app_state)
}
