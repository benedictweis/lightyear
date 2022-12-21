
pub struct Response {
    protocol_version: String,
    response_code: String,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        Response { protocol_version: "HTTP/1.1".into(), response_code: "200 OK".into(), body: "".into() }
    }

    pub fn send(&mut self, body_str: &str) {
        self.body = body_str.into();
    }

    pub fn compose(&self) -> String {
        format!("{} {}\r\n\r\n{}", self.protocol_version, self.response_code, self.body)
    }
}
