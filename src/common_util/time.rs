use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, ParseError, TimeZone, Utc};

pub const TIME_FORMAT_1: &str = "%Y%m";
pub const TIME_FORMAT_6: &str = "%Y%m%d";
pub const TIME_FORMAT_2: &str = "%Y-%m-%d";
pub const TIME_FORMAT_3: &str = "%Y%m%d%H%M%S";
pub const TIME_FORMAT_4: &str = "%Y-%m-%d %H:%M:%S";
pub const TIME_FORMAT_5: &str = "%Y-%m-%dT%H:%M:%S";

pub struct TimeUtil;

impl TimeUtil {
    pub fn gen_current_timestamp() -> String {
        Local::now().timestamp_millis().to_string()
    }

    pub fn gen_time(format: &str) -> String {
        Local::now().format(format).to_string()
    }

    /// 将 NaiveDateTime 转换为指定格式的字符串
    ///
    /// # 参数
    /// - `naive_date_time`: 无时区的日期时间对象
    /// - `format`: 格式化字符串，遵循 chrono 的格式语法
    ///
    /// # 返回值
    /// 格式化后的字符串
    /// ```
    pub fn naive_time_format(naive_date_time: NaiveDateTime, format: &str) -> String {
        naive_date_time.format(format).to_string()
    }

    /// 判断给定的时间字符串是否已过期UTC时间
    ///
    /// # 参数
    /// - `time_string`: UTC时间字符串，格式："2026-01-04 12:29:29"
    ///
    /// # 返回值
    /// - `true`: 时间已过期
    /// - `false`: 时间未过期或解析失败
    ///
    /// # 注意
    /// 此版本假设输入的时间字符串已经是UTC时间，
    /// 适用于简单的、不涉及时区转换的场景。
    pub fn is_expired_utc(time_string: &str) -> bool {
        // 尝试解析为NaiveDateTime（无时区）
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(time_string, TIME_FORMAT_4) {
            // 转换为UTC DateTime（假设输入就是UTC时间）
            let utc_time = Utc.from_utc_datetime(&naive_dt);
            let now = Utc::now();
            utc_time < now
        } else {
            // 如果解析失败，默认返回true（视为已过期），避免安全问题
            true
        }
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
