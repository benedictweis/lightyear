use std::collections::HashMap;
use std::str::FromStr;
use crate::method::Method;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    protocol_version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub(crate) fn new() -> Request{
        Request { method: Method::GET, path: "".into(), protocol_version: "".into(), headers: HashMap::new(), body: "".into() }
    }
}

impl FromStr for Request {
    type Err = ();

    fn from_str(input: &str) -> Result<Request, Self::Err> {
        let mut lines: Vec<&str> = input.lines().collect();
        let mut request = Request::new();

        let split: Vec<_> = lines.remove(0).split( ' ').collect();

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