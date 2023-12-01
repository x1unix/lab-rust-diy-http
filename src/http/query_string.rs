use std::collections::HashMap;

#[derive(Debug)]
pub enum QueryParam {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug)]
pub struct QueryString {
    items: HashMap<String, QueryParam>,
}

impl QueryString {
    fn get(&self, key: &str) -> Option<&QueryParam> {
        self.items.get(key)
    }

    pub fn to_string(&self) -> String {
        // TODO: use FP approach
        let mut str = String::new();
        str.push('?');
        for (key, param) in self.items.iter() {
            match param {
                QueryParam::Single(v) => {
                    str.push_str(key);
                    str.push('=');
                    str.push_str(v);
                    str.push('&');
                }
                QueryParam::Multiple(values) => {
                    for item in values {
                        str.push_str(key);
                        str.push('=');
                        str.push_str(item);
                        str.push('&');
                    }
                }
            }
        }
        return str;
    }
}

impl From<&str> for QueryString {
    fn from(buf: &str) -> Self {
        let mut items = HashMap::new();
        for param in buf.split('&') {
            let (key, value) = match param.find('=') {
                Some(i) => (
                    String::from(&param[..i].to_owned()),
                    String::from(&param[i + 1..]),
                ),
                None => (param.to_string(), "".to_string()),
            };

            items
                .entry(key)
                .and_modify(|prev_val| match prev_val {
                    QueryParam::Single(old) => {
                        *prev_val = QueryParam::Multiple(vec![old.to_owned(), value.to_owned()]);
                    }
                    QueryParam::Multiple(v) => {
                        v.push(value.to_owned());
                    }
                })
                .or_insert(QueryParam::Single(value.to_owned()));
        }
        QueryString { items }
    }
}
