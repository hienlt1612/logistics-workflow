pub mod schema;
pub mod queries;
pub mod seed;

use sqlx::postgres::PgPool;

/// Global database pool, initialized once at startup.
static POOL: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

/// Initialize the global database pool.
pub async fn init_pool(url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    POOL.set(pool).map_err(|_| {
        sqlx::Error::Protocol("Pool already initialized".into())
    })?;
    log::info!("Database pool initialized");
    Ok(())
}

/// Get a reference to the global pool.
pub fn pool() -> &'static PgPool {
    POOL.get().expect("Database pool not initialized. Call init_pool first.")
}
