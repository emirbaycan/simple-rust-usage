mod general;

mod job;
mod project;
mod testimonial;
mod detail;

mod user;
mod image;
mod auth;

mod route;

use std::sync::Arc;

use std::fs;
use std::path::Path;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use sqlx::{Pool, Postgres};

use time::Duration;

use tower_sessions::{session_store::ExpiredDeletion, Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let images = Path::new("images");
    if !images.exists() {
        fs::create_dir(images).ok();
    }

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();
 
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(1800)));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(session_layer).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1998").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    deletion_task.await.unwrap().unwrap();
    
}