use sqlx::{SqlitePool, FromRow};
use sqlx::types::chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub role: String,
    pub password_hash: String,
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, email, created_at, role, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_user(pool: &SqlitePool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, email, role, password_hash) VALUES ($1, $2, $3, $4)",
        user.username,
        user.email,
        user.role,
        user.password_hash
    )
    .execute(pool)
    .await?;
    Ok(())
}
