use std::collections::HashMap;

struct Range(i32, i32);

#[derive(Debug)]
pub enum QueryParam {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug)]
pub struct QueryString {
    data: String,
    items: HashMap<String, String>,
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
        let mut qs = QueryString {
            data: String::from(buf),
            items: HashMap::new(),
        };

        for param in qs.data.split('&') {
            let (key, value) = match param.find('=') {
                Some(i) => (String::from(&param[..i]), String::from(&param[i + 1..])),
                None => (String::from(param), String::new()),
            };

            qs.items
                .entry(key)
                .and_modify(|prev_val| match prev_val {
                    QueryParam::Single(old) => {
                        *prev_val = QueryParam::Multiple(vec![old, value]);
                    }
                    QueryParam::Multiple(v) => {
                        v.push(value);
                    }
                })
                .or_insert(QueryParam::Single(value));
        }
        qs
    }
}
