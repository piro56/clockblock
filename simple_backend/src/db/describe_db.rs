use std::fmt;

use sqlx::{Pool, Postgres, postgres::PgQueryResult};

pub async fn get_tables(pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let mut result = sqlx::query!("
    SELECT table_name FROM information_schema.tables WHERE table_schema='public';
    ").fetch_all(pool).await;


    let records = match result {
        Ok(x) => x,
        Err(e) => return Err(e)
    };

    let mut out:Vec<String> = Vec::new();
    for record in records {
        match record.table_name {
            Some(name) => {
                out.push(name);
            },
            None => { continue; }
        }
    }
    return Ok(out);
}

pub async fn get_table_info(pool: &Pool<Postgres>, table_name: &str) -> Result<PgQueryResult, sqlx::Error> {
    // Get Rows from Table
    // Get Table Creation Date
    // Get Table Accessors

    let result = sqlx::query(
        format!(r#"
        SELECT column_name, data_type FROM information_schema.columns 
        WHERE table_name='{table_name}'
        "#).as_str()
    ).execute(pool).await;

    result

}