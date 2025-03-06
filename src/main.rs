use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory_path = if args.len() > 2 && args[1] == "--directory" {
        args[2].clone()
    } else {
        String::from("./")
    };

    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory_path_clone = directory_path.clone();
                std::thread::spawn(move || handle_client(stream, directory_path_clone));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, directory_path: String) {
    println!("accepted new connection");

    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf).unwrap();

    if bytes_read == 0 {
        return;
    }

    // Example Request
    // GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n

    let request_str = String::from_utf8_lossy(&buf[0..bytes_read]);
    println!("Received request:\n{}", request_str);

    let request_lines: Vec<&str> = request_str.split("\r\n").collect();
    if request_lines.is_empty() {
        return;
    }

    let parts: Vec<&str> = request_lines[0].split_whitespace().collect();
    let path = parts[1];

    match path {
        "/" => {
            stream
                .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .unwrap();
        }
        _ => {
            if path.starts_with("/echo/") {
                let echo_content = &path[6..];
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    echo_content.len(),
                    echo_content,
                );
                stream.write_all(response.as_bytes()).unwrap()
            } else if path.starts_with("/user-agent") {
                let user_agent = request_lines[2].split(": ").nth(1).unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent,
                );
                stream.write_all(response.as_bytes()).unwrap()
            } else if path.starts_with("/files/") {
                let file_name = &path[7..];
                let file_path = format!("{}/{}", directory_path, file_name);

                match std::fs::read(&file_path) {
                    Ok(file_content) => {
                        let header = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                            file_content.len(),
                        );
                        stream.write_all(header.as_bytes()).unwrap();
                        stream.write_all(&file_content).unwrap();
                    }
                    Err(_) => {
                        stream
                            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .unwrap();
                    }
                }
            } else {
                stream
                    .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .unwrap();
            }
        }
    }
}
