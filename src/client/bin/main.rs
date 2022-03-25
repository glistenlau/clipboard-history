use clap::Parser;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The date of the clipboard history, like today, 2022-03-24, etc.
    #[clap(default_value = "today")]
    date: String,
}

fn main() {
    let args = Args::parse();

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write(args.date.as_bytes()).unwrap();

    let mut buffer = String::new();
    let size = stream.read_to_string(&mut buffer).unwrap();

    let history: Vec<String> = serde_json::from_str(&buffer).unwrap();

    history.iter().for_each(|content| println!("{}", content));
}
