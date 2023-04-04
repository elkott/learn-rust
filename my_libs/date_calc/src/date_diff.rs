include!("basic.rs");

use chrono::{Datelike, Duration, NaiveDate};

/// Calculate the difference between two dates in days.
///
/// # Description
///
/// Return the number of days between two dates. A negative return value
/// means that the input start date is beyond the end date.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate};
///
/// let start_date = NaiveDate::from_ymd_opt(2023, 4, 4).unwrap();
/// let end_date   = NaiveDate::from_ymd_opt(2023, 3, 4).unwrap();
/// let num_days   = date_calc::days_between_dates(start_date, end_date);
/// 
/// assert_eq!(31, num_days);
/// ```
///
/// # Input
///
/// * satrt_date:NaiveDate
/// * end_date:  NaiveDate
///
/// # Output
///
/// * num_days
pub fn days_between_dates(end_date: NaiveDate, start_date: NaiveDate) -> i64 {
    (end_date - start_date).num_days()
}

#[cfg(test)]
mod tests_date_diff {
    use super::*;

    #[test]
    fn test_date_diff_days() {
        assert_eq!(
            0,
            days_between_dates(
                NaiveDate::from_ymd_opt(2023, 4, 4).unwrap(),
                NaiveDate::from_ymd_opt(2023, 4, 4).unwrap()
            )
        );

        assert_eq!(
            365,
            days_between_dates(
                NaiveDate::from_ymd_opt(1971, 2, 7).unwrap(),
                NaiveDate::from_ymd_opt(1970, 2, 7).unwrap()
            )
        );
    }
}
