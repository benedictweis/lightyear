#![allow(clippy::new_without_default)]

mod method;
mod mime;
mod request;
mod response;
mod response_code;
mod routes;

use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::routes::Routes;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str::FromStr;

pub struct Lightyear {
    routes: Routes,
}

impl Lightyear {
    pub fn new() -> Lightyear {
        Lightyear { routes: Routes::new() }
    }

    pub fn get<F: 'static>(&mut self, path: &str, function: F)
    where
        F: Fn(Request, &mut Response),
    {
        self.routes.get.insert(path.into(), Box::new(function));
    }

    pub fn error<F: 'static>(&mut self, error_code: &str, function: F)
    where
        F: Fn(Request, &mut Response),
    {
        self.routes.errors.insert(error_code.into(), Box::new(function));
    }

    pub fn static_files(&mut self, path: &str, local_path: &Path) {
        self.routes.static_files.insert(path.into(), local_path.canonicalize().unwrap().as_os_str().to_str().unwrap().into());
    }

    pub fn listen<F>(&self, port: usize, function: F)
    where
        F: Fn(),
    {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        function();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let http_request = read_stream_to_string(&mut stream);

            let request = Request::from_str(&http_request).unwrap();
            let mut response = Response::new();

            let closure = self.find_matching_closure(&request);

            closure(request.clone(), &mut response);

            if response.error != "0" {
                let error = self.routes.errors.get(&response.error).unwrap_or_else(|| self.routes.errors.get("500").unwrap());
                error(request, &mut response);
            }

            response.compose_default_headers();

            stream.write_all(response.compose().as_bytes()).unwrap();
        }
    }

    fn find_matching_closure(&self, request: &Request) -> &Box<dyn Fn(Request, &mut Response)> {
        let closure = match request.method {
            Method::GET => self.routes.get.get(&request.path).unwrap_or_else(|| self.routes.errors.get("404").unwrap()),
            Method::POST => self.routes.post.get(&request.path).unwrap_or_else(|| self.routes.errors.get("404").unwrap()),
            _ => self.routes.errors.get("405").unwrap(),
        };
        closure
    }
}

fn read_stream_to_string(mut stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: String = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect::<Vec<String>>().join("\n");
    http_request
}
