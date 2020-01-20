use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Bind to port
    let uri = "127.0.0.1:7878";
    let listener = match TcpListener::bind(uri) {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind to {} [{}]", uri, e);
            std::process::exit(1);
        }
    };

    // Handle connections as they are established
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("Error opening connection [{}]", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read request
    let mut buffer = [0; 512];
    if let Err(e) = stream.read(&mut buffer) {
        println!("Failed to read stream, exiting [{}]", e);
        return;
    }

    // Load response from file
    let contents_path = "hello.html";
    let contents = match fs::read_to_string(contents_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Failed to read {}, exiting [{}]", contents_path, e);
            return;
        }
    };

    // Write response to stream
    let response = format!(
        "HTTP/1.1 200 OK\rContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    if let Err(e) = stream.write(response.as_bytes()) {
        println!("Failed to write response to stream, exiting [{}]", e);
        return;
    }

    // Wait for all bytes to finish writing to the connection
    if let Err(e) = stream.flush() {
        println!("Failed to flush stream, exiting [{}]", e);
        return;
    }
}
