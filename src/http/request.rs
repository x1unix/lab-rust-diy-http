use super::query_string::QueryString;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[cfg(test)]
mod tests;

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

const HTTP_HEADER_MAX_SIZE: usize = 8 * 1024; // 8KiB

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

pub struct Request<'buf> {
    path: &'buf str,
    method: Method,
    query_string: Option<QueryString>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        // Convert &Option<T> to Option<&T>
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf mut dyn Read> for Request<'buf> {
    type Error = ParseError;
    fn try_from(src: &'buf mut dyn Read) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(src);
        let mut buff = String::with_capacity(HTTP_HEADER_MAX_SIZE);
        reader.read_line(&mut buff)?;

        unimplemented!();
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buff: &'buf [u8]) -> Result<Self, Self::Error> {
        let req_str = std::str::from_utf8(buff)?;
        let (method, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let mut query_string = None;
        let method: Method = method.parse()?;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Request {
            path,
            method,
            query_string,
        })
    }
}

struct RequestHeader<'buf> {
    path: String,
    method: Method,
    query_string: Option<QueryString>,
}

fn parse_header(req_str: &str) -> Result<RequestHeader, ParseError> {
    let (method, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
    let (path, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
        return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;
    if let Some(i) = path.find('?') {
        return Ok(RequestHeader {
            path: String::from(&path[..i]),
            query_string: Some(QueryString::from(&path[i + 1..])),
            method,
        });
    }

    Ok(RequestHeader {
        path: String::from(path),
        query_string: None,
        method,
    })
}

fn get_next_word(src: &str) -> Option<(&str, &str)> {
    if src.is_empty() {
        return None;
    }
    for (i, char) in src.chars().enumerate() {
        if char.is_whitespace() {
            return Some((&src[..i], &src[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidProtocol,
    InvalidEncoding,
    ReadError(std::io::Error),
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidRequest => String::from("Invalid request"),
            Self::InvalidMethod => String::from("Invalid method"),
            Self::InvalidProtocol => String::from("Invalid protocol"),
            Self::InvalidEncoding => String::from("Invalid encoding"),
            Self::ReadError(err) => {
                format!("Read error: {err}")
            }
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        Self::ReadError(err)
    }
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if let Self::ReadError(err) = self {
            return write!(f, "ReadError({})", err);
        }

        write!(
            f,
            "{}",
            match self {
                Self::InvalidRequest => "InvalidRequest",
                Self::InvalidMethod => "InvalidMethod",
                Self::InvalidProtocol => "InvalidProtocol",
                Self::InvalidEncoding => "InvalidEncoding",
                Self::ReadError(_) => "ReadError",
            }
        )
    }
}

impl Error for ParseError {}
