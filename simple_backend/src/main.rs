use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};


mod database;
mod pages;
mod routes;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome.");
    dotenv().ok();

    let postgres_url = std::env::var("DATABASE_URL").expect("Must set up DB URL");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_url)
    .await.expect("DB Failed to connect");


    HttpServer::new(
        move ||
        {
            let app = App::new()
                .app_data(Data::new(AppState { db: pool.clone() }));
            let app = app.service(routes::index_page)
                    .service(actix_files::Files::new("/static", "./static").show_files_listing())
                    .service(routes::db::db_page)
                    .service(routes::db::table_page)
                    .service(routes::encrypt::requestlock)
                    .service(routes::encrypt::requestunlock)
                    .wrap(actix_cors::Cors::permissive());
            return app;
        }
    ).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
