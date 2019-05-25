extern crate actix_web;
use actix_web::{web, HttpServer, App, HttpRequest};

fn index (req: HttpRequest) -> &'static str {
    "Hello World!"
}


fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(
            web::resource("/").to(index))
        )
        .bind("127.0.0.1:8080")
        .expect("Failed to start the server")
        .run()
}
