use std::{
    fs,
    io::{prelude::*, BufReader}, // enable reading and writing to the TcpStream
    net::TcpStream 
};

/// 
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // add buffering to the reader, improves the speed of small and repeated read calls to the same file or network socket.

    /* parse the stream into a vector of strings 
        ** NOTE, all instances of unwrap() should be replaced with proper error handling in production code
    */
    let http_request: Vec<_> = buf_reader // take the buffer
        .lines() // return an iterator over the lines
        .map(|result| result.unwrap()) // get each string, return an error if data isn't UFT-8
        .take_while(|line| !line.is_empty()) // filter out empty lines
        .collect();
    
    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK"; // http version 1.1, status code 200, OK reason phrase
    let contents = fs::read_to_string("public/index.html").unwrap(); // does what it says on the box
    let length = contents.len();

    /*
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body 
    */
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); 
    stream
        .write_all(response.as_bytes()) // convert response to bytes and send down the connection
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_works() {
        println!("{:?}", fs::read_to_string("public/index.html").unwrap())
    }
}