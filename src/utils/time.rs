use chrono::{DateTime, Datelike, TimeZone, Utc};

pub fn to_day_bounds(date: &DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap();
    let end = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
        .unwrap();

    (start, end)
}
