use dotenvy::dotenv;
use std::env;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; 
use chrono::{Utc, Duration};

use jsonwebtoken::{
    encode,
    decode, 
    Header, 
    EncodingKey,
    DecodingKey,
    Validation, 
    Algorithm
};

use bcrypt::{hash, DEFAULT_COST};

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
async fn main() -> Result<(), Box<dyn std::error::Error>>   {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("[Attention!]: DATABASE_URL must be set");
    println!("Connecting to database at: {}", db_url);

    // load the password & JWT secret
    let secret_password = env::var("ADMIN_PASSWORD").expect("[Attention!]: Password for admin is not set");
    let jwt_secret = env::var("JWT_SECRET").expect("[Attention!]: JWT_SECRET must be set");

    // now connect to db
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("[DB]: Connected to the database!");


    // Create the admin
    // let password_hash = hash(&secret_password, DEFAULT_COST)?;
    // sqlx::query!(
    //     "INSERT INTO users (username, email, role, password_hash) 
    //     VALUES ($1, $2, $3, $4)",
    //     "admin",
    //     "admin@example.com",
    //     "admin",
    //     password_hash
    // )
    // .execute(&pool)
    // .await?;

    // Fetch user
    let admin = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", 1)
        .fetch_one(&pool)
        .await?;

    // -- Debug: print the username
    println!("Admin name is {}", admin.username);

    // Add token logic
    let expires_at_datetime = (Utc::now() + Duration::days(30)).naive_utc();
    let exp_claim_timestamp = expires_at_datetime.timestamp() as usize;
    
    let claims = Claims {
        sub: admin.username,
        role: "admin".to_string(),
        exp: exp_claim_timestamp,
    };

    // Create that token
    let refresh_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref())
    ).map_err(|_| "Failed to generate token".to_string())?;

    sqlx::query!(
        "INSERT INTO tokens (refresh_token, expires_at, user_id) 
        VALUES ($1, $2, $3)",
        refresh_token,
        expires_at_datetime,
        admin.id
    )
    .execute(&pool)
    .await?;

    let token = sqlx::query_as!(Token, "SELECT * FROM tokens WHERE id = $1", 1)
        .fetch_one(&pool)
        .await?;

    println!("[INFO]: Token: {:?}", token);

    Ok(())
}