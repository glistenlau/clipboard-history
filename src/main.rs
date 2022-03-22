mod clipboard_capturer;
mod constants;
mod history_accessor;
mod history_dao;
mod utils;

use std::io;
use std::thread;
use std::time::Duration;
use std::vec;

use anyhow::Result;
use chrono::Date;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Utc;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use history_accessor::get_all;
use log::info;
use shine_library::core::log::setup_logger;
use shine_library::proxy::rocksdb::get_conn;
use shine_library::proxy::rocksdb::RocksMergeOp;

use utils::get_local_datetime_content;

use crate::constants::CF;
use crate::constants::MERGE_OP;
use crate::constants::PATH;
use crate::history_accessor::get_history_on_date;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    thread::spawn(|| {
        clipboard_capturer::start_capturing();
    });

    loop {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer);
        let input_str = buffer.trim();

        let date_results: Vec<String>;
        if input_str.to_lowercase().contains("today") {
            date_results = get_history_on_date(Local::today()).unwrap();
        } else if input_str.to_lowercase().contains("all") {
            date_results = get_all().unwrap();
        } else {
            let naive_date = NaiveDate::parse_from_str(&input_str, "%Y-%m-%d").unwrap();
            let offset = Local.offset_from_utc_date(&naive_date);
            let date: Date<Local> = Date::from_utc(naive_date, offset);
            date_results = get_history_on_date(date).unwrap();
        }
        date_results
            .iter()
            .for_each(|content| println!("{}", get_local_datetime_content(content)));
    }
}
