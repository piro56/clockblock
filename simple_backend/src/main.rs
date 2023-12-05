use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer, Responder, Result as AwResult};
use dotenv::dotenv;
use maud::Markup;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres};


mod db;
mod pages;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    dotenv().ok();
    

    let postgres_url = std::env::var("DATABASE_URL").expect("Must set up DB URL");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_url)
    .await.expect("DB Failed to connect");

    db::init_db(&pool).await.expect("Failed to initialize tables");


    HttpServer::new(
        move ||
        {
            let mut app = App::new()
                .app_data(Data::new(AppState { db: pool.clone() }));
            app = app.service(main_page)
                    .service(actix_files::Files::new("/static", "./static").show_files_listing())
                    .service(db_page);
            return app;
        }
    ).bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn main_page() -> AwResult<Markup> {
    Ok(pages::simple_page::getPage())
}

#[get("/db")]
async fn db_page(state: Data<AppState>) -> AwResult<Markup> {
    let mut tables = db::describe_db::get_tables(&state.db).await;
    let tables = match tables {
        Some(x) => x,
        None => Vec::new()
    };

    Ok(pages::db_page::initial_page(&tables))
}

#[get("/db/{table_name}")]
async fn table_page(state: Data<AppState>, path: web::Path<(String)>) -> AwResult<Markup> {
    let table_name = path.into_inner();
    Ok(pages::simple_page::getPage())
}
