use crate::http::{Handler, HeaderNames, Method, ParseError, Request, Response, StatusCode};
use std::fs;
use std::path::Path;

pub struct EchoHandler {
    static_dir: String,
}

impl EchoHandler {
    pub fn new(static_dir: String) -> Self {
        Self { static_dir }
    }
    fn read_file(&self, path: &str) -> std::io::Result<String> {
        // TODO: use filepath join and check path
        fs::read_to_string(path)
    }

    fn normalize_path(&self, path: &str) -> Option<String> {
        let full_path = format!("{}/{}", self.static_dir, path);
        fs::canonicalize(full_path)
            .ok()
            .filter(|p| p.starts_with(&self.static_dir))
            .and_then(|p| p.into_os_string().into_string().ok())
    }

    fn guess_content_type(filename: &str) -> String {
        Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext {
                "jpg" | "jpeg" | "png" | "gif" | "webp" => Some(format!("image/{ext}")),
                "html" | "txt" | "css" => Some(format!("text/{ext}")),
                "wasm" | "js" | "pdf" => Some(format!("application/{ext}")),
                _ => None,
            })
            .unwrap_or_else(|| String::from("application/otcet-stream"))
    }

    fn serve_file<'a>(&self, path: &'a str) -> Response<'a> {
        self.normalize_path(path)
            .and_then(|f| fs::File::open(&f).ok())
            .and_then(|f| {
                Some({
                    let mut rsp = Response::new(StatusCode::OK)
                        .with_content_type(Self::guess_content_type(path));
                    if let Ok(stat) = f.metadata() {
                        rsp.headers.set_content_length(stat.len())
                    }

                    rsp.with_body(Box::new(f))
                })
            })
            .unwrap_or_else(|| {
                Response::string(StatusCode::NotFound, format!("File {} not found", path))
            })
    }
}

impl Handler for EchoHandler {
    fn handle_request<'a>(&mut self, req: Request<'a>) -> Response<'a> {
        match req.method {
            Method::GET => match req.path() {
                "/" => self.serve_file("index.html"),
                path => self.serve_file(path),
            },
            Method::POST => {
                // TODO: use streams instead of strings in Response.
                dump_request(req)
                    .unwrap_or_else(|e| Response::string(StatusCode::BadRequest, format!("{}", e)))
            }
            _ => Response::string(
                StatusCode::BadRequest,
                "Unsupported HTTP method".to_string(),
            ),
        }
    }

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        Response::error(StatusCode::BadRequest, err)
    }
}

fn dump_request(req: Request) -> anyhow::Result<Response> {
    let req = Box::new(req);
    let length = req
        .headers
        .content_length()
        .ok_or(anyhow::anyhow!("Missing content length"))?;

    let content_type = req
        .headers
        .get(HeaderNames::ContentType.as_ref())
        .unwrap_or("application/otcet-stream");

    // Keep this mess until Response gets Read support
    Ok(Response::new(StatusCode::OK)
        .with_content_type(content_type.to_owned())
        .with_content_length(length)
        .with_body(req))
}
