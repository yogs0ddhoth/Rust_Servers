use std::{
    io::{prelude::*, BufReader}, // enable reading and writing to the TcpStream
    net::TcpStream 
};

/// 
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // add buffering to the reader, improves the speed of small and repeated read calls to the same file or network socket.

    /* parse the stream into a vector of strings 
        ** NOTE, the unwrap() in the closure passed to map() should be replaced with proper error handling in production code
    */
    let http_request: Vec<_> = buf_reader // take the buffer
        .lines() // return an iterator over the lines
        .map(|result| result.unwrap()) // get each string, return an error if data isn't UFT-8
        .take_while(|line| !line.is_empty()) // filter out empty lines
        .collect();
    
    println!("Request: {:#?}", http_request);
}