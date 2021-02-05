use std::net;
use zero2prod;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    let listener = net::TcpListener::bind(&format!("127.0.0.1:{}", &port))
        .expect(&format!("Failed to bind to port {}.", port));
    zero2prod::run(listener)?.await
}
