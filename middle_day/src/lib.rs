use chrono::{NaiveDate, Datelike, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    if year < 1 || year > 9999 {
        return None;
    }
    let leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let days_in_year = if leap_year { 366 } else { 365 };

    if days_in_year % 2 == 0 {
        return None;
    }

    let middle_day = NaiveDate::from_ymd(year, 7, 2);
    Some(middle_day.weekday())
}

// later implementation;
// use the method .ordinal()
// this method will directly return the number of days given a date object an