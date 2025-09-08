use jsonwebtoken::{
    encode,
    decode, 
    Header, 
    EncodingKey,
    DecodingKey,
    Validation, 
    Algorithm
};

use bcrypt::{hash, DEFAULT_COST, verify};

use dotenvy::dotenv;
use std::env;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; 
use chrono::{Utc, Duration};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Option<i64>,
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

// CLI
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Login CLI")]
#[command(about = "Login using username and password", long_about = None)]
struct Args {
    /// Username to login with
    #[arg(long)]
    username: String,

    /// Password to verify
    #[arg(long)]
    password: String,
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let args = Args::parse();

    let db_url = env::var("DATABASE_URL").expect("[Attention!]: DATABASE_URL must be set");
    println!("Connecting to database at: {}", db_url);

    // load the password & JWT secret
    let secret_password = env::var("ADMIN_PASSWORD").expect("[Attention!]: Password for admin is not set");
    let jwt_secret = env::var("JWT_SECRET").expect("[Attention!]: JWT_SECRET must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        args.username
    )
    .fetch_one(&pool)
    .await?;

    if verify(&args.password, &user.password_hash).unwrap_or(false) {
        let expires_at = (Utc::now() + Duration::days(30)).naive_utc();
        let exp_ts = expires_at.timestamp() as usize;

        let claims = Claims {
            sub: user.username.clone(),
            role: user.role.clone(),
            exp: exp_ts,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|_| sqlx::Error::Protocol("JWT generation failed".into()))?;

        println!("[SUCCESS]: Logged in as {}", user.username);
        println!("[TOKEN]: {}", token);
    } else {
        println!("[ERROR]: Invalid password");
    }

    Ok(())
}