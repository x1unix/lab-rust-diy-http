use core::panic;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

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

pub struct Request {
    path: String,
    method: Method,
    query_params: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        let req_str = std::str::from_utf8(buff)?;
        let (method, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (path, req_str) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req_str).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        unimplemented!();
    }
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
}

impl ParseError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidEncoding => "Invalid encoding",
        }
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
        write!(
            f,
            "{}",
            match self {
                Self::InvalidRequest => "InvalidRequest",
                Self::InvalidMethod => "InvalidMethod",
                Self::InvalidProtocol => "InvalidProtocol",
                Self::InvalidEncoding => "InvalidEncoding",
            }
        )
    }
}

impl Error for ParseError {}
