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

pub fn get_local_datetime_content(content: &str) -> String {
    let datetime_utc_str = &content[1..28];
    let datetime: DateTime<Local> = DateTime::parse_from_rfc3339(datetime_utc_str)
        .unwrap()
        .with_timezone(&Local);

    format!(
        "[{}]{}",
        &datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        &content[29..]
    )
}
