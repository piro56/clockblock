use std::time::{SystemTime, UNIX_EPOCH};
use openssl::{rsa::{Rsa, Padding}, error::ErrorStack};
use sqlx::{Postgres, Pool};
use anyhow::Error;
use serde::Deserialize;
use actix_web::{post, web, App, web::Data, HttpServer, Result, Error as awError, ResponseError as rError};


use crate::database;
use crate::AppState;


// Encrypt data
// Store keys
// [id] [private key] [timestamp]
pub async fn ssl_encrypt(pool: &Pool<Postgres>, store: &String, timestamp_secs: i64) -> Result<Vec<u8>, Error> {

    let rsa = Rsa::generate(2048).unwrap();
    let data = store.as_bytes();
    let mut buf = vec![0; rsa.size() as usize];    
    let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1)?;
    
    let privkey = rsa.private_key_to_pem()?;
    let pubkey = rsa.public_key_to_pem_pkcs1()?;

    database::timelock::add_key(pool, &privkey, timestamp_secs as i64).await?;

    return Ok(pubkey)
}

#[derive(Deserialize)]
struct LockRequest {
    data: String,
    lock_time: String
}

#[post("/encrypt/lock")]
pub async fn requestlock(state: Data<AppState>, lock: web::Json<LockRequest>) -> Result<String> {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
                                .expect("Time Error");
    let now = since_the_epoch.as_secs() as i64;

    let key = lock.data.clone();
    let lock_time = lock.lock_time.clone();
    let lock_time = match lock_time.parse::<i64>() {
        Ok(t) => t,
        Err(_) => { return Err(actix_web::error::ErrorBadRequest("Time invalid"))}
    };

    if now > lock_time {
        return Err(actix_web::error::ErrorBadRequest("Time invalid"));
    }

    let pub_key = ssl_encrypt(&state.db, &key, lock_time).await;
    let pub_key = match pub_key {
        Ok(k) => k,
        Err(e) => { return Err(actix_web::error::ErrorBadRequest(e.to_string())) }
    };

    let stringkey = match std::str::from_utf8(&pub_key) {
        Ok(k) => k,
        Err(e) => { return Err(actix_web::error::ErrorBadRequest(e.to_string())) }
    };

    return Ok(String::from(stringkey));
}

pub fn decrypt_store(enc: &[u8], priv_key: &String) {
    

}
