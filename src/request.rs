use crate::method::Method;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Request {
    pub raw: String,
    pub method: Method,
    pub path: String,
    protocol_version: String,
    pub headers: HashMap<String, String>,
    pub raw_params: String,
    pub params: HashMap<String, String>,
    _body: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            raw: "".into(),
            method: Method::GET,
            path: "".into(),
            protocol_version: "".into(),
            headers: HashMap::new(),
            raw_params: "".into(),
            params: HashMap::new(),
            _body: "".into(),
        }
    }
}

impl FromStr for Request {
    type Err = ();

    fn from_str(input: &str) -> Result<Request, Self::Err> {
        let mut lines: Vec<&str> = input.lines().collect();
        let mut request = Request::new();

        let split: Vec<_> = lines.remove(0).split(' ').collect();

        request.method = Method::from_str(split[0]).unwrap();
        request.path = split[1].into();
        request.protocol_version = split[2].into();

        for line in lines {
            let line_split: Vec<_> = line.split(": ").collect();
            request.headers.insert(line_split[0].into(), line_split[1].into());
        }
        Ok(request)
    }
}

fn parse_params(raw_params: String) -> Option<HashMap<String, String>> {
    None
}
