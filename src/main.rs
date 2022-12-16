use crate::lightyear::Lightyear;

mod lightyear;
mod routes;
mod request;
mod response;

fn main() {
    let mut app = Lightyear::new();
    let port = 3000;

    app.get("/", | req, res | {
        res.send("Hello World!");
    });

    app.listen(port, | | {
        println!("Example app listening on port {}", port);
    });
}