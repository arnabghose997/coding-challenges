use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    pub ip: String,
    pub request_type: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
}

pub fn run_server() {
    let port: &str = "80";
    let addr: String = format!("localhost:{}", &port);
    let listener = TcpListener::bind(addr).unwrap_or_else(|_| panic!("unable to bind port {}", port));

    match listener.accept() {
        Ok(_) => println!("hii"),
        Err(e) => println!("Some error: {e:?}")
    }
}