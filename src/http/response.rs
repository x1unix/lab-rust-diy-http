use super::{header::Headers, header::Names, status::StatusCode};
use std::convert::AsRef;
use std::error::Error;
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

    pub fn string(status_code: StatusCode, body: String) -> Response<'a> {
        let mut headers = Headers::new();
        headers.set_content_length(body.len() as u64);

        Response {
            status_code,
            headers,
            body: Some(Box::new(Cursor::new(body.into_bytes()))),
        }
    }

    pub fn error(status_code: StatusCode, err: &impl Error) -> Response<'a> {
        Self::string(status_code, err.to_string())
    }

    pub fn with_body(mut self, body: Box<dyn Read + 'a>) -> Response<'a> {
        self.body = Some(body);
        self
    }

    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.headers.add(key.to_owned(), val.to_owned());
        self
    }

    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.headers
            .add(Names::ContentType.to_string(), content_type);
        self
    }

    pub fn with_content_length(mut self, len: u64) -> Self {
        self.headers.set_content_length(len);
        self
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
        if let Some(ref mut body) = self.body {
            io::copy(body, stream)?;
        }

        if is_chunked {
            write!(stream, "\r\n\r\n")?;
        }

        Ok(())
    }
}
