use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // create a socket listener binded to port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // if client connects to the socket, then this part runs
    for stream in listener.incoming() {
        // stream is open connection between server and client
        let stream = stream.unwrap();

        // we call handle_connection function to handle requests from client
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // HTTP Request is text based requet. so stream itself is a string.
    println!("Request: {:#?}", request_line);

    // handle request respectively to request_line
    if request_line == "GET / HTTP/1.1" {
        // create response and put it into the stream
        // at this point, stream is a magic door that connects server and client
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/public/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        println!("Response: {:#?}", response);

        // send bytes of response to the stream
        return stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("src/public/404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        return stream.write_all(response.as_bytes()).unwrap();
    }
}
