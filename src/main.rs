extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{web, HttpServer, App, HttpRequest};
use std::env;


fn index (_req: HttpRequest) -> &'static str {
    "Hello World!"
}

fn get_address_port() -> String {
    let address: String = env::var("ADDRESS")
        .unwrap_or_else(|_| String::from("0.0.0.0"));

    let port: String = env::var("PORT")
        .unwrap_or_else(|_| String::from("80"));

    format!("{}:{}", address, port)
}


fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(
        || App::new().service(
            web::resource("/").to(index))
        );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(get_address_port()).unwrap()
    };

    server.run()
}
