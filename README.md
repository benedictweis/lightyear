# lightyear ☄️

A lightning fast web server written in rust, inspired by express syntax

## Usage

Express like syntax

```rust
use lightyear::Lightyear;

let mut app = Lightyear::new();
let port = 3000;

app.get("/", | req, res | {
    res.send("Hello World!");
});

app.listen(port, | | {
    println!("Example app listening on port {}", port);
});
```
