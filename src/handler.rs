use crate::http::{Handler, Method, ParseError, Request, Response, StatusCode};
use std::fs;

pub struct EchoHandler {
    static_dir: String,
}

impl EchoHandler {
    pub fn new(static_dir: String) -> Self {
        Self { static_dir }
    }
    fn read_file(&self, path: &str) -> std::io::Result<String> {
        // TODO: use filepath join and check path
        fs::read_to_string(path)
    }

    fn normalize_path(&self, path: &str) -> Option<String> {
        let full_path = format!("{}/{}", self.static_dir, path);
        match fs::canonicalize(full_path) {
            Ok(canonical) => {
                if !canonical.starts_with(&self.static_dir) {
                    return None;
                }

                Some(canonical.into_os_string().into_string().unwrap())
            }
            Err(_) => None,
        }
    }

    fn serve_file(&self, path: &str) -> Response {
        match self.normalize_path(path) {
            Some(full_path) => match self.read_file(&full_path) {
                Ok(data) => Response::new(StatusCode::OK, Some(data)),
                Err(err) => Response::new(StatusCode::NotFound, Some(format!("Error: {err}"))),
            },
            None => Response::new(StatusCode::NotFound, None),
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
            _ => Response::new(
                StatusCode::BadRequest,
                Some("Unsupported HTTP method".to_string()),
            ),
        }
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some(format!("{}", err)))
    }
}
