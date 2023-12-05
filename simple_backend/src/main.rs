use actix_web::{get, web, web::Data, App, HttpServer, Result as AwResult};
use dotenv::dotenv;
use maud::Markup;
use sqlx::postgres::{PgPoolOptions};
use sqlx::{Pool, Postgres};


mod database;
mod pages;
mod routes;

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

    database::init_db(&pool).await.expect("Failed to initialize tables");


    HttpServer::new(
        move ||
        {
            let mut app = App::new()
                .app_data(Data::new(AppState { db: pool.clone() }));
            app = app.service(routes::index_page)
                    .service(actix_files::Files::new("/static", "./static").show_files_listing())
                    .service(routes::db::db_page)
                    .service(routes::db::table_page);
            return app;
        }
    ).bind(("127.0.0.1", 8080))?
    .run()
    .await
}


