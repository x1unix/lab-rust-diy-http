use super::query_string::QueryString;

pub struct URL {
    pub path: String,
    pub query: Option<QueryString>,
}

impl From<&str> for URL {
    fn from(value: &str) -> Self {
        let (path, query) = match value.find('?') {
            Some(i) => (&value[..i], Some(QueryString::from(&value[i + 1..]))),
            None => (value, None),
        };
        let path = path.to_owned();
        Self { path, query }
    }
}
