use super::status::StatusCode;
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.phrase(),
            match &self.body {
                Some(body) => body,
                None => "",
            }
        )
    }
}
