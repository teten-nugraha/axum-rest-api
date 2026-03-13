use axum::extract::State;
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tracing_subscriber::registry::Data;
use crate::service::auth_service;

pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(req): Json<RegisterRequest>,
) -> & 'static str {
    auth_service::register(&db, req.email, req.password).await;

    "registered"
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(req): Json<LoginRequest>,
) -> String {
    let token = auth_service::login(&db, req.email, req.password).await.unwrap();

    token
}

