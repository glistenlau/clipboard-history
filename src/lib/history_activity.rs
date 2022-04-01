use chrono::{Date, Local, NaiveDate, TimeZone};

use crate::{
    history_accessor::{get_all, get_history_on_date},
    utils::get_local_datetime_content,
};

fn handle_results(x: anyhow::Error) -> Vec<String> {
    log::error!("query history error: {}", x);
    vec![]
}

pub fn query_history(input_str: &str) -> Vec<String> {
    let date_results: Vec<String>;
    let input_str_trim = input_str.trim().to_lowercase();

    if input_str_trim.to_lowercase().contains("today") {
        date_results = get_history_on_date(Local::today()).unwrap_or_else(handle_results);
    } else if input_str_trim.to_lowercase().contains("all") {
        date_results = get_all().unwrap_or_else(handle_results);
    } else {
        date_results = match NaiveDate::parse_from_str(&input_str_trim, "%Y-%m-%d") {
            Ok(naive_date) => {
                let offset = Local.offset_from_utc_date(&naive_date);
                let date: Date<Local> = Date::from_utc(naive_date, offset);
                get_history_on_date(date).unwrap_or_else(handle_results)
            }
            Err(e) => {
                log::error!("parse date {} error: {}", &input_str_trim, e);
                vec![]
            }
        }
    }

    date_results
        .iter()
        .map(|content| match get_local_datetime_content(content) {
            Ok(local_content) => local_content,
            Err(e) => {
                log::error!(
                    "Convert content {} to local datetime content error: {}",
                    content,
                    e
                );
                content.to_string()
            }
        })
        .collect()
}
