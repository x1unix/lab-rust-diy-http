use std::collections::HashMap;

enum QueryParam<'buf> {
    Single(&'buf str),
    Multiple(&'buf Vec<&'buf str>)
}

struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryParam<'buf>> 
}

impl<'buf> QueryString<'buf> {
    fn get(key: &str) -> Option<QueryParam> {
       unimplemented!() 
    }
}
