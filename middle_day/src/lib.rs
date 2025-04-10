use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    if (is_leap_year && 366 % 2 == 0) || (!is_leap_year && 365 % 2 == 0) {
        return None;
    } else {
        let middle_day_number = if is_leap_year { 183 } else { 183 };
        let middle_day = NaiveDate::from_ymd(year, 7, 2);
        return Some(middle_day.weekday());
    }
}
