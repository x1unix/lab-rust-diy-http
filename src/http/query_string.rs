use std::collections::HashMap;

#[derive(Debug)]
pub enum QueryParam<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    items: HashMap<&'buf str, QueryParam<'buf>>,
}

impl<'buf> QueryString<'buf> {
    fn get(&self, key: &str) -> Option<&QueryParam<'buf>> {
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

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(buf: &'buf str) -> Self {
        let mut items = HashMap::new();
        for param in buf.split('&') {
            let (key, value) = match param.find('=') {
                Some(i) => (&param[..i], &param[i + 1..]),
                None => (param, ""),
            };

            items
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
        QueryString { items }
    }
}
