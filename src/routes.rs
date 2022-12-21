use std::collections::HashMap;
use crate::request::Request;
use crate::response::Response;

type RouteStorage = HashMap<String, Box<dyn Fn(Request, &mut Response)>>;

pub(crate) struct Routes {
    pub(crate) get: RouteStorage,
    pub(crate) post: RouteStorage,
    pub(crate) errors: RouteStorage,
}

impl Routes {
    pub(crate) fn new() -> Routes {

        let not_found = Box::new(|req: Request, res: &mut Response| {
            res.send(&format!("404 File Not Found ({})", req.path));
        });

        let unsupported = Box::new(|req: Request, res: &mut Response| {
            res.send(&format!("405 Method Not Allowed ({})", req.method));
        });

        let mut errors: RouteStorage = HashMap::new();
        errors.insert("404".to_string(), not_found);
        errors.insert("405".to_string(), unsupported);

        Routes { get: HashMap::new(), post: HashMap::new() , errors }
    }
}