use crate::http::{Handler, ParseError, Request, Response, StatusCode};

pub struct EchoHandler {}

impl EchoHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler for EchoHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        let body = format!(
            "<!DOCTYPE html>
            <html>
                <head>
                    <title>&#129408; &#129408; &#129408;</title>
                </head>
                <body>
                    <h1>Hello from &#129408;!</h1>
                    <div>
                        <p>Your request URL:</p>
                        <pre>{} {}</pre>
                    </div>
                </body>
            </html>",
            req.method, req.path
        );

        Response::new(StatusCode::OK, Some(body))
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some(format!("{}", err)))
    }
}
