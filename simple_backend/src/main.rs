use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres};
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    dotenv().ok();
    

    let postgres_url = std::env::var("DATABASE_URL").expect("Must set up DB URL");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_url)
    .await.expect("DB Failed to connect");

    db::init_tables(&pool).await.expect("Failed to initialize tables");


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

