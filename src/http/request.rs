use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

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

pub struct Request {
    path: String,
    method: Method,
    query_params: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        let data = std::str::from_utf8(buff)?;
        unimplemented!();
    }
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
