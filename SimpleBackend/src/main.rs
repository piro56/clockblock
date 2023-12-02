use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
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
