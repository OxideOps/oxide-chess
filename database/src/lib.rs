use std::env;

use once_cell::sync::OnceCell;
use sqlx::{pool::PoolOptions, Error, Pool, Postgres};
use uuid::Uuid;

pub static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn run_migrations() -> Result<(), Error> {
    Ok(sqlx::migrate!().run(POOL.get().unwrap()).await?)
}

async fn init_db_pool() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    POOL.set(
        PoolOptions::<Postgres>::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await?,
    )
    .expect("database pool must be empty on initialization");

    Ok(())
}

pub async fn connect() -> Result<(), Error> {
    init_db_pool().await
}

pub async fn create_account(username: &str, password: &str, email: &str) -> Result<i32, Error> {
    sqlx::query!(
        "INSERT INTO accounts (username, password, email) VALUES ($1, $2, $3)",
        username,
        password,
        email,
    )
    .execute(POOL.get().unwrap())
    .await?;

    Ok(
        sqlx::query!("SELECT id FROM accounts WHERE username = $1", username)
            .fetch_one(POOL.get().unwrap())
            .await?
            .id,
    )
}

pub async fn create_email_verification_token(account_id: i32) -> Result<Uuid, Error> {
    sqlx::query!(
        r#"
        INSERT INTO email_verification_tokens 
            (account_id, expires_at) 
        VALUES 
            ($1, CURRENT_TIMESTAMP + INTERVAL '24 hours')
        "#,
        account_id,
    )
    .execute(POOL.get().unwrap())
    .await?;

    // Retrieve the generated token from the database.
    let row = sqlx::query!(
        r#"
        SELECT 
            token 
        FROM 
            email_verification_tokens 
        WHERE 
            account_id = $1 
        ORDER BY 
            created_at DESC 
        LIMIT 1
        "#,
        account_id
    )
    .fetch_one(POOL.get().unwrap())
    .await?;

    Ok(row.token)
}

pub async fn fetch_password(username: &str) -> Result<String, Error> {
    Ok(sqlx::query!(
        "SELECT password FROM accounts WHERE username = $1",
        username
    )
    .fetch_one(POOL.get().unwrap())
    .await?
    .password)
}
