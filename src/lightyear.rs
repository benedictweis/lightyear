use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::str::FromStr;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::routes::Routes;

pub struct Lightyear {
    routes: Routes,
}

impl Lightyear {
    pub fn new() -> Lightyear{
        Lightyear { routes: Routes::new() }
    }

    pub fn get<F: 'static>(&mut self, path: &str, function: F) where F: Fn(Request, &mut Response) {
        self.routes.get.insert(path.into(), Box::new(function));
    }

    pub fn listen<F>(&self, port: usize, function: F) where F: Fn() {

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        function();

        for stream in listener.incoming() {

            let mut stream = stream.unwrap();

            let buf_reader = BufReader::new(&mut stream);
            let http_request: String = buf_reader.lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect::<Vec<String>>()
                .join("\n");

            let request = Request::from_str(&http_request).unwrap();
            let mut response = Response::new();

            let closure = match request.method {
                Method::GET => {
                    self.routes.get.get(&request.path).unwrap_or_else(|| self.routes.errors.get("404").unwrap())
                }
                Method::POST => {
                    self.routes.post.get(&request.path).unwrap_or_else(|| self.routes.errors.get("404").unwrap())
                }
                _ => self.routes.errors.get("405").unwrap(),
            };

            closure(request, &mut response);


            stream.write_all(response.compose().as_bytes()).unwrap();
        }
    }
}
