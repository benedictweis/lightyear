use crate::response_code::ResponseCode;
use std::collections::HashMap;

pub struct Response {
    protocol_version: String,
    response_code: ResponseCode,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            protocol_version: "HTTP/1.1".into(),
            response_code: ResponseCode::OK,
            headers: HashMap::new(),
            body: "".into(),
        }
    }

    pub fn send(&mut self, body_str: &str) {
        self.body = body_str.into();
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

    pub fn compose(self) -> String {
        let headers = {
            if self.headers.is_empty() {
                "".to_string()
            } else {
                self.headers.iter().map(|x| format!("{}: {}\r\n", x.0, x.1)).collect()
            }
        };

        format!("{} {}\r\n{}\r\n{}", self.protocol_version, self.response_code.compose(), headers, self.body)
    }
}
