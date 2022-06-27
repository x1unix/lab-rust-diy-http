pub enum StatusCode {}

pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
       Response { status_code, body } 
    } 
}
