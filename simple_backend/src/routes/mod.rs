pub mod db;

use actix_web::{get, Result as AwResult};
use maud::Markup;

use crate::pages;

#[get("/")]
pub async fn index_page() -> AwResult<Markup> {
    Ok(pages::simple_page::get_page())
}
