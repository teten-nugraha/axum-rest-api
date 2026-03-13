mod entity;
mod handler;
mod config;
mod database;
mod repository;
mod service;
mod logging;
mod middlewares;
use axum::{
    Router,
    routing::{get, post},
    middleware,
};
use crate::middlewares::auth_middleware::auth;
use handler::user_handler::get_users;
use tower_http::trace::TraceLayer;
use crate::handler::auth_handler::{login, register};

#[tokio::main]
async fn main() {
    config::init();

    let _guard = logging::init();

    tracing::info!("Starting server on port 3000");

    let db = database::connect_db().await;

    // let protected_route = Router::new()
    //     .route("/profile", get(protected))
    //     .route_layer(middleware::from_fn(auth));

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        // .merge(protected_route)
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let port = config::server_port();

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    println!("Server running on {}", port);

    axum::serve(listener, app).await.unwrap();
}
