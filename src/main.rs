use actix_web::get;
use actix_web::App;
use actix_web::HttpServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello world!"
}
