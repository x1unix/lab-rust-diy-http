use crate::http::{ParseError, Request};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
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
                    println!("Accepted connection from {addr}");
                    let mut buffer = [0; 4096];
                    match stream.read(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(req) => {
                                println!(
                                    "{} {} {}",
                                    req.method,
                                    req.path,
                                    req.query_string.unwrap_or("")
                                );
                                write!(stream, "{} {}", req.method, req.path).unwrap();
                            }
                            Err(err) => {
                                println!("Error: {err}");
                                write!(stream, "Error: {}", err).unwrap();
                            }
                        },

                        // Ok(count) => {
                        //     let req_str = String::from_utf8_lossy(&buffer[0..count]);
                        //     println!("Accepted request ({count} bytes): {req_str:?}");
                        //     stream.write(&buffer[..count]).unwrap();
                        // }
                        Err(err) => println!("Failed to read request: {err:}"),
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
}
