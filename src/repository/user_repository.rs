use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entity::user::Entity as User;

pub async fn find_all(
    db: &DatabaseConnection,
) -> Vec<crate::entity::user::Model> {
    User::find()
        .all(db)
        .await
        .unwrap()
}