mod entity;
mod handler;
mod config;
mod database;
mod repository;
mod service;
mod logging;

use axum::{Router, routing::get};
use handler::user_handler::get_users;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    config::init();

    let _guard = logging::init();

    tracing::info!("Starting server on port 3000");

    let db = database::connect_db().await;

    let app = Router::new()
        .route("/users", get(get_users))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let port = config::server_port();

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    println!("Server running on {}", port);

    axum::serve(listener, app).await.unwrap();
}
