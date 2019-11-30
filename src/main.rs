extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{web, HttpServer, App, HttpRequest};
use std::env;
use std::sync::Arc;
use std::sync::Mutex;


mod amqp;


fn index (_req: HttpRequest) -> &'static str {

    "Hello World!"
}

struct AppState {
    queue: Arc<amqp::amqp::AmqpCollector>,
}

fn get_address_port() -> String {
    let address: String = env::var("ADDRESS")
        .unwrap_or_else(|_| String::from("0.0.0.0"));

    let port: String = env::var("PORT")
        .unwrap_or_else(|_| String::from("80"));

    format!("{}:{}", address, port)
}

fn make_queue() -> amqp::amqp::AmqpCollector {
    println!("Creating Queue, Starting Connection");
    let url: String = env::var("CLOUDAMQP_URL")
        .unwrap_or_else(|_| String::from("amqp://guest:guest@localhost:5672"));

    let api_key: String = env::var("CLOUDAMQP_APIKEY")
        .unwrap_or_else(|_| String::from(""));

    let queue_name: String = env::var("QUEUE_NAME")
        .unwrap_or_else(|_| String::from("rust_queue"));

    amqp::amqp::AmqpCollector::new(url, api_key, queue_name)
}


fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(
        || {
            println!("Server created!!!!!!!!!!!!");
            return App::new()
            .service(
                web::resource("/").to(index)
            )
        });
    
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(get_address_port()).unwrap()
    };

    server.run()
}
