use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    auth::route::auth_router,
    detail::{
        handler::{
            create_detail_handler, delete_detail_handler, detail_list_handler, edit_detail_handler,
            get_detail_handler,
        },
        route::detail_router,
    },
    general::route::general_router,
    image::route::image_router,
    job::route::job_router,
    project::route::project_router,
    testimonial::route::testimonial_router,
    user::route::user_router,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let detail_route = detail_router(app_state.clone());
    let general_route = general_router(app_state.clone());
    let image_route = image_router(app_state.clone());
    let auth_route = auth_router(app_state.clone());
    let job_route = job_router(app_state.clone());
    let project_route = project_router(app_state.clone());
    let testimonial_route = testimonial_router(app_state.clone());
    let user_route = user_router(app_state);

    Router::new()
        .nest("/", detail_route)
        .nest("/", general_route)
        .nest("/", image_route)
        .nest("/", auth_route)
        .nest("/", job_route)
        .nest("/", project_route)
        .nest("/", testimonial_route)
        .nest("/", user_route)
}
