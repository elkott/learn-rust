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
/// let num_days   = date_calc::days_between(start_date, end_date);
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
pub fn days_between(end_date: NaiveDate, start_date: NaiveDate) -> i64 {
    (end_date - start_date).num_days()
}

/// Calculate the difference between two dates in years, months, and days.
///
/// # Description
///
/// Perform date subtraction on two dates. An example to demonstrate the
/// methodology used is below.
///
/// FROM:   1950-02-28
///
/// TO:     2023-04-02
///
/// ------------------
/// Years:  73
///
/// Months: 1
///
/// Days:   5
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Datelike};
///
/// let y0: i32 = 1950;
/// let m0: u32 = 2;
/// let d0: u32 = 28;
///
/// let y1: i32 = 2023;
/// let m1: u32 = 4;
/// let d1: u32 = 2;
///
/// let date_0 = NaiveDate::from_ymd_opt(y0, m0, d0);
/// let date_1 = NaiveDate::from_ymd_opt(y1, m1, d1);
///
/// let res = date_calc::date_diff(date_1.unwrap(), date_0.unwrap(), false);
///
/// let mut num_years = 0;
/// let mut num_months = 0;
/// let mut num_days = 0;
///
/// match res {
///     Ok((years, months, days)) => {
///         num_years = years as u32;
///         num_months = months as u32;
///         num_days = days as u32;
///     }
///     Err(error) => {
///         println!("Error: {}", error);
///     }
/// }
/// assert_eq!(73, num_years);
/// assert_eq!(1,  num_months);
/// assert_eq!(5,  num_days);
/// ```
///
/// # Input
///
/// * satrt_date:NaiveDate
/// * end_date:  NaiveDate
/// * include_last_day: boolean
///
/// # Output
///
/// * Result<(u32, u32, u32), String> enum containing the years, months and
///   days between the two dates, or
/// * error message in case the input is inversed.
pub fn date_diff(
    end_date: NaiveDate,
    start_date: NaiveDate,
    include_last_day: bool,
) -> Result<(u32, u32, u32), String> {
    // Calculate date difference in days.
    let diff_days = days_between(end_date, start_date);

    // Check error case.
    if diff_days < 0 {
        return Result::Err(String::from(
            "Invalid dates order. Did you switch start, and end dates?",
        ));
    } else {
        // Prepare data;
        let d0 = start_date.day();
        let m0 = start_date.month();
        let y0 = start_date.year();

        let mut d1 = end_date.day();
        let mut m1 = end_date.month();
        let mut y1 = end_date.year();

        // Adjust end date days.
        if d1 < d0 {
            if m1 > 1 {
                d1 += num_days_in_month(m1 - 1, y1).unwrap();
            } else {
                d1 += num_days_in_month(m1, y1).unwrap();
            }
            m1 -= 1;
        }

        // Adjust for including last day.
        if include_last_day {
            d1 += 1;
        }

        // Adjust end date years, and months.
        if m0 > m1 {
            m1 += 12;
            y1 -= 1;
        }

        // Subtract dates.
        let days = d1 - d0;
        let months = m1 - m0;
        let years = y1 - y0;

        return Result::Ok((years as u32, months, days));
    }
}

/// Calculate the difference between a past date and today's date
/// in years, months, and days.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Datelike};
///
/// let y0: i32 = 1950;
/// let m0: u32 = 2;
/// let d0: u32 = 28;
///
/// let start_date = NaiveDate::from_ymd_opt(y0, m0, d0);
/// let res = date_calc::years_months_days_since(start_date.unwrap(), false);
///
/// let mut num_years = 0;
/// let mut num_months = 0;
/// let mut num_days = 0;
///
/// match res {
///     Ok((years, months, days)) => {
///         num_years = years as u32;
///         num_months = months as u32;
///         num_days = days as u32;
///         
///     println!(
///         "Time since {:?} is: {} Years, {} Months, {} Days.",
///         start_date, num_years, num_months, num_days
///     );
///     }
///     Err(error) => {
///         println!("Error: {}", error);
///     }
/// }
/// ```
///
/// # Input
///
/// * satrt_date:NaiveDate
/// * include_today: boolean
///
/// # Output
///
/// * Result<(u32, u32, u32), String> enum containing the years, months and
///   days between today's date and start_date, or
/// * error message in case start_date is in the future.
pub fn years_months_days_since(
    start_date: NaiveDate,
    include_today: bool,
) -> Result<(u32, u32, u32), String> {
    // Today's date
    let today = chrono::Local::now().naive_local();
    let today_naive_date =
        NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    date_diff(today_naive_date, start_date, include_today)
}

/// Calculate the difference between a future date and today's date
/// in years, months, and days.
///
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Datelike};
///
/// let y0: i32 = 2099;
/// let m0: u32 = 2;
/// let d0: u32 = 28;
///
/// let end_date = NaiveDate::from_ymd_opt(y0, m0, d0);
/// let res = date_calc::years_months_days_till(end_date.unwrap(), false);
///
/// let mut num_years = 0;
/// let mut num_months = 0;
/// let mut num_days = 0;
///
/// match res {
///     Ok((years, months, days)) => {
///         num_years = years as u32;
///         num_months = months as u32;
///         num_days = days as u32;
///
///     println!(
///         "Time till {:?} is: {} Years, {} Months, {} Days.",
///         end_date, num_years, num_months, num_days
///     );         
///     }
///     Err(error) => {
///         println!("Error: {}", error);
///     }
/// }
/// ```
///
/// # Input
///
/// * satrt_date:NaiveDate
/// * include_today: boolean
///
/// # Output
///
/// * Result<(u32, u32, u32), String> enum containing the years, months and
///   days between today's date and start_date, or
/// * error message in case start_date is in the future.
pub fn years_months_days_till(
    end_date: NaiveDate,
    include_last_day: bool,
) -> Result<(u32, u32, u32), String> {
    // Today's date
    let today = chrono::Local::now().naive_local();
    let today_naive_date =
        NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    date_diff(end_date, today_naive_date, include_last_day)
}

#[cfg(test)]
mod tests_date_diff {
    use super::*;

    #[test]
    fn test_date_diff_days() {
        assert_eq!(
            0,
            days_between(
                NaiveDate::from_ymd_opt(2023, 4, 4).unwrap(),
                NaiveDate::from_ymd_opt(2023, 4, 4).unwrap()
            )
        );

        assert_eq!(
            365,
            days_between(
                NaiveDate::from_ymd_opt(1971, 2, 7).unwrap(),
                NaiveDate::from_ymd_opt(1970, 2, 7).unwrap()
            )
        );
    }
}
