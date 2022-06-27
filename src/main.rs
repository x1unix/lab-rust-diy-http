#![allow(dead_code)]

mod handler;
mod http;
use http::Server;

fn main() {
    let mut handler = handler::EchoHandler::new();
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.start(&mut handler);
}
