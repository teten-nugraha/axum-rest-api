use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::service::auth_service::Claims;

const  SECRET: &str = "s3cre3t";
pub async fn auth(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_header.unwrap().replace("Bearer ", "");

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    );

    if decoded.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}