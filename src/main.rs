use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf).unwrap();

    if bytes_read == 0 {
        return;
    }

    let request_str = String::from_utf8_lossy(&buf[0..bytes_read]);
    println!("Received request:\n{}", request_str);

    let request_lines: Vec<&str> = request_str.split("\r\n").collect();
    if request_lines.is_empty() {
        return;
    }

    let request_line = request_lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let path = parts[1];
    if path == "/" {
        stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();
    } else {
        stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap();
    }
}
