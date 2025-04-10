use chrono::{NaiveDate, Datelike, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let days_in_year = if NaiveDate::from_ymd_opt(year, 12, 31).unwrap().leap_year() {
        366
    } else {
        365
    };

    if days_in_year % 2 == 0 {
        return None;
    }

    let middle_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap() + chrono::Duration::days(days_in_year / 2);
    Some(middle_day.weekday())
}

// later implementation;
// use the method .ordinal()
// this method will directly return the number of days given a date object
