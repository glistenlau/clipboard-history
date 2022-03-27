use chrono::{Date, DateTime, Duration, TimeZone};

use crate::history_dao::{self, get_between_datetime};

use anyhow::Result;

pub fn get_all() -> Result<Vec<String>> {
    history_dao::get_all()
}

pub fn get_history_on_date<Tz: TimeZone>(date: Date<Tz>) -> Result<Vec<String>> {
    let datetime: DateTime<Tz> = date.and_hms(0, 0, 0);
    let datetime_next_day: DateTime<Tz> = (date + Duration::days(1)).and_hms(0, 0, 0);

    get_between_datetime(datetime, datetime_next_day)
}
