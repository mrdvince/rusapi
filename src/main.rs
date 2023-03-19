use std::net::TcpListener;

use rusapi::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed binding to port specified");
    run(listener)?.await
}
