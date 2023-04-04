mod mod99 {
    use chrono::{Datelike, Duration, NaiveDate};

    #[derive(Debug)]
    pub struct Person {
        name: String,
        dob_day: u32,
        dob_month: u32,
        dob_year: i32,
        age_years: u32,
        age_months: u32,
        age_days: u32,
    }

    impl Person {
        pub fn update_age(&mut self) {
            let dob = NaiveDate::from_ymd_opt(self.dob_year, self.dob_month, self.dob_day).unwrap();

            let today = chrono::Local::now().naive_local();

            let today_naive_date =
                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

            let full_age = today_naive_date - dob;

            // let (years, months, rem_days) = full_age.num_days() % 365;

            let res = calc_years_months_days_since_date(dob, false);

            match res {
                Ok((years, months, days)) => {
                    self.age_years = years as u32;
                    self.age_months = months as u32;
                    self.age_days = days as u32;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
    }

    fn calc_num_days_between(end_date: NaiveDate, start_date: NaiveDate) -> i64 {
        (end_date - start_date).num_days()
    }

    fn calc_date_diff(
        end_date: NaiveDate,
        start_date: NaiveDate,
        include_last_day: bool,
    ) -> Result<(u32, u32, u32), String> {
        // Difference in days.
        let diff_days = calc_num_days_between(end_date, start_date);

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
                    d1 += num_days_in_month(m1 - 1, y1);
                } else {
                    d1 += num_days_in_month(m1, y1);
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

    fn calc_years_months_days_since_date(
        prev_date: NaiveDate,
        include_today: bool,
    ) -> Result<(u32, u32, u32), String> {
        // Today's date
        let today = chrono::Local::now().naive_local();
        let today_naive_date =
            NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

        calc_date_diff(today_naive_date, prev_date, include_today)
    }

    // Check whether year is leap year.
    fn is_leap_year(year: i32) -> bool {
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

    // Find the number of days in a month.
    fn num_days_in_month(month: u32, year: i32) -> u32 {
        if month == 2 {
            if is_leap_year(year) {
                return 29;
            } else {
                return 28;
            }
        } else {
            if month == 4 || month == 6 || month == 9 || month == 11 {
                return 30;
            } else {
                return 31;
            }
        }
    }

    pub fn date_demo_0() {
        let y0: i32 = 1970;
        let m0: u32 = 2;
        let d0: u32 = 7;

        let y1: i32 = 2023;
        let m1: u32 = 4;
        let d1: u32 = 2;

        let date_0 = NaiveDate::from_ymd_opt(y0, m0, d0);
        let date_1 = NaiveDate::from_ymd_opt(y1, m1, d1);

        let res = calc_date_diff(date_1.unwrap(), date_0.unwrap(), false);

        let mut num_years = 0;
        let mut num_months = 0;
        let mut num_days = 0;

        match res {
            Ok((years, months, days)) => {
                num_years = years as u32;
                num_months = months as u32;
                num_days = days as u32;
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }

        println!("\n=============================");
        println!("D A T E   D I F F E R E N C E");
        println!("=============================");
        println!("FROM:\t{:?}", date_0);
        println!("TO:\t{:?}", date_1);
        println!("_____________________________");
        println!("Years:\t{}", num_years);
        println!("Months:\t{}", num_months);
        println!("Days:\t{}", num_days);

        println!("\nDIFF IN DAYS = \t{}", date_calc::days_between_dates(date_0.unwrap(), date_1.unwrap()));
    }
}
