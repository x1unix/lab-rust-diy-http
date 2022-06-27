use crate::http::{Request, Response};
use std::{
    io::Read,
    net::{Shutdown, SocketAddr, TcpListener},
};

use super::ParseError;

pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response;
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn start(&self, handler: &mut impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server is running on {}", self.address);
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 4096];
                    if let Err(err) = stream.read(&mut buffer) {
                        println!("[{addr}] Error: failed to read request: {err:}");
                        _ = stream.shutdown(Shutdown::Both);
                        continue;
                    }

                    let rsp = match Request::try_from(&buffer[..]) {
                        Ok(req) => {
                            Self::log_request(&req, &addr);
                            handler.handle_request(&req)
                        }
                        Err(err) => {
                            println!("[{addr}] Error: failed to parse request: {err}");
                            handler.handle_bad_request(&err)
                        }
                    };

                    match rsp.send(&mut stream) {
                        Ok(()) => {
                            println!("[{addr}] {} {}", rsp.status_code, rsp.status_code.phrase())
                        }
                        Err(err) => println!("[{addr}] Error: failed to serve response: {err}"),
                    }
                    _ = stream.shutdown(Shutdown::Both);
                }
                Err(err) => {
                    println!("Failed to accept connection: {err}");
                    continue;
                }
            }
        }
    }

    fn log_request(req: &Request, addr: &SocketAddr) {
        let query_params = match &req.query_string {
            Some(str) => str.to_string(),
            None => String::new(),
        };

        println!("[{}] {} {}{}", addr, req.method, req.path, &query_params,);
    }
}
