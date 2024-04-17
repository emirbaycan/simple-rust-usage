use std::sync::Arc;

use axum::Router;
use tower_sessions::SessionStore;

use crate::{
    auth::route::auth_router,
    detail::route::detail_router,
    general::route::general_router,
    image::route::image_router,
    job::route::job_router,
    project::route::project_router,
    testimonial::route::testimonial_router,
    user::route::user_router,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    
    let detail_route: Router = detail_router(app_state.clone());
    let general_route = general_router(app_state.clone());
    let image_route = image_router(app_state.clone());
    let auth_route = auth_router(app_state.clone());
    let job_route = job_router(app_state.clone());
    let project_route = project_router(app_state.clone());
    let testimonial_route = testimonial_router(app_state.clone());
    let user_route = user_router(app_state);

    Router::new()
        .merge(detail_route)
        .merge(general_route)
        .merge(image_route)
        .merge(auth_route)
        .merge(job_route)
        .merge(project_route)
        .merge(testimonial_route)
        .merge(user_route)
}
