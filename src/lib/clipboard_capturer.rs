use anyhow::Result;
use chrono::DateTime;

use chrono::Utc;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use log::info;

use shine_library::proxy::rocksdb::get_conn;

use shine_library::proxy::rocksdb::RocksdbProxy;

use crate::constants::CF;
use crate::constants::MERGE_OP;
use crate::constants::PATH;
use crate::utils::get_save_key;
use crate::utils::get_save_value;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};

use std::io;

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        match ctx.get_contents() {
            Ok(content) => {
                log::debug!("got new clipboard content: {}", &content);
                let datetime: DateTime<Utc> = Utc::now();
                let save_key = get_save_key(&datetime);
                let save_value = get_save_value(&datetime, &content);

                if let Err(e) = save_to_rocks(&save_key, &save_value) {
                    info!("save to rocks error: {}", e);
                }
            }
            Err(err) => {
                log::error!("get clipboard content error: {}", err);
            }
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        log::error!("Clipboard error: {}", error);
        CallbackResult::Next
    }
}

fn save_to_rocks(key: &str, val: &str) -> Result<()> {
    let mut db = get_conn(PATH, MERGE_OP);

    RocksdbProxy::append_to_array(CF, key, val, &mut db)
}

pub fn start_capturing() {
    let _ = Master::new(Handler).run();
}
