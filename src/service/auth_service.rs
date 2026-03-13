use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::repository::user_repository;

const SECRET: &str = "s3cre3t";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub async fn register(
    db: &DatabaseConnection,
    email: String,
    password: String,
){
    let hashed = hash(password, 10).unwrap();

    user_repository::create_user(db, email, hashed).await;
}

pub async fn login(
    db: &DatabaseConnection,
    email: String,
    password: String,
)-> Option<String> {

    let user = user_repository::find_by_email(db, &email).await?;

    let valid = verify(password, &user.password).unwrap();
    if !valid {
        return None;
    }

    let claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = encode (
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    ).unwrap();

    Some(token)

}