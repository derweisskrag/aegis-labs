// Server code
// Database is the SQLite! If you want to use PostgreSQL, please follow instructions.
use dotenvy::dotenv;
use std::env;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;

use axum::{
    routing::{post, get},
    Router,
    response::{IntoResponse, Json},
    body::Body,
    extract::Json as ExtractJson,
    extract::State,
    extract::Query
};

use serde::{Deserialize, Serialize}; 

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt_secret: String,

}


impl AppState {
    pub fn new(pool: SqlitePool, jwt_secret: String) -> Self {
        AppState { pool, jwt_secret }
    }
}


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub role: String,
    pub password_hash: String
}


#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Token {
    pub id: i64,
    pub refresh_token: String,
    pub created_at: Option<NaiveDateTime>,
    pub expires_at: NaiveDateTime,
    pub user_id: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject = username
    role: String,
    exp: usize, // expiration
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Can be repeatetive:
    // Might be re-factored in future 
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", db_url);

    // load the password
    let secret_password = env::var("ADMIN_PASSWORD").expect("Password for admin is not set");

    // JWT secret
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // now connect to db
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://users.sqlite")
        .await?;

    println!("[DB]: Connected to the database!");

    Ok(())
}
