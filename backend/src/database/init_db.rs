use sqlx::{Pool, Postgres};

pub async fn init_tables(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!("
    CREATE TABLE IF NOT EXISTS test_table (
        id bigserial,
        info text
    );").execute(pool).await?;

    sqlx::query!("
    CREATE TABLE IF NOT EXISTS timelock (
        id bigint NOT NULL,
        key bytea,
        unlock_time timestamp without time zone,
        entered_time timestamp without time zone,
        encrypted_data bytea
    );").execute(pool).await?;
    return Ok(());
}
