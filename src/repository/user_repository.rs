use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use crate::entity::user::{ActiveModel, Entity as User, Model};
use crate::entity::user::Column;

pub async fn find_all(
    db: &DatabaseConnection,
) -> Vec<crate::entity::user::Model> {
    User::find()
        .all(db)
        .await
        .unwrap()
}

pub async fn find_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Option<Model> {
    User::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await
        .unwrap()
}

pub async fn create_user(
    db: &DatabaseConnection,
    email: String,
    password: String,
) -> Model {
    use crate::entity::user::ActiveModel;
    use sea_orm::{ActiveModelTrait, Set};

    let user = ActiveModel {
        email: Set(email),
        password: Set(password),
        ..Default::default()
    };

    user.insert(db).await.unwrap()
}