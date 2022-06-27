use crate::http::Request;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, SocketAddr},
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
                                write!(stream, "{} {} from {addr}", req.method, req.path).unwrap();
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
            None => String::new()
        };

        println!(
            "[{}] {} {}{}",
            addr,
            req.method,
            req.path,
            &query_params,
        );
    }
}
