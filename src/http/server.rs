use crate::http::{Request, Response};
use anyhow::{Context, Result};
use std::{
    io::Read,
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
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
        let mut listener = TcpListener::bind(&self.address).unwrap();
        println!("Server is running on {}", self.address);
        loop {
            if let Err(err) = self.accept_request(&mut listener, handler) {
                println!("Error: {err}")
            }
        }
    }

    fn accept_request(&self, listener: &mut TcpListener, handler: &mut impl Handler) -> Result<()> {
        let (mut stream, addr) = listener.accept().with_context(|| "TCP accept")?;
        let mut buff = Box::new([0; 1024]);
        let byte_count = stream
            .read(&mut *buff)
            .with_context(|| format!("TCP read failed: {}", addr))?;

        println!("Received {byte_count} bytes from {addr}");
        let rsp = match Request::try_from(&buff[0..byte_count]) {
            Ok(req) => {
                Self::log_request(&req, &addr);
                handler.handle_request(&req)
            }
            Err(err) => handler.handle_bad_request(&err),
        };
        rsp.send(&mut stream)
            .with_context(|| format!("{addr}: failed to send response"))
    }

    fn log_request(req: &Request, addr: &SocketAddr) {
        let query_params = match &req.query_string() {
            Some(str) => str.to_string(),
            None => String::new(),
        };

        println!(
            "[{}] {} {}{}",
            addr,
            req.method(),
            req.path(),
            &query_params,
        );
    }
}
