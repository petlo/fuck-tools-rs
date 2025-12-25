use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, ParseError};

pub struct TimeTools;

impl TimeTools {
    pub fn get_now_timestamp() -> String {
        Local::now().timestamp_millis().to_string()
    }

    /// 获取时间字符串: 格式 202307
    pub fn generate_time_1() -> String {
        Local::now().format("%Y%m").to_string()
    }

    /// 获取时间字符串: 格式 20230724153045
    pub fn generate_time_2() -> String {
        Local::now().format("%Y%m%d%H%M%S").to_string()
    }

    /// 获取时间字符串: 格式 2025-01-01 00:00:00
    pub fn generate_time_3() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// 获取时间字符串: 格式 2025-01-01T00:00:00
    pub fn generate_time_4() -> String {
        Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    pub fn parse_t_time(time_str: &str) -> String {
        NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S")
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string())
            .unwrap_or_else(|_| time_str.replace(' ', "T"))
    }

    /// 获取时间字符串: 格式 2025-01-01
    pub fn generate_time_5() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }

    pub fn get_age_by_birthday(birthday: &str) -> String {
        let birth_date = match NaiveDate::parse_from_str(birthday, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return "".to_string(),
        };
        let today = Local::now().date_naive();
        let mut age = today.year() - birth_date.year();
        let has_birthday_passed =
            (today.month(), today.day()) >= (birth_date.month(), birth_date.day());
        if !has_birthday_passed {
            age -= 1;
        }
        age.to_string()
    }

    pub fn get_days(start_time: &str, end_time: &str) -> Result<i64, ParseError> {
        let start_date = NaiveDate::parse_from_str(start_time, "%Y-%m-%d %H:%M:%S")?;
        let end_date = NaiveDate::parse_from_str(end_time, "%Y-%m-%d %H:%M:%S")?;
        Ok((end_date - start_date).num_days() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_local_timestamp() {
        println!("{}", TimeTools::get_now_timestamp());
    }

    #[test]
    fn test_generate_time_1() {
        println!("{}", TimeTools::generate_time_1());
    }

    #[test]
    fn test_generate_time_2() {
        println!("{}", TimeTools::generate_time_2());
    }

    #[test]
    fn test_generate_time_3() {
        println!("{}", TimeTools::generate_time_3());
    }

    #[test]
    fn test_get_days() {
        let time1 = "2025-11-04 00:00:00";
        let time2 = "2025-11-08 00:00:00";
        println!("{:?}", TimeTools::get_days(time1, time2));
    }

    #[test]
    fn test_parse_time() {
        let time1 = "2025-11-04 00:00:00";
        println!("{}", TimeTools::parse_t_time(time1));
    }
}
