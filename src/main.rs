extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{web, HttpServer, App, HttpRequest};

fn index (_req: HttpRequest) -> &'static str {
    "Hello World!"
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
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run()
}
