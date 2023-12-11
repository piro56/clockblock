use std::time::{SystemTime, UNIX_EPOCH};
use openssl::{rsa::{Rsa, Padding}};
use sqlx::{Postgres, Pool};
use anyhow::Error;
use serde::Deserialize;
use actix_web::{post, put, web, web::Data, Result};


use crate::database;
use crate::database::timelock::{UnlockInfo};
use crate::AppState;


// Encrypt data
// Store keys
// [id] [private key] [timestamp]
// Don't need public key
pub async fn ssl_encrypt(pool: &Pool<Postgres>, store: &String, timestamp_secs: i64) -> Result<i64, Error> {

    let rsa = Rsa::generate(2048).unwrap();
    let data = store.as_bytes();
    let mut buf = vec![0; rsa.size() as usize];    
    let _ = rsa.public_encrypt(data, &mut buf, Padding::PKCS1)?;
    
    let privkey = rsa.private_key_to_pem()?;
    let pubkey = rsa.public_key_to_pem_pkcs1()?;

    let dbresult = database::timelock::add_key(pool, &buf, &privkey, timestamp_secs as i64).await?;
        

    return Ok(dbresult)
}

#[derive(Deserialize)]
struct LockRequest {
    data: String,
    lock_time: String
}

#[derive(Deserialize)]
struct UnlockRequest {
    id: i64
}

#[put("/encrypt/lock")]
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

    let id = ssl_encrypt(&state.db, &key, lock_time).await;
    let id = match id {
        Ok(k) => k,
        Err(e) => { return Err(actix_web::error::ErrorBadRequest(e.to_string())) }
    };

    let stringkey = id.to_string();
    println!("Created lock key ID: {id}");

    return Ok(String::from(stringkey));
}

#[post("/decrypt/unlock")]
pub async fn requestunlock(state: Data<AppState>, unlock: web::Json<UnlockRequest>) -> Result<String> {
    println!("Unlocking ID: {}", unlock.id);

    let result = decrypt_store(&state.db, unlock.id).await;
    let result = match result {
        Ok(r) => r,
        Err(e) => { return Err(actix_web::error::ErrorBadRequest(e.to_string())) }
    };

    return Ok(result)
}

pub async fn decrypt_store(pool: &Pool<Postgres>, id: i64) -> Result<String, Error> {

    let dbinfo = database::timelock::get_key(pool, id).await?;

    let priv_key = Rsa::private_key_from_pem(&dbinfo.privatekey)?;
    let mut buf = vec![0; priv_key.size() as usize];
    let bufsz = priv_key.private_decrypt(&dbinfo.encrypted_data, &mut buf, Padding::PKCS1)?;
    buf.truncate(bufsz);
    
    let result = String::from_utf8(buf)?;

    return Ok(result);
}

