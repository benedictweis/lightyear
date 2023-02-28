use crate::mime;
use crate::response_code::ResponseCode;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};
use chrono::Local;

pub struct Response {
    protocol_version: String,
    response_code: ResponseCode,
    headers: HashMap<String, String>,
    body: String,
    pub(crate) error: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            protocol_version: "HTTP/1.1".into(),
            response_code: ResponseCode::OK,
            headers: HashMap::new(),
            body: "".into(),
            error: "0".into(),
        }
    }

    pub fn send(&mut self, body_str: &str) {
        self.body = body_str.into();
        self.header("Content-Type", "text/html");
    }

    pub fn send_file(&mut self, file_path: &Path) -> io::Result<()> {
        let file_path = file_path.canonicalize()?;
        self.body = fs::read_to_string(file_path.clone())?;
        self.header("Content-Type", &mime::match_extension(file_path.extension().unwrap_or_default().to_str().unwrap_or_default()));
        Ok(())
    }

    pub fn send_json(&mut self, json: serde_json::Value) {
        self.body = json.to_string();
        self.header("Content-Type", "application/json");
    }

    pub fn header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.into(), value.into());
    }

    pub fn redirect(&mut self, location: &str) {
        self.response_code = ResponseCode::Found;
        self.header("Location", location);
    }

    pub fn status(&mut self, status_code: usize) {
        self.response_code = ResponseCode::try_from(status_code).unwrap();
    }

    pub fn error(&mut self, error_code: &str) {
        self.error = error_code.into();
    }

    pub fn compose_default_headers(&mut self) {
        self.header("Server", "lightyear");
        self.header("Content-Length", &self.body.bytes().len().to_string());
        self.header("Date", &Local::now().to_rfc2822());
    }

    pub fn compose(self) -> String {

        let headers = {
            if self.headers.is_empty() {
                "".to_string()
            } else {
                self.headers.into_iter().map(|x| format!("{}: {}\r\n", x.0, x.1)).collect()
            }
        };

        format!("{} {}\r\n{}\r\n{}", self.protocol_version, self.response_code.compose(), headers, self.body)
    }
}
