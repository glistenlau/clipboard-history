use lib::clipboard_capturer;
use lib::history_activity::query_history;
use lib::utils::get_base_folder;
use lib::utils::get_tcp_address;
use shine_library::core::log::setup_logger;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    match setup_logger(&format!("{}/log", get_base_folder())) {
        Ok(_) => log::info!("setup logger successfully."),
        Err(e) => println!("setup logger failed: {}", e),
    }

    thread::spawn(|| {
        clipboard_capturer::start_capturing();
    });

    let tcp_address = get_tcp_address();

    log::info!("start tcp server on {}", &tcp_address);

    let listener = TcpListener::bind(tcp_address).unwrap();

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
