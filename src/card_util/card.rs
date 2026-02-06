use lazy_static::lazy_static;
use regex::Regex;

pub struct CardUtil;

impl CardUtil {
    /// 判断是否是有效证件号码
    pub fn is_valid_card(card_no: &str) -> bool {
        if card_no.is_empty() {
            return false;
        }

        let card_no = card_no.trim();
        let len = card_no.len();

        match len {
            15 => Self::is_valid_15_card(card_no),
            18 => Self::is_valid_18_card(card_no),
            _ => false,
        }
    }

    /// 从身份证获取生日
    pub fn get_birthday(card_no: &str) -> Option<String> {
        if !Self::is_valid_card(card_no) {
            return None;
        }

        let len = card_no.len();
        match len {
            15 => {
                let year = &card_no[6..8];
                let month = &card_no[8..10];
                let day = &card_no[10..12];
                let prefix = if &year[0..1] > "2" { "19" } else { "20" };
                Some(format!("{}{}-{}-{}", prefix, year, month, day))
            }
            18 => {
                let year = &card_no[6..10];
                let month = &card_no[10..12];
                let day = &card_no[12..14];
                Some(format!("{}-{}-{}", year, month, day))
            }
            _ => None,
        }
    }

    /// 从身份证获取性别
    pub fn get_gender(card_no: &str) -> Option<String> {
        if !Self::is_valid_card(card_no) {
            return None;
        }

        let len = card_no.len();

        // 获取性别位
        let gender_digit_char = match len {
            15 => card_no.chars().nth(14)?, // 15位身份证：第15位（索引14）
            18 => card_no.chars().nth(16)?, // 18位身份证：第17位（索引16）
            _ => return None,
        };

        // 转换为数字
        let gender_digit = gender_digit_char.to_digit(10)?;

        // 判断性别：奇数为男，偶数为女
        Some(if gender_digit % 2 == 1 {
            "M".to_string()
        } else {
            "F".to_string()
        })
    }

    /// 从身份证获取年龄
    pub fn get_age(card_no: &str) -> Option<i32> {
        let birthday_str = Self::get_birthday(card_no)?;

        // 解析生日
        let parts: Vec<&str> = birthday_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;

        // 获取当前日期
        use chrono::prelude::*;
        let now = Local::now();
        let current_year = now.year();
        let current_month = now.month();
        let current_day = now.day();

        // 计算年龄
        let mut age = current_year - year;

        // 如果今年生日还没过，年龄减1
        if current_month < month || (current_month == month && current_day < day) {
            age -= 1;
        }

        Some(age)
    }

    /// 掩码处理身份证（显示前6位和后4位）
    pub fn mask_card(card_no: &str) -> String {
        if card_no.len() < 10 {
            return "*".repeat(card_no.len());
        }

        let prefix = &card_no[0..6];
        let suffix = &card_no[card_no.len() - 4..];

        format!("{}****{}", prefix, suffix)
    }

    /// 验证15位身份证
    fn is_valid_15_card(card_no: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{15}$").unwrap();
        }

        if !RE.is_match(card_no) {
            return false;
        }

        // 验证生日
        Self::is_valid_birthday_15(card_no)
    }

    /// 验证18位身份证
    fn is_valid_18_card(card_no: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{17}[\dXx]$").unwrap();
        }

        if !RE.is_match(card_no) {
            return false;
        }

        // 验证生日
        if !Self::is_valid_birthday_18(card_no) {
            return false;
        }

        // 验证校验码
        Self::verify_check_code(card_no)
    }

    /// 验证15位身份证生日
    fn is_valid_birthday_15(card_no: &str) -> bool {
        let year = &card_no[6..8];
        let month = &card_no[8..10];
        let day = &card_no[10..12];

        Self::is_valid_date(year, month, day, true)
    }

    /// 验证18位身份证生日
    fn is_valid_birthday_18(card_no: &str) -> bool {
        let year = &card_no[6..10];
        let month = &card_no[10..12];
        let day = &card_no[12..14];

        Self::is_valid_date(year, month, day, false)
    }

    /// 验证日期是否有效
    fn is_valid_date(year_str: &str, month_str: &str, day_str: &str, is_15_format: bool) -> bool {
        let year: i32 = if is_15_format {
            // 15位身份证：年份为2位，需要补全
            let prefix = if &year_str[0..1] > "2" { "19" } else { "20" };
            format!("{}{}", prefix, year_str).parse().unwrap_or(0)
        } else {
            year_str.parse().unwrap_or(0)
        };

        let month: u32 = month_str.parse().unwrap_or(0);
        let day: u32 = day_str.parse().unwrap_or(0);

        // 基本验证
        if !(1900..=2100).contains(&year) || month == 0 || month > 12 || day == 0 || day > 31 {
            return false;
        }

        // 月份天数验证
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                // 闰年判断
                let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                if is_leap_year { 29 } else { 28 }
            }
            _ => return false,
        };

        day <= days_in_month
    }

    /// 验证18位身份证校验码
    fn verify_check_code(card_no: &str) -> bool {
        // 权重系数
        let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
        // 校验码对应值
        let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

        let mut sum = 0;
        for (i, ch) in card_no.chars().take(17).enumerate() {
            let digit = ch.to_digit(10).unwrap_or_default();
            sum += digit * weights[i];
        }

        let mod_value = sum % 11;
        let check_code = card_no.chars().nth(17).unwrap_or_default().to_ascii_uppercase();

        check_codes[mod_value as usize] == check_code
    }
}
