use crate::http::{Handler, ParseError, Request, Response, StatusCode, Method};

pub struct EchoHandler {}

impl EchoHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler for EchoHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => {
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
                                <a href=\"/admin\">Admin panel</a>
                            </body>
                        </html>",
                        req.method(), req.path()
                    );
            
                    Response::new(StatusCode::OK, Some(body))
                },
                "/admin" => Response::new(StatusCode::Unauthorized, Some("Access Denied".to_string())),
                _ => Response::new(StatusCode::NotFound, Some(format!("Page not found - {}", req.path())))
            },
            _ => Response::new(StatusCode::BadRequest, Some("Unsupported HTTP method".to_string()))
        }
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some(format!("{}", err)))
    }
}
