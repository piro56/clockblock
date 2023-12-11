use actix_web::{get, web, web::Data, Result as AwResult};
use maud::Markup;

use crate::AppState;
use crate::database;
use crate::pages;


#[get("/db")]
pub async fn db_page(state: Data<AppState>) -> AwResult<Markup> {
    let tables = database::describe_db::get_tables(&state.db).await;
    let tables = match tables {
        Ok(x) => x,
        Err(_) => Vec::new()
    };

    Ok(pages::db_page::initial_page(&tables))
}

#[get("/db/{table_name}/")]
pub async fn table_page(state: Data<AppState>, path: web::Path<String>) -> AwResult<Markup> {
    let table_name = path.into_inner();
    let table_schema = database::describe_db::get_table_info(&state.db, &table_name).await;
    let table_schema = table_schema.unwrap_or(Vec::new());

    Ok(pages::db_page::table_info_page(&table_name, &table_schema))
}
