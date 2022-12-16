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
        println!("Getting things!");
        self.routes.get.insert(path.into(), Box::new(function));
    }

    pub fn listen<F>(&self, port: usize, function: F) where F: Fn() {
        println!("Listening!");
    }
}
