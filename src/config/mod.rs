use dotenvy::dotenv;
use std::env;

pub fn init(){
    dotenv().ok();
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn server_port() -> String {
    env::var("SERVER_PORT").unwrap_or("3000".to_string())
}