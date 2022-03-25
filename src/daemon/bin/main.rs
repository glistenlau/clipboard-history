use lib::clipboard_capturer;
use lib::history_activity::query_history;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    thread::spawn(|| {
        clipboard_capturer::start_capturing();
    });

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..size]);

    let query_result = query_history(&request_str);

    stream
        .write(serde_json::to_string(&query_result).unwrap().as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
