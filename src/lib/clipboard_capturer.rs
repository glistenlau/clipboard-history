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
use shine_library::proxy::rocksdb::get_conn;
use shine_library::proxy::rocksdb::RocksMergeOp;
use shine_library::proxy::rocksdb::RocksdbProxy;

use crate::constants::CF;
use crate::constants::MERGE_OP;
use crate::constants::PATH;
use crate::utils::get_save_key;
use crate::utils::get_save_value;

fn save_to_rocks(key: &str, val: &str) -> Result<()> {
    let mut db = get_conn(PATH, MERGE_OP);

    RocksdbProxy::append_to_array(CF, key, val, &mut db)
}

pub fn start_capturing() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut current_content = String::new();
    let mut started = false;

    match setup_logger("./log") {
        Ok(_) => info!("setup logger successfully."),
        Err(e) => println!("setup logger failed: {}", e),
    }
    loop {
        match ctx.get_contents() {
            Ok(content) => {
                if !content.eq(&current_content) {
                    current_content = content;

                    if !started {
                        started = true;
                        continue;
                    }

                    let datetime: DateTime<Utc> = Utc::now();
                    let save_key = get_save_key(&datetime);
                    let save_value = get_save_value(&datetime, &current_content);

                    if let Err(e) = save_to_rocks(&save_key, &save_value) {
                        info!("save to rocks error: {}", e);
                    }
                }
            }
            Err(err) => {}
        }
        thread::sleep(Duration::from_millis(100))
    }
}
