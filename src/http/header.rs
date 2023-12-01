use std::{collections::HashMap, ops::Deref};

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.deref())
    }

    fn add(&mut self, key: String, value: String) {
        self.0.insert(key, value);
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
        (Some(key), Some(val)) if !key.is_empty() => Some((key.to_owned(), val.to_owned())),
        _ => None,
    }
}
