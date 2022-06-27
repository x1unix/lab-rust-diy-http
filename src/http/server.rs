use crate::http::{Request, Response, StatusCode};
use std::{
    io::{Read},
    net::{Shutdown, SocketAddr, TcpListener},
};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn start(&self) {
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

                    let rsp = Self::respond(&addr, &mut buffer);
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

    fn respond(addr: &SocketAddr, buff: &[u8]) -> Response {
        match Request::try_from(&buff[..]) {
            Ok(req) => Self::handle_request(&req, addr),
            Err(err) => {
                println!("[{addr}] Error: failed to parse request: {err}");
                Response::new(StatusCode::BadRequest, Some(format!("{}", err)))
            },
        }
    }

    fn handle_request(req: &Request, addr: &SocketAddr) -> Response {
        Self::log_request(&req, &addr);

        let body = format!(
            "<html><body><pre>{} {} from {addr}</pre></body></html>",
            req.method, req.path
        );
        return Response::new(StatusCode::OK, Some(body));
    }

    fn log_request(req: &Request, addr: &SocketAddr) {
        let query_params = match &req.query_string {
            Some(str) => str.to_string(),
            None => String::new(),
        };

        println!("[{}] {} {}{}", addr, req.method, req.path, &query_params,);
    }
}
