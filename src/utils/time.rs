use chrono::{DateTime, Datelike, TimeZone, Utc};

pub fn to_day_bounds(date: &DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let start = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap();

    (start, start + chrono::Duration::days(1))
}
