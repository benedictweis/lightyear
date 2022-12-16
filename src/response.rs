
pub struct Response();

impl Response {
    fn new() -> Response {
        Response()
    }

    pub fn send(&self, body_str: &str) {
        println!("Sending stuff!")
    }
}
