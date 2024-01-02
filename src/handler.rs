use anyhow::{Context, Result};

use crate::http::{Handler, HeaderNames, Method, ParseError, Request, Response, StatusCode};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::Path;

const MAX_REQUEST_SIZE: u64 = 1024 * 1024;
const INDEX_FILE: &str = "index.html";

pub struct StaticHandler {
    static_dir: String,
}

impl StaticHandler {
    pub fn new(static_dir: String) -> Self {
        Self { static_dir }
    }

    fn normalize_path(&self, path: &str) -> Result<String> {
        let full_path = format!("{}/{}", self.static_dir, path);
        fs::canonicalize(full_path)
            .with_context(|| format!("Not Found: {}", path))
            .and_then(|p| {
                p.into_os_string()
                    .into_string()
                    .map_err(|_| anyhow::anyhow!("Failed to convert path to string"))
            })
            .map_err(Into::into)
    }

    fn guess_content_type(filename: &str) -> String {
        Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext {
                "jpg" | "jpeg" | "png" | "gif" | "webp" => Some(format!("image/{ext}")),
                "html" | "txt" | "css" => Some(format!("text/{ext}; charset=utf-8")),
                "wasm" | "js" | "pdf" => Some(format!("application/{ext}")),
                _ => None,
            })
            .unwrap_or_else(|| String::from("application/otcet-stream"))
    }

    fn serve_file<'a>(&self, path: &str) -> Response<'a> {
        self.normalize_path(path)
            .and_then(|abspath| {
                fs::File::open(&abspath)
                    .with_context(|| format!("Not Found: {}", abspath))
                    .and_then(|f| {
                        match f.metadata() {
                            Ok(stat) if stat.is_dir() => {
                                // List directory contents
                                self.serve_dir_list(&abspath, path)
                            }
                            Ok(stat) => Ok(Response::new(StatusCode::OK)
                                .with_content_length(stat.len())
                                .with_content_type(Self::guess_content_type(path))
                                .with_body(Box::new(f))),
                            Err(err) => Ok(Response::string(
                                StatusCode::Forbidden,
                                format!("Can't stat: {}", err),
                            )),
                        }
                    })
            })
            .unwrap_or_else(|err| Response::string(StatusCode::NotFound, format!("{}", err)))
    }

    fn serve_dir_list<'a>(&self, path: &str, public_path: &str) -> Result<Response<'a>> {
        let index_file = Path::new(path).join(INDEX_FILE);
        if index_file.exists() {
            return Ok(self.serve_file(format!("{public_path}/{INDEX_FILE}").as_str()));
        }

        let mut buff = Vec::from("<html><body>");
        write!(buff, "<h1>Index of {public_path}</h1><div><ul>")?;
        fs::read_dir(path)?.filter_map(|e| e.ok()).for_each(|s| {
            if let Ok(name) = s.file_name().into_string() {
                let _ = write!(
                    buff,
                    "\n<li><a href=\"{public_path}/{name}\">{name}</a></li>"
                );
            }
        });

        buff.extend_from_slice(b"\n</ul></div></body></html>");
        Ok(Response::new(StatusCode::OK)
            .with_content_length(buff.len() as u64)
            .with_content_type("text/html".to_string())
            .with_body(Box::new(Cursor::new(buff))))
    }
}

unsafe impl Send for StaticHandler {}
unsafe impl Sync for StaticHandler {}

impl Handler for StaticHandler {
    fn handle_request<'a, 'b>(&self, req: Request<'a>) -> Response<'b> {
        match req.method {
            Method::GET => match req.path() {
                "/" => self.serve_file(INDEX_FILE),
                path => self.serve_file(path),
            },
            Method::POST => dump_request(req)
                .unwrap_or_else(|e| Response::string(StatusCode::BadRequest, format!("{}", e))),
            _ => Response::string(
                StatusCode::BadRequest,
                "Unsupported HTTP method".to_string(),
            ),
        }
    }

    fn handle_bad_request(&self, err: &ParseError) -> Response {
        Response::error(StatusCode::BadRequest, err)
    }
}

fn dump_request<'a, 'b>(mut req: Request<'a>) -> anyhow::Result<Response<'b>> {
    let len = match req.headers.content_length() {
        Some(len) if len > MAX_REQUEST_SIZE => {
            return Ok(Response::string(
                StatusCode::PayloadTooLarge,
                "Request entity too large".to_owned(),
            ))
        }
        Some(len) => len,
        None => {
            return Ok(Response::string(
                StatusCode::BadRequest,
                "missing content length".to_owned(),
            ))
        }
    };

    // Right now I can't pass the body itself as a body
    // because this will cause 2 simultaneous mutable borrows (read + write).
    let mut body = Vec::with_capacity(len as usize);
    req.read_to_end(&mut body)
        .with_context(|| "Failed to read request body")?;
    let cur = Box::new(Cursor::new(body));

    let content_type = req
        .headers
        .get(HeaderNames::ContentType.as_ref())
        .unwrap_or("application/otcet-stream");

    Ok(Response::new(StatusCode::OK)
        .with_content_type(content_type.to_owned())
        .with_content_length(len)
        .with_body(cur))
}
