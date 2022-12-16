use std::collections::HashMap;
use crate::request::Request;
use crate::response::Response;

pub(crate) struct Routes {
    pub(crate) get: HashMap<String, Box<dyn Fn(Request, Response)>>,
    pub(crate) post: HashMap<String, Box<dyn Fn(Request, Response)>>,
}

impl Routes {
    pub(crate) fn new() -> Routes {
        Routes { get: HashMap::new(), post: HashMap::new() }
    }
}