mod tests;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use rust_tutorial_webserver::ThreadPool;
fn main() {
    // create a socket listener binded to port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);

    // if client connects to the socket, then this part runs
    for stream in listener.incoming() {
        // stream is open connection between server and client
        let stream = stream.unwrap();

        // create a new thread for each request
        // this is not a good way due to exposure to DDOS attack
        pool.execute(|| {
            // we call handle_connection function to handle requests from client
            handle_connection(stream);
        });
    }

    println!("shutting down server");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // HTTP Request is text based requet. so stream itself is a string.
    println!("Request: {:#?}", request_line);

    let html_page_dir = "src/public";

    // handle request respectively to request_line

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", format!("{html_page_dir}/index.html")),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", format!("{html_page_dir}/index.html"))
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            format!("{html_page_dir}/404.html"),
        ),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
