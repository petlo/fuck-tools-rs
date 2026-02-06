pub mod card;
pub mod certificate;

/// 证件类型枚举
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CertificateType {
    /// 身份证（18位）
    #[default]
    IdCard18,
    /// 身份证（15位）
    IdCard15,
    /// 护照
    Passport,
    /// 统一社会信用代码
    CreditCode,
    /// 其他/未知
    Other,
}
