use super::{header::Headers, header::Names, status::StatusCode};
use std::convert::AsRef;
use std::io::{self, Cursor, Read, Write};
use std::string::ToString;

pub struct Response<'a> {
    pub status_code: StatusCode,
    pub body: Option<Box<dyn Read + 'a>>,
    pub headers: Headers,
}

impl<'a> Response<'a> {
    pub fn new(status_code: StatusCode) -> Response<'a> {
        Response {
            status_code,
            body: None,
            headers: Headers::new(),
        }
    }

    pub fn string(status_code: StatusCode, body: &str) -> Response<'a> {
        let mut headers = Headers::new();
        headers.set_content_length(body.len());

        Response {
            status_code,
            headers,
            body: Some(Box::new(Cursor::new(body.to_owned().into_bytes()))),
        }
    }

    pub fn send(&mut self, stream: &mut impl Write) -> std::io::Result<()> {
        let is_chunked = self.body.is_some() && !self.headers.has(Names::ContentLength.as_ref());

        // Write request in chunked mode if content length isn't specified
        if is_chunked {
            self.headers
                .add(Names::TransferEncoding.to_string(), String::from("chunked"));
        }

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_code.phrase(),
        )?;
        self.headers.send(stream)?;

        write!(stream, "\r\n\r\n")?;
        if self.body.is_some() {
            io::copy(self.body.unwrap().as_mut(), stream)?;
        }

        if is_chunked {
            write!(stream, "\r\n\r\n")?;
        }

        Ok(())
    }
}
