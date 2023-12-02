use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row, Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    dotenv().ok();
    

    let postgres_url = std::env::var("DATABASE_URL").expect("Must set up DB URL");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_url)
    .await.expect("DB Failed to connect");

    init_tables(&pool).await.expect("Failed to initialize tables");


    HttpServer::new(
        ||
        {
            let mut app = App::new();
            app = app.service(main_page);
            return app;
        }
    ).bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

async fn init_tables(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS test_table (
        id bigserial,
        info text
    );"#,).execute(pool).await?;

    return Ok(());
}