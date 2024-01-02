use std::{
    collections::HashMap,
    io::{self, Write},
    ops::Deref,
};
use strum_macros::{self, AsRefStr, Display};

#[derive(Display, AsRefStr, Debug)]
pub enum Names {
    #[strum(serialize = "content-length")]
    ContentLength,

    #[strum(serialize = "transfer-encoding")]
    TransferEncoding,

    #[strum(serialize = "content-type")]
    ContentType,
}

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.deref())
    }

    pub fn add(&mut self, key: String, value: String) {
        self.0.insert(key.to_lowercase(), value);
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_lowercase(), value.to_owned());
    }

    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn content_length(&self) -> Option<u64> {
        self.0
            .get("content-length")
            .and_then(|s| s.parse::<u64>().ok())
    }

    pub fn set_content_length(&mut self, length: u64) {
        self.0.remove(Names::TransferEncoding.as_ref());
        self.0
            .insert(Names::ContentLength.to_string(), length.to_string());
    }

    pub fn send(&self, writer: &mut impl Write) -> io::Result<()> {
        for (i, (k, v)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(writer, "\r\n")?;
            }
            write!(writer, "{k}: {v}")?;
        }

        Ok(())
    }

    pub fn new() -> Headers {
        Headers(HashMap::new())
    }
}

impl From<&str> for Headers {
    fn from(value: &str) -> Self {
        Self(
            value
                .trim_start()
                .split("\r\n")
                .filter_map(split_header)
                .collect::<HashMap<String, String>>(),
        )
    }
}

fn split_header(header: &str) -> Option<(String, String)> {
    // TODO: normalize header names
    let mut iter = header.splitn(2, ": ");
    match (iter.next(), iter.next()) {
        (Some(key), Some(val)) if !key.is_empty() => {
            Some((key.to_lowercase().to_owned(), val.to_owned()))
        }
        _ => None,
    }
}
