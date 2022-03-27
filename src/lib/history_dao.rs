use anyhow::{anyhow, Result};
use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;

use shine_library::proxy::rocksdb::get_conn;

use shine_library::proxy::rocksdb::RocksdbProxy;

use crate::constants::CF;
use crate::constants::MERGE_OP;
use crate::constants::PATH;
use crate::utils::get_save_key;
use crate::utils::get_value_datetime_str;

pub fn get_all() -> Result<Vec<String>> {
    let mut db = get_conn(PATH, MERGE_OP);
    let result = RocksdbProxy::query_range_array_forward::<&str>(CF, None, None, &mut db)?;

    let flatten_result: Vec<String> = flat_multi_results(result);

    Ok(flatten_result)
}

pub fn get_between_datetime<T: TimeZone, Z: TimeZone>(
    from: DateTime<T>,
    to: DateTime<Z>,
) -> Result<Vec<String>> {
    if from >= to {
        return Err(anyhow!("invalid range."));
    }

    let start_value_str = format!("[{} ", get_value_datetime_str(&from));
    let end_value_str = format!("[{} ", get_value_datetime_str(&to));
    let from_key = get_save_key(&from);
    let to_key = get_save_key(&(to + Duration::days(1)));

    let mut db = get_conn(PATH, MERGE_OP);
    let result =
        RocksdbProxy::query_range_array_forward(CF, Some(&from_key), Some(&to_key), &mut db)?;

    let mut flatten_result: Vec<String> = flat_multi_results(result);

    let start_index = flatten_result.partition_point(|x| x < &start_value_str);
    let end_index = flatten_result.partition_point(|x| x < &end_value_str);

    Ok(flatten_result.drain(start_index..end_index).collect())
}

fn flat_multi_results(mut results: Vec<(String, Result<Vec<String>>)>) -> Vec<String> {
    results
        .drain(..)
        .flat_map(|(_, val)| match val {
            Ok(val_vec) => val_vec,
            Err(e) => {
                log::error!("encounter value error in get_all: {}", e);
                vec![]
            }
        })
        .collect()
}
