use crate::http::{Request, Response, StatusCode};
use std::{
    io::{Read, Write},
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
                    match stream.read(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(req) => {
                                Self::log_request(&req, &addr);

                                let body = format!("{} {} from {addr}", req.method, req.path);
                                let rsp = Response::new(StatusCode::OK, Some(body));
                                write!(stream, "{}", rsp).unwrap();
                            }
                            Err(err) => {
                                println!("Error: {err}");
                                write!(stream, "[{addr}] Error: {}", err).unwrap();
                            }
                        },
                        Err(err) => println!("[{addr}] Error: failed to read request: {err:}"),
                    }

                    stream.shutdown(Shutdown::Both).unwrap();
                }
                Err(err) => {
                    println!("Failed to accept connection: {err:}");
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
