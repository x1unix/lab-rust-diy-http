use super::ParseError;
use crate::http::{Request, Response};
use anyhow::{Context, Result};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub trait Handler {
    fn handle_request<'a>(&mut self, req: Request<'a>) -> Response<'a>;
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
                println!("Error: {err}");
                dbg!(err);
            }
        }
    }

    fn accept_request(&self, listener: &mut TcpListener, handler: &mut impl Handler) -> Result<()> {
        let (mut stream, addr) = listener.accept().with_context(|| "TCP accept failed")?;
        let mut rsp = Self::do_req(&mut stream, &addr, handler);

        println!("{}", rsp.status_code);
        rsp.send(&mut stream)
            .with_context(|| format!("{addr}: failed to send response"))
    }

    fn do_req<'a>(
        stream: &'a mut TcpStream,
        addr: &SocketAddr,
        handler: &'a mut impl Handler,
    ) -> Response<'a> {
        match Request::from_reader(stream) {
            Ok(req) => {
                Self::log_request(&req, &addr);
                handler.handle_request(req)
            }
            Err(err) => {
                println!("{addr}: can't parse request - {err}");
                handler.handle_bad_request(&err)
            }
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
