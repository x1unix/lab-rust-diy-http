use super::ParseError;
use crate::http::{Request, Response};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub trait Handler {
    fn handle_request<'a, 'b>(&mut self, req: Request<'a>) -> Response<'b>;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response;
}

pub struct Server<'a> {
    address: String,
    handler: &'a mut dyn Handler,
}

impl<'a> Server<'a> {
    pub fn new(address: String, handler: &'a mut dyn Handler) -> Self {
        Self { address, handler }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server is running on {}", self.address);
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    self.handle_request(stream, addr);
                }
                Err(err) => {
                    println!("TCP accept failed: {err}");
                    dbg!(err);
                    return;
                }
            }
        }
    }

    fn handle_request(&mut self, mut stream: TcpStream, addr: SocketAddr) {
        let mut rsp = match Request::from_reader(&mut stream) {
            Ok(req) => {
                Self::log_request(&req, &addr);
                self.handler.handle_request(req)
            }
            Err(err) => {
                println!("{addr}: can't parse request - {err}");
                self.handler.handle_bad_request(&err)
            }
        };

        println!("{}", rsp.status_code);
        if let Err(err) = rsp.send(&mut stream) {
            println!("{addr}: failed to send response - {err}")
        }
    }

    fn log_request(req: &Request, addr: &SocketAddr) {
        let query_params = match &req.query_string() {
            Some(str) => str.to_string(),
            None => String::new(),
        };

        println!("[{}] {} {}{}", addr, req.method, req.path(), &query_params,);
    }
}
