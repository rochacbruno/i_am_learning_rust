extern crate actix_web;
use actix_web::{http, server, App, Path};

fn index(info: Path<(u32, String)>) -> String {
    format!("Hello {}! id:{}", info.1, info.0)
}

// http://localhost:8080/50/bruno/index.html
fn main() {
    server::new(
        || App::new()
            .route("/{id}/{name}/index.html", http::Method::GET, index))
        .bind("localhost:8080").unwrap()
        .run();
}

// Hello bruno! id:50


// cargo watch -x run
