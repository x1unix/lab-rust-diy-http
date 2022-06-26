mod http;
use http::Server;

fn main() {
    let mut srv = Server::new("127.0.0.1:8080".to_string());
    srv.start();
}
