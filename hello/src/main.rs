use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader}, // enable reading and writing to the TcpStream
    net::{TcpListener, TcpStream}, // The std::net module provides networking functionality for TCP
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878") // bind an instance of TcpListener to port 7878
        .unwrap(); // production ready code would require a full error handler
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // production ready code would require a full error handler

        println!("Connection established!");
        /* Comment out lines 20 and 23 to make the server singlethreaded */
        pool.execute(|| {
            handle_connection(stream);
        });
    } // when stream is dropped, the connection is closed

    println!("Shutting down.");
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // add buffering to the reader, improves the speed of small and repeated read calls to the same file or network socket.

    /* parse the stream into a vector of strings
     **NOTE, all instances of unwrap() should be replaced with proper error handling in production code
     */
    let http_request: Vec<_> = buf_reader // take the buffer
        .lines() // return an iterator over the lines
        .map(|result| result.unwrap()) // get each string, return an error if data isn't UFT-8
        .take_while(|line| !line.is_empty()) // filter out empty lines
        .collect();

    let (status_line, filename) = match &http_request[0][..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap(); // does what it says on the box
    let length = contents.len();

    /* response follows:
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
    */
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream
        .write_all(response.as_bytes()) // convert response to bytes and send down the connection
        .unwrap()
}
