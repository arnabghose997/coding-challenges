use std::{
    io::{BufRead, BufReader, Read, Write}, 
    net::TcpListener
};

/// Returns standard HTTP/1.1 reponse
fn get_standard_response() -> Vec<u8> {
    let body = "hello";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let response_bytes = response.into_bytes();
    return response_bytes;
}

/// Runs the load balancer server
pub fn run_server() {
    let port: &str = "80";
    let host: &str = "localhost";
    let addr: String = format!("{}:{}", &host, &port);
    let listener = TcpListener::bind(addr).unwrap_or_else(|_| panic!("unable to bind port {}", port));

    loop {
        match listener.accept() {
            Ok((mut socket, _)) => {
                // Read the Request headers from the socket
                let mut socket_buffer = BufReader::new(&mut socket);
                for line in socket_buffer.by_ref().lines() {
                    let line = line.unwrap();
                    if line.is_empty() {
                        break;
                    }
                    println!("{}", line)
                }

                // Send back "hello" as response 
                let standard_response = get_standard_response();
                socket.write_all(&standard_response).unwrap();
            },
            Err(e) => println!("Some error: {e:?}")
        }
    }
}