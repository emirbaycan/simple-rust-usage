use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    detail::handler::{
        create_detail_handler, delete_detail_handler, detail_list_handler, edit_detail_handler,
        get_detail_handler,
    }, general::handler::health_checker_handler, image::handler::{
        upload_image_handler, create_image_handler, delete_image_handler, edit_image_handler, get_image_handler, image_list_handler, show_image_handler
    }, job::handler::{
        create_job_handler, delete_job_handler, edit_job_handler, get_job_handler, job_list_handler,
    }, project::handler::{
        create_project_handler, delete_project_handler, edit_project_handler, get_project_handler,
        project_list_handler,
    }, testimonial::handler::{
        create_testimonial_handler, delete_testimonial_handler, edit_testimonial_handler,
        get_testimonial_handler, testimonial_list_handler,
    }, user::handler::{
        create_user_handler, delete_user_handler, edit_user_handler,
        get_user_handler, user_list_handler,
    }, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/users", get(user_list_handler))
        .route("/api/users", post(create_user_handler))
        .route(
            "/api/users/:id",
            get(get_user_handler)
                .patch(edit_user_handler)
                .delete(delete_user_handler),
        )
        .route("/images/:path", get(show_image_handler))
        .route("/api/image", post(upload_image_handler))
        .route("/api/images", post(create_image_handler))
        .route("/api/images", get(image_list_handler))
        .route(
            "/api/images/:id",
            get(get_image_handler)
                .patch(edit_image_handler)
                .delete(delete_image_handler),
        )
        .route("/api/jobs", get(job_list_handler))
        .route("/api/jobs", post(create_job_handler))
        .route(
            "/api/jobs/:id",
            get(get_job_handler)
                .patch(edit_job_handler)
                .delete(delete_job_handler),
        )
        .route("/api/projects", get(project_list_handler))
        .route("/api/projects", post(create_project_handler))
        .route(
            "/api/projects/:id",
            get(get_project_handler)
                .patch(edit_project_handler)
                .delete(delete_project_handler),
        )
        .route("/api/testimonials", get(testimonial_list_handler))
        .route("/api/testimonials", post(create_testimonial_handler))
        .route(
            "/api/testimonials/:id",
            get(get_testimonial_handler)
                .patch(edit_testimonial_handler)
                .delete(delete_testimonial_handler),
        )
        .route("/api/details", get(detail_list_handler))
        .route("/api/details", post(create_detail_handler))
        .route(
            "/api/details/:id",
            get(get_detail_handler)
                .patch(edit_detail_handler)
                .delete(delete_detail_handler),
        )
        .with_state(app_state)
}
