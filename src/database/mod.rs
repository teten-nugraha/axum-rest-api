use sea_orm::{Database, DatabaseConnection};
use crate::config;

pub async fn connect_db() -> DatabaseConnection {
    let database_url = config::database_url();

    Database::connect(database_url)
        .await
        .expect("Unable to connect to database")
}