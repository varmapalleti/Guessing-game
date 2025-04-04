use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;



fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("server is listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    use std::path::Path;

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    let path = if path == "/" {
        "index.html".to_string()
    } else {
        path.trim_start_matches('/').to_string()
    };

    let file_path = Path::new(&path);

    let (status_line, contents) = match fs::read(&file_path) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents),
        Err(_) => {
            let contents = fs::read("404.html").unwrap_or_else(|_| b"404 Not Found".to_vec());
            ("HTTP/1.1 404 NOT FOUND", contents)
        }
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        contents.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
    stream.flush().unwrap();
}
