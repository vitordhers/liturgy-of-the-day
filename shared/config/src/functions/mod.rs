use chrono::Datelike;
use chrono::{Duration, NaiveDate, Weekday};
pub fn calculate_easter_sunday(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = ((h + l - 7 * m + 114) / 31) as u32;
    let day = (((h + l - 7 * m + 114) % 31) + 1) as u32;

    NaiveDate::from_ymd_opt(year, month, day).expect("easter day to exist in calendar")
}

pub fn next_weekday_after(date: NaiveDate, weekday: Weekday) -> NaiveDate {
    let mut next_date = date;
    while next_date.weekday() != weekday {
        next_date += Duration::days(1);
    }
    next_date
}

pub fn previous_weekday_before(date: NaiveDate, weekday: Weekday) -> NaiveDate {
    let mut prev_date = date;
    while prev_date.weekday() != weekday {
        prev_date -= Duration::days(1);
    }
    prev_date
}
