pub use chrono::{Datelike, NaiveDate, Weekday};
pub use Weekday  as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let days_in_year = if NaiveDate::from_ymd_opt(year, 12, 31).unwrap().leap_year() {
        366
    } else {
        365
    };

    if days_in_year % 2 == 0 {
        return None;
    }

    let middle_day =
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap() + chrono::Duration::days(days_in_year / 2);
    Some(middle_day.weekday())
}

// later implementation;
// use the method .ordinal()
// this method will directly return the number of days given a date object an


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_day() {
        assert_eq!(wd::Tue, middle_day(2019).unwrap());
        assert_eq!(wd::Wed, middle_day(1997).unwrap());
        assert_eq!(wd::Mon, middle_day(1663).unwrap());
        assert_eq!(wd::Wed, middle_day(1873).unwrap());
        assert_eq!(wd::Thu, middle_day(1953).unwrap());
        assert_eq!(wd::Wed, middle_day(1879).unwrap());
    }
}
