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

            let res = calc_years_months_days_since_date(dob);

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

    fn calc_date_diff(
        current_date: NaiveDate,
        prev_date: NaiveDate,
    ) -> Result<(i32, i32, i32), String> {
        let mut days = 0;
        let mut months = 0;
        let mut years = 0;
        return Result::Ok((years as i32, months as i32, days as i32));
    }

    fn calc_years_months_days_since_date(prev_date: NaiveDate) -> Result<(i32, i32, i32), String> {
        // Today's date
        let today = chrono::Local::now().naive_local();

        // Calculate the difference
        let today_naive_date =
            NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

        let duration = today_naive_date - prev_date;

        if duration.num_days() < 0 {
            return Result::Err(String::from("Invalid dates!"));
        } else {
            // Prepare data;
            let d0 = prev_date.day();
            let m0 = prev_date.month();
            let y0 = prev_date.year();

            let mut d1 = today_naive_date.day();
            let mut m1 = today_naive_date.month();
            let mut y1 = today_naive_date.year();

            let mut days = 0;
            let mut months = 0;
            let mut years = 0;

            // Subtract days.
            if d0 > d1 {
                d1 += num_days_in_month(m1, y1);
                m1 -= 1;
                days = d1 - d0;
            } else {
                days = d1 - d0;
            }

            // Subtract months.
            if m0 > m1 {
                m1 += 12;
                y1 -= 1;
                months = m1 - m0;
            } else {
                months = m1 - m0;
            }
            years = y1 - y0;

            return Result::Ok((years as i32, months as i32, days as i32));
        }
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

    pub fn date_demo() {
        let mut p = Person {
            dob_day: 7,
            dob_month: 2,
            dob_year: 1970,
            name: String::from("Diaa ElKott"),
            age_years: 1,
            age_months: 1,
            age_days: 1,
        };

        p.update_age();
        println!("\n\nPERSON:\t{:?}", p);
    }
}
