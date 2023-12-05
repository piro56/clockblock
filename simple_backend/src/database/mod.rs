use sqlx::{Pool, Postgres};
mod init_db;
pub mod describe_db;

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    init_db::init_tables(pool).await
}