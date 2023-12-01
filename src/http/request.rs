use super::header::Headers;
use super::query_string::QueryString;
use super::url::URL;
use std::convert::{From, TryFrom};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;
use std::str::FromStr;
use thiserror::Error;

const READ_BUFFER_SIZE: usize = 1024;
const REQUEST_DELIMITER: &[u8; 4] = b"\r\n\r\n";
const READ_LIMIT: usize = 32 * 1024;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to read request")]
    ReadError,
    #[error("invalid request")]
    InvalidRequest,
    #[error("invalid HTTP method")]
    InvalidMethod,
    #[error("invalid HTTP protocol")]
    InvalidProtocol,
    #[error("invalid encoding")]
    InvalidEncoding,
    #[error("missing request body")]
    MissingBody,
    #[error("request too big")]
    RequestTooBig,
}

impl From<InvalidMethod> for ParseError {
    fn from(_: InvalidMethod) -> Self {
        Self::InvalidMethod
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

#[derive(Copy, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

pub struct InvalidMethod;

impl FromStr for Method {
    type Err = InvalidMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "DELETE" => Ok(Method::DELETE),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(InvalidMethod),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::GET => "GET",
                Self::POST => "POST",
                Self::PUT => "PUT",
                Self::HEAD => "HEAD",
                Self::DELETE => "DELETE",
                Self::OPTIONS => "OPTIONS",
                Self::CONNECT => "CONNECT",
                Self::TRACE => "TRACE",
                Self::PATCH => "PATCH",
            }
        )
    }
}

pub struct Request {
    pub method: Method,
    pub url: URL,
    pub headers: Headers,
    pub body: Option<Box<dyn Read>>,
}

impl<'buf> Request {
    pub fn path(&self) -> &str {
        &self.url.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        // Convert &Option<T> to Option<&T>
        self.url.query.as_ref()
    }

    pub fn from_reader(reader: &mut dyn Read) -> Result<Request, ParseError> {
        // Consume HTTP request until find payload delimiter.
        let (buff, body_offset) = read_until_payload(reader)?;

        dbg!(buff.len(), &buff[body_offset..]);
        let header_str = std::str::from_utf8(&buff[0..body_offset])?;
        let (url, method, offset) = parse_proto(header_str)?;

        // Collect http headers until request body starts
        let headers = Headers::from(&header_str[offset..]);

        dbg!(&buff[body_offset..]);

        Ok(Request {
            url,
            method,
            headers,
            body: None,
        })
    }
}

fn find_body_delimiter(buf: &[u8]) -> Option<usize> {
    buf.windows(REQUEST_DELIMITER.len())
        .position(|v| v == REQUEST_DELIMITER)
}

// impl TryFrom<&[u8]> for Request {
//     type Error = ParseError;
//     fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
//         let req_str = std::str::from_utf8(buff)?;
//         dbg!(req_str);

//         let (url, method, offset) = parse_proto(req_str)?;
//         Ok(Request {
//             url,
//             method,
//             body: None,
//         })
//     }
// }

// impl TryFrom<dyn Read> for Request {
//     type Error = ParseError;

//     fn try_from(value: dyn Read) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

/// Reads the reader until HTTP request payload delimiter (`\r\n\r\n`).
///
/// Returns buffer with consumed data and offset to end of HTTP header.
///
/// Returns [ParseError::MissingBody] if body delimiter is missing or [ParseError::RequestTooBig] if header is too big.
fn read_until_payload(src: &mut dyn Read) -> Result<(Vec<u8>, usize), ParseError> {
    let mut buff = vec![0; READ_BUFFER_SIZE];
    let mut bytes_read = 0;
    let mut can_continue = true;
    loop {
        match find_body_delimiter(&buff) {
            Some(end_offset) => {
                buff.resize(bytes_read, 0);
                return Ok((buff, end_offset));
            }
            None if !can_continue => {
                return Err(ParseError::MissingBody);
            }
            None if bytes_read >= READ_LIMIT => {
                return Err(ParseError::RequestTooBig);
            }
            None => {
                // Resize buffer if necessary and request next bytes.
                let offset = bytes_read;

                if offset + READ_BUFFER_SIZE > buff.len() {
                    buff.resize(buff.len() + READ_BUFFER_SIZE, 0);
                }

                // TODO: handle EOF
                let read_len = src
                    .read(&mut buff[offset..])
                    .map_err(|_| ParseError::ReadError)?;

                can_continue = read_len == READ_BUFFER_SIZE;
                bytes_read += read_len;
            }
        }
    }
}

fn parse_proto(src: &str) -> Result<(URL, Method, usize), ParseError> {
    // GET /path HTTP/1.1
    // TODO: handle very long paths...
    let head_end = src.find('\r').ok_or_else(|| ParseError::InvalidRequest)?;
    let chunks: Vec<&str> = src[..head_end].splitn(3, ' ').collect();
    if chunks.len() != 3 {
        return Err(ParseError::InvalidRequest);
    }

    let (method, path, protocol) = (chunks[0], chunks[1], chunks[2]);
    if protocol != "HTTP/1.1" {
        return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;
    let url = URL::from(path);
    Ok((url, method, head_end))
}

fn get_next_word(src: &str) -> Option<(&str, usize)> {
    if src.is_empty() {
        return None;
    }
    for (i, char) in src.chars().enumerate() {
        if char.is_whitespace() {
            return Some((&src[..i], i + 1));
        }
    }

    None
}
