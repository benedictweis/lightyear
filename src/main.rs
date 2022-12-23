#![allow(clippy::new_without_default)]

use crate::lightyear::Lightyear;

mod lightyear;
mod method;
mod request;
mod response;
mod response_code;
mod routes;

fn main() {
    let mut app = Lightyear::new();
    let port = 3000;

    app.get("/", |req, res| {
        res.send(&format!("Hello World! You are {}", req.headers.get("User-Agent").unwrap_or(&"No one".to_string())));
    });

    app.get("/redirect", |_req, res| {
        res.redirect("/");
    });

    app.listen(port, || {
        println!("Example app listening on port {}", port);
    });
}
