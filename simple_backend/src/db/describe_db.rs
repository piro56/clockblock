use sqlx::{Pool, Postgres};

pub async fn get_tables(pool: &Pool<Postgres>) -> Option<Vec<String>> {
    let mut result = sqlx::query!("
    SELECT table_name FROM information_schema.tables WHERE table_schema='public';
    ").fetch_all(pool).await;


    let records = match result {
        Ok(x) => x,
        Err(_) => return None
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
    return Some(out);
}

pub async fn get_table_info(pool: &Pool<Postgres>) {
    // Get Rows from Table
    // Get Table Creation Date
    // Get Table Accessors
}