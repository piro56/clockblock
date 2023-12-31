use actix_cors::Cors;
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
    let port: u16 = std::env::var("BACKEND_PORT").expect("Must specify BACKEND_PORT").parse().expect("Port Read Error");
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postgres_url)
    .await.expect("DB Failed to connect");
    database::init_db(&pool).await.expect("Database failed to construct");
    HttpServer::new(
        move ||
        {
            let app = App::new()
                .app_data(Data::new(AppState { db: pool.clone() }));
            let app = app.service(routes::index_page)
            .wrap(Cors::permissive())
                    .service(actix_files::Files::new("/static", "./static").show_files_listing())
                    .service(routes::db::db_page)
                    .service(routes::db::table_page)
                    .service(routes::encrypt::requestlock)
                    .service(routes::encrypt::requestunlock);
                
            return app;
        }
    ).bind(("0.0.0.0", port))?
    .run()
    .await
}
