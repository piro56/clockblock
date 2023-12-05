

use sqlx::{Column, ValueRef};
use sqlx::{Pool, Postgres, postgres::PgQueryResult, Row};
use sqlx::postgres::{PgRow, PgColumn};


pub async fn get_tables(pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let result = sqlx::query!("
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

pub async fn get_table_info(pool: &Pool<Postgres>, table_name: &str) -> Result<Vec<(String, String)>, sqlx::Error> {
    // Get Rows from Table
    // Get Table Creation Date
    // Get Table Accessors

    let rows: Vec<PgRow> = sqlx::query(
        format!(r#"
        SELECT column_name, data_type FROM information_schema.columns 
        WHERE table_name='{table_name}';
        "#).as_str()
    ).fetch_all(pool).await?;    

    let mut table_schema: Vec<(String, String)> = Vec::new();

    for row in rows {
        let col_name = row.try_get_raw(0)?;
        let col_name = match col_name.is_null() {
            true => "NULL".to_string(),
            false => col_name.as_str().unwrap().to_string()
        };
        let data_type = row.try_get_raw(1)?;
        let data_type = match data_type.is_null() {
            true => "NULL".to_string(),
            false => data_type.as_str().unwrap().to_string()
        };
        table_schema.push((col_name, data_type));
    }

    Ok(table_schema)

}