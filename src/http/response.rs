use super::status::StatusCode;
use std::fmt::Display;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
       Response { status_code, body } 
    } 
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.phrase(), match &self.body {
            Some(body) => body,
            None => ""
        })
    }
}