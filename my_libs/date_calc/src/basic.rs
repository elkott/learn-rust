/// Test whether a year is leap year or not.
///
/// # Description
///
/// A leap year is a year that is evenly divisible by 4, except for
/// years that are divisible by 100 but NOT divisible by 400.
/// For example, 2000 and 2400 are leap years, but 1800 and 1900 are not.
///
/// # Example
///
/// ```
/// let is_leap = date_calc::is_leap_year(2000); // returns true.
/// let is_leap = date_calc::is_leap_year(1900); // returns false.
/// ```
///
/// # Input
///
/// * year:i32
///
/// # Output
///
/// * true/false.
pub fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    } else {
        return false;
    }
}

/// Find the number of days in a given month.
///
/// # Description
///
/// The year is passed as an input argument in order to
/// return the number of days in February.
///
/// # Example
///
/// ```
/// let month: u32 = 2;
/// let year: i32 = 2000;
/// let res = date_calc::num_days_in_month(month, year);
/// match res {
///     Ok(days) => {
///         println!("\nNumber of days in {}-{} is {}", month, year, days);
///     }
///     Err(error) => {
///         println!("Error: {}", error);
///     }
/// }
/// ```
/// The above code snippet will print the string:
///
/// Number of days in 2-2000 is 29
///
/// # Input
///
/// * month:u32
/// * year: i32
///
/// # Return
///
/// Result<u32, String> enum containing the number of days in month, or
/// error message in case the month input is wrong.
pub fn num_days_in_month(month: u32, year: i32) -> Result<u32, String> {
    if month < 1 || month > 12 {
        return Result::Err(String::from("Invalid month value!"));
    }

    if month == 2 {
        if is_leap_year(year) {
            return Result::Ok(29);
        } else {
            return Result::Ok(28);
        }
    } else {
        if month == 4 || month == 6 || month == 9 || month == 11 {
            return Result::Ok(30);
        } else {
            return Result::Ok(31);
        }
    }
}

#[cfg(test)]
mod tests_basic {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert_eq!(false, is_leap_year(1900));
        assert_eq!(false, is_leap_year(1970));
        assert_eq!(true, is_leap_year(2000));
        assert_eq!(false, is_leap_year(2019));
        assert_eq!(true, is_leap_year(2020));
        assert_eq!(false, is_leap_year(2021));
        assert_eq!(true, is_leap_year(2024));
    }

    #[test]
    fn test_num_days_in_month() {
        // February cases.
        assert_eq!(
            Result::Err(String::from("Invalid month value!")),
            num_days_in_month(0, 2019)
        );

        assert_eq!(
            Result::Err(String::from("Invalid month value!")),
            num_days_in_month(15, 2019)
        );

        assert_eq!(28, num_days_in_month(2, 2019).unwrap());
        assert_eq!(29, num_days_in_month(2, 2020).unwrap());

        // 30-days months.
        assert_eq!(30, num_days_in_month(4, 2019).unwrap());
        assert_eq!(30, num_days_in_month(6, 2019).unwrap());
        assert_eq!(30, num_days_in_month(9, 2019).unwrap());
        assert_eq!(30, num_days_in_month(11, 2019).unwrap());

        // 31-days months.
        assert_eq!(31, num_days_in_month(1, 2019).unwrap());
        assert_eq!(31, num_days_in_month(3, 2019).unwrap());
        assert_eq!(31, num_days_in_month(5, 2019).unwrap());
        assert_eq!(31, num_days_in_month(7, 2019).unwrap());
        assert_eq!(31, num_days_in_month(8, 2019).unwrap());
        assert_eq!(31, num_days_in_month(10, 2019).unwrap());
        assert_eq!(31, num_days_in_month(12, 2019).unwrap());
    }
}
