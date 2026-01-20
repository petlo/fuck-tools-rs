use snowflake_rs::{STANDARD_EPOCH, SnowFlakeId};
use uuid::Uuid;

pub struct IdTools;

impl IdTools {
    /// 生成雪花ID
    pub fn gen_snowflake_id() -> Result<u64, String> {
        SnowFlakeId::new(1, STANDARD_EPOCH).generate_id()
    }

    /// 生成 UUID V4
    pub fn gen_uuid_v4() -> Uuid {
        Uuid::new_v4()
    }

    /// 生成简化版本 UUID V4
    pub fn gen_uuid_v4_simple() -> String {
        Uuid::new_v4().simple().to_string()
    }

    /// 生成 UUID V7
    pub fn gen_uuid_v7() -> Uuid {
        Uuid::now_v7()
    }

    /// 生成 UUID V7 的简化版本
    pub fn gen_uuid_v7_simple() -> String {
        Uuid::now_v7().simple().to_string()
    }

    /// 生成带有前缀的UUID V4
    pub fn gen_uuid_v4_with_prefix(prefix: &str) -> String {
        format!("{}{}", prefix, Self::gen_uuid_v4())
    }

    /// 生成带有前缀的UUID V7
    pub fn gen_uuid_v7_with_prefix(prefix: &str) -> String {
        format!("{}{}", prefix, Self::gen_uuid_v7())
    }

    /// 生成多个UUID V4
    pub fn gen_multiple_uuid_v4(count: usize) -> Vec<Uuid> {
        (0..count).map(|_| Self::gen_uuid_v4()).collect()
    }

    /// 验证UUID格式
    pub fn is_valid_uuid(uuid_str: &str) -> bool {
        Uuid::parse_str(uuid_str).is_ok()
    }

    /// 解析UUID字符串
    pub fn parse_uuid(uuid_str: &str) -> Option<Uuid> {
        Uuid::parse_str(uuid_str).ok()
    }
}
