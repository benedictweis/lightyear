use crate::lightyear::Lightyear;

mod lightyear;
mod routes;
mod request;
mod response;
mod method;

fn main() {
    let mut app = Lightyear::new();
    let port = 3000;

    app.get("/", | req, res | {
        res.send(&format!("Hello World! You are {}", req.headers.get("User-Agent").unwrap_or(&"No one".to_string())));
    });

    app.listen(port, | | {
        println!("Example app listening on port {}", port);
    });
}