use sqlx::{FromRow, Row, Pool, Postgres};


pub async fn init_tables(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!("
    CREATE TABLE IF NOT EXISTS test_table (
        id bigserial,
        info text
    );").execute(pool).await?;
    return Ok(());
}
