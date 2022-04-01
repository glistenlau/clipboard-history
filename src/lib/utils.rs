use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, TimeZone, Utc};

pub fn get_save_key<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String {
    format!("{:?}", datetime.with_timezone(&Utc).date())
}

pub fn get_save_value<Tz: TimeZone>(datetime: &DateTime<Tz>, content: &str) -> String {
    format!("[{:?}] {}", datetime.with_timezone(&Utc), content)
}

pub fn get_value_datetime_str<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String {
    format!("{:?}", datetime.with_timezone(&Utc))
}

pub fn get_local_datetime_content(content: &str) -> Result<String> {
    let right = match content.find('Z') {
        Some(pos) => pos,
        None => return Err(anyhow!("Invalid content format.")),
    };

    let datetime_utc_str = &content[1..right + 1];
    let datetime: DateTime<Local> =
        DateTime::parse_from_rfc3339(datetime_utc_str)?.with_timezone(&Local);

    let local_datetime_content = format!(
        "[{}]{}",
        &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        &content[29..]
    );
    Ok(local_datetime_content)
}

pub fn get_tcp_address() -> String {
    if cfg!(debug_assertions) {
        "127.0.0.1:8888".to_string()
    } else {
        "127.0.0.1:7878".to_string()
    }
}

pub fn get_base_folder() -> &'static str {
    "./.clipboard_history"
}
