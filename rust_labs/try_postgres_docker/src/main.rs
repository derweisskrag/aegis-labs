use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 1. Get the URL from the environment variable set in compose.yml
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // 2. Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 3. APPLY MIGRATIONS AUTOMATICALLY
    // This looks for the /migrations folder relative to your project root
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("✅ Database migrated successfully!");


    let python_name = "Snappy";
    sqlx::query("INSERT INTO pythons (name) VALUES ($1)")
        .bind(python_name)
        .execute(&pool)
        .await?;
    
    println!("🐍 Added {} to the Everglades!", python_name);

    // 2. Read it back
    let row: (i32, String) = sqlx::query_as("SELECT id, name FROM pythons WHERE name = $1")
        .bind(python_name)
        .fetch_one(&pool)
        .await?;

    println!("🔎 Found Python in DB: ID={}, Name={}", row.0, row.1);

    // Keep the app running (or start your web server)
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}