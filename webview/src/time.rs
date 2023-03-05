use chrono::offset::Local;
use chrono::NaiveDateTime;

pub fn current_timestamp() -> i64 {
    Local::now().timestamp_millis()
}

pub fn timestamp_to_isodate(timestamp: i64) -> String {
    NaiveDateTime::from_timestamp_millis(timestamp)
        .expect("Invalid datetime")
        .format("%Y-%m-%d")
        .to_string()
}

pub fn isodate_to_timestamp(isodate: String) -> i64 {
    NaiveDateTime::parse_from_str(&isodate, "%Y-%m-%d")
        .expect("Invalid datetime")
        .timestamp_millis()
}
