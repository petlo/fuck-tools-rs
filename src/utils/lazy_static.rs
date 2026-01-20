use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // 用户名正则：字母数字下划线，3-20位
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();

    // 手机号正则（简化的国际格式）
    pub static ref MOBILE_REGEX: Regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();

    // 邮箱正则
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
}
