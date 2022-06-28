use crate::http::{Handler, ParseError, Request, Response, StatusCode, Method};
use std::fs;

pub struct EchoHandler {
    static_dir: String
}

impl EchoHandler {
    pub fn new(static_dir: String) -> Self {
        Self { static_dir }
    }
    fn read_file(&self, path: &str) -> std::io::Result<String> {
        // TODO: use filepath join and check path
        let full_path = format!("{}/{}", self.static_dir, path);
        fs::read_to_string(full_path.as_str())
    }
    fn serve_file(&self, path: &str) -> Response {
        match self.read_file(path) {
            Ok(data) => Response::new(StatusCode::OK, Some(data)),
            Err(err) => Response::new(StatusCode::NotFound, Some(format!("Error: {err}")))
        }
    }
}

impl Handler for EchoHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => self.serve_file("index.html"),
                path => self.serve_file(path),
            },
            _ => Response::new(StatusCode::BadRequest, Some("Unsupported HTTP method".to_string()))
        }
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some(format!("{}", err)))
    }
}
