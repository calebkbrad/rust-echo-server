use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error in creating listener");

    println!("Listening on port 8080");

    for connection in listener.incoming() {
        let connection = connection.unwrap();
        echo_response(connection);
    }
}

fn echo_response(mut stream: TcpStream) {
    let request_reader = BufReader::new(&mut stream);

    let request_lines: Vec<_> = request_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = request_lines.join("\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}