use lightyear::Lightyear;

fn main() {
    let mut app = Lightyear::new();
    let port = 3000;

    app.get("/", |req, res| {
        res.send(&format!("Hello World! You are {}", req.headers.get("User-Agent").unwrap_or(&"No one".to_string())));
    });

    app.get("/test/:id", |_req, res| {});

    app.listen(port, || {
        println!("Example app listening on port {}", port);
    });
}
