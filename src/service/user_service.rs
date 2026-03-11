use sea_orm::DatabaseConnection;
use tracing::info;
use crate::repository::user_repository;

pub async fn get_all_users(
    db: &DatabaseConnection,
)-> Vec<crate::entity::user::Model> {

    info!("fetchng users from database");

    user_repository::find_all(db).await
}