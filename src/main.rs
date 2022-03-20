use std::thread;
use std::time::Duration;

use anyhow::Result;
use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use log::info;
use shine_library::core::log::setup_logger;
use shine_library::proxy::rocksdb::RocksdbProxy;
use shine_library::proxy::rocksdb::RocksMergeOp;
use shine_library::proxy::rocksdb::get_conn;

const PATH: &str = "./data";
const CF: &str = "CLIPBOARD_HISTORY";
const MERGE_OP: Option<RocksMergeOp> = Some(RocksMergeOp::JsonArray);

fn save_to_rocks(key: &str, val: &str) -> Result<()> {
    let mut db = get_conn(PATH, Some(RocksMergeOp::JsonArray));

    RocksdbProxy::append_to_array(Some(CF), key, val, &mut db)
}

fn get_all_dates() {
    let mut db = get_conn(PATH, MERGE_OP);
    let result = RocksdbProxy::query_range_array_forward::<&str>(Some(CF), None, None, &mut db);
    println!("{:#?}", result);
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut current_content = String::new();
    match setup_logger("./log") {
        Ok(_) => info!("setup logger successfully."),
        Err(e) => println!("setup logger failed: {}", e),
    }
    get_all_dates();
    // loop {
    //     match ctx.get_contents() {
    //         Ok(content) => {
    //             if !content.eq(&current_content) {
    //                 current_content = content;
    //                 let datetime: DateTime<Utc> = Utc::now();
    //                 let save_value = format!("[{:?}] {}", datetime, current_content,);
    //                 println!("[{:?}] {}", datetime.with_timezone(&Local), current_content);
    //                 if let Err(e) = save_to_rocks(&format!("{:?}", datetime.date()), &save_value) {
    //                     info!("save to rocks error: {}", e);
    //                 }
    //             }
    //         }
    //         Err(err) => println!("error: {:?}", err),
    //     }
    //     thread::sleep(Duration::from_millis(100))
    // }
}
