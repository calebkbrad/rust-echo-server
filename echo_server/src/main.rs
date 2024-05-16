use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let ip = "0.0.0.0";
    let port = 8080;
    let address = format!("{ip}:{}", port.to_string());

    let listener = TcpListener::bind(address).expect("Error in creating listener");

    println!("Listening on port {port}");

    for connection in listener.incoming() {
        println!("Connection success");
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