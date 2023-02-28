use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;

const ERROR_NOT_FOUND: fn(Request, &mut Response) = |req: Request, res: &mut Response| {
    res.status(404);
    res.send(&format!("404 File Not Found ({})", req.path));
};

const ERROR_NOT_ALLOWED: fn(Request, &mut Response) = |req: Request, res: &mut Response| {
    res.status(405);
    res.send(&format!("405 Method Not Allowed ({})", req.method));
};

const ERROR_INTERNAL_SERVER: fn(Request, &mut Response) = |req: Request, res: &mut Response| {
    res.status(500);
    res.send(&format!("500 Internal Server Error ({})", req.method));
};

type RouteStorage = HashMap<String, Box<dyn Fn(Request, &mut Response)>>;

pub struct Routes {
    pub get: RouteStorage,
    pub post: RouteStorage,
    pub errors: RouteStorage,
    pub static_files: HashMap<String, String>,
}

impl Routes {
    pub fn new() -> Routes {
        let mut errors: RouteStorage = HashMap::new();
        errors.insert("404".to_string(), Box::new(ERROR_NOT_FOUND));
        errors.insert("405".to_string(), Box::new(ERROR_NOT_ALLOWED));
        errors.insert("500".to_string(), Box::new(ERROR_INTERNAL_SERVER));

        Routes {
            get: HashMap::new(),
            post: HashMap::new(),
            errors,
            static_files: HashMap::new(),
        }
    }
}
