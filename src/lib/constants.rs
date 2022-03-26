use shine_library::proxy::rocksdb::RocksMergeOp;

pub const PATH: &str = "./.clipboard_history/data";
pub const CF: Option<&str> = Some("CLIPBOARD_HISTORY");
pub const MERGE_OP: Option<RocksMergeOp> = Some(RocksMergeOp::JsonArray);
