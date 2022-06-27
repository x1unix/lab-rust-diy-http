#![allow(dead_code)]

mod http;
use http::Server;

fn main() {
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.start();
}
