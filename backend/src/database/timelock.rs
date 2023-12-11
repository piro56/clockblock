
use anyhow::{Result, Error, anyhow};
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UnlockInfo {
    pub privatekey: Vec<u8>,
    pub encrypted_data: Vec<u8>,
    pub unlock_time: NaiveDateTime
}


pub async fn add_key(pool: &Pool<Postgres>, encrypted_data: &[u8], private_key: &[u8], timestamp: i64) -> Result<i64, anyhow::Error> {

    let tstamp= chrono::DateTime::from_timestamp(timestamp, 0);
    if tstamp.is_none() {
        return Err(anyhow!("Timestamp invalid"));
    }
    let tstamp = tstamp.unwrap().naive_utc();
    
    let now = chrono::offset::Utc::now().naive_utc();


    let result = sqlx::query!(
        r#"
        INSERT INTO timelock (key, encrypted_data, unlock_time, entered_time)
        VALUES ($1, $2, $3, $4) RETURNING id;
    "#, private_key, encrypted_data, tstamp, now).fetch_one(pool).await?.id;

    Ok(result)
}


pub async fn get_key(pool: &Pool<Postgres>, id: i64) -> Result<UnlockInfo, anyhow::Error> {

    let result = sqlx::query!(
    r#"
    SELECT key, encrypted_data, unlock_time FROM timelock
    WHERE id=$1;
    "#, id).fetch_one(pool).await?;
    let key = result.key.expect("Private Key Not Found");
    let data = result.encrypted_data.expect("Encrypted Data Not Found");
    let unlock_time = result.unlock_time.unwrap_or(chrono::offset::Utc::now().naive_utc());
    
    let unlock_info: UnlockInfo = UnlockInfo {privatekey: key, encrypted_data: data, unlock_time};

    return Ok(unlock_info);


}