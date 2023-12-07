
use actix_web::Error;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use std::time::{SystemTime, UNIX_EPOCH};


pub async fn add_key(pool: &Pool<Postgres>, private_key: &[u8], timestamp: i64) -> Result<PgQueryResult, sqlx::Error> {

    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
                                .expect("Time Error");
    let now = since_the_epoch.as_secs() as i64;

    let result = sqlx::query("
        INSERT INTO timelock (key, unlock_time, entered_time)
        VALUES ($1, $2, $3)
    ").bind(private_key).bind(timestamp).bind(now)
    .execute(pool).await;


    result
}