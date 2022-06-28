#![allow(dead_code)]

mod handler;
mod http;
use std::env;

use http::Server;

fn main() {
    // Use build directory as root if public dir is not defined
    let static_path = env::var("PUBLIC_DIR")
        .unwrap_or(format!("{}/public", env!("CARGO_MANIFEST_DIR")).to_string());

    println!("Serving files from {}", static_path);
    let mut handler = handler::EchoHandler::new(static_path);
    let srv = Server::new("127.0.0.1:8080".to_string());
    srv.start(&mut handler);
}
