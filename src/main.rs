use std::net::TcpListener;
use std::io::{Write, Read};
fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();
    println!("Servidor ejecutándose en http://localhost:8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\n\r\n¡Hola, mundo desde Rust!";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
