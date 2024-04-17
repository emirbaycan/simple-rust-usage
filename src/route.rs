use std::sync::Arc;

use axum::{middleware, Router};

use crate::{
    auth::route::{auth_admin, auth_router}, detail::route::detail_router, general::route::general_router,
    image::route::{admin_image_router, image_router, visitor_image_router}, job::route::job_router, project::route::project_router,
    testimonial::route::testimonial_router, user::route::user_router, AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {

    let detail_route: Router = detail_router(app_state.clone());
    let general_route = general_router(app_state.clone());
    let image_route = image_router(app_state.clone());
    let auth_route = auth_router(app_state.clone());
    let job_route = job_router(app_state.clone());
    let project_route = project_router(app_state.clone());
    let testimonial_route = testimonial_router(app_state.clone());
    let user_route = user_router(app_state.clone());

    
    let admin_prefix = "/api/admin";

    let admin_route = Router::new()
    .nest(admin_prefix, detail_route)
    .nest(admin_prefix, general_route)
    .nest(admin_prefix, admin_image_router(app_state.clone()))
    .nest(admin_prefix, image_route)
    .nest(admin_prefix, job_route)
    .nest(admin_prefix, project_route)
    .nest(admin_prefix, testimonial_route)
    .nest(admin_prefix, user_route)
    .layer(middleware::from_fn(auth_admin));

    let visitor_prefix = "/";
    
    let visitor_route = Router::new()
    .nest(visitor_prefix, visitor_image_router(app_state))
    .nest(visitor_prefix, auth_route);

    Router::new()
        .merge(admin_route)
        .merge(visitor_route)
}
