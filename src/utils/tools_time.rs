use chrono::{Datelike, Local, NaiveDate, ParseError};

pub const TIME_FORMAT_1: &str = "%Y%m";
pub const TIME_FORMAT_6: &str = "%Y%m%d";
pub const TIME_FORMAT_2: &str = "%Y-%m-%d";
pub const TIME_FORMAT_3: &str = "%Y%m%d%H%M%S";
pub const TIME_FORMAT_4: &str = "%Y-%m-%d %H:%M:%S";
pub const TIME_FORMAT_5: &str = "%Y-%m-%dT%H:%M:%S";

pub struct TimeTools;

impl TimeTools {
    pub fn gen_current_timestamp() -> String {
        Local::now().timestamp_millis().to_string()
    }

    pub fn gen_time(format: &str) -> String {
        Local::now().format(format).to_string()
    }

    pub fn get_age_by_birthday(birthday: &str) -> Result<i32, ParseError> {
        let birth_date = NaiveDate::parse_from_str(birthday, TIME_FORMAT_2)?;
        let today = Local::now().date_naive();
        let mut age = today.year() - birth_date.year();
        let current_month_day = (today.month(), today.day());
        let birth_month_day = (birth_date.month(), birth_date.day());
        if current_month_day < birth_month_day {
            age -= 1;
        }
        Ok(age)
    }

    pub fn calculate_days(start_time: &str, end_time: &str) -> Result<i64, ParseError> {
        let start_date = NaiveDate::parse_from_str(start_time, TIME_FORMAT_4)?;
        let end_date = NaiveDate::parse_from_str(end_time, TIME_FORMAT_4)?;
        Ok((end_date - start_date).num_days() + 1)
    }
}
