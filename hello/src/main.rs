use hello::handle_connection;
use std::net::TcpListener; // The std::net module provides networking functionality for TCP

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878") // bind an instance of TcpListener to port 7878
        .unwrap(); // production ready code would require a full error handler

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // production ready code would require a full error handler

        println!("Connection established!");
        handle_connection(stream);
    } // when stream is dropped, the connection is closed
}
