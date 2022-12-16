use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener};
use std::str::FromStr;
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

    pub fn get<F: 'static>(&mut self, path: &str, function: F) where F: Fn(Request, Response) {
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

            println!("{:?}", Request::from_str(&http_request));

            let response = "HTTP/1.1 200 OK\r\n\r\n";


            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
