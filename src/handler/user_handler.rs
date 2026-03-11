use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;
use crate::service::user_service;

pub async fn get_users(
    State(db): State<DatabaseConnection>
) -> Json<Vec<crate::entity::user::Model>> {
    let users = user_service::get_all_users(&db).await;

    Json(users)
}