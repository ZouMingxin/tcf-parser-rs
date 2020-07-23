use chrono::{DateTime, NaiveDateTime, Utc};

pub(crate) fn from_i64_to_datetime(i: i64) -> DateTime<Utc> {
    let naive_date_time =
        NaiveDateTime::from_timestamp((i / 10) as i64, ((i % 10) * 100_000_000) as u32);

    DateTime::<Utc>::from_utc(naive_date_time, Utc)
}

pub(crate) fn from_u8_to_char(i: u8) -> char {
    (65 + i).into()
}
