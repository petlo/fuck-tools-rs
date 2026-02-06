use crate::card_util::CertificateType;
use lazy_static::lazy_static;
use regex::Regex;

pub struct CertificateUtil;

impl CertificateUtil {
    /// 识别证件类型
    pub fn identify_certificate(certificate: &str) -> CertificateType {
        let cert = certificate.trim();

        if Self::is_id_card(cert) {
            if cert.len() == 18 {
                CertificateType::IdCard18
            } else {
                CertificateType::IdCard15
            }
        } else if Self::is_passport(cert) {
            CertificateType::Passport
        } else if Self::is_credit_code(cert) {
            CertificateType::CreditCode
        } else {
            CertificateType::Other
        }
    }

    /// 判断是否是身份证
    fn is_id_card(cert: &str) -> bool {
        // 检查长度
        if cert.len() != 15 && cert.len() != 18 {
            return false;
        }

        // 基本格式检查
        lazy_static! {
            static ref ID_CARD_18: Regex = Regex::new(r"^\d{17}[\dXx]$").unwrap();
            static ref ID_CARD_15: Regex = Regex::new(r"^\d{15}$").unwrap();
        }

        if cert.len() == 18 {
            if !ID_CARD_18.is_match(cert) {
                return false;
            }
            Self::verify_id_card_18(cert);
            true
        } else {
            ID_CARD_15.is_match(cert)
        }
    }

    /// 验证18位身份证校验码
    fn verify_id_card_18(cert: &str) -> bool {
        let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
        let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

        let mut sum = 0;
        // 使用 enumerate() 遍历 chars 并同时获取索引
        for (i, ch) in cert.chars().take(17).enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                sum += digit * weights[i];
            } else {
                return false;
            }
        }

        let mod_value = (sum % 11) as usize;
        let check_code = cert.chars().nth(17).unwrap().to_ascii_uppercase();

        check_codes[mod_value] == check_code
    }

    /// 判断是否是护照
    fn is_passport(cert: &str) -> bool {
        // 中国护照格式：
        // 1. 普通护照：E/G/D/S/P开头 + 8位数字
        // 2. 公务护照：DE/SE/PE开头 + 7位数字
        // 3. 外交护照：DE/SE/PE开头 + 7位数字

        lazy_static! {
            // 普通护照：E/G/D/S/P + 8位数字
            static ref PASSPORT_NORMAL: Regex = Regex::new(r"^[EGDSP]\d{8}$").unwrap();
            // 公务/外交护照：DE/SE/PE + 7位数字
            static ref PASSPORT_OFFICIAL: Regex = Regex::new(r"^(DE|SE|PE)\d{7}$").unwrap();
            // 香港护照：K/H + 8位数字
            static ref PASSPORT_HK: Regex = Regex::new(r"^[KH]\d{8}$").unwrap();
            // 澳门护照：MA/M + 7位数字
            static ref PASSPORT_MACAU: Regex = Regex::new(r"^(MA|M)\d{7}$").unwrap();
            // 台湾护照：TB/P + 7位数字
            static ref PASSPORT_TAIWAN: Regex = Regex::new(r"^(TB|P)\d{7}$").unwrap();
            // 新版电子护照：E + 8位数字
            static ref PASSPORT_E: Regex = Regex::new(r"^E\d{8}$").unwrap();

            // 外国护照通常以字母开头，包含字母和数字，长度5-12位
            static ref PASSPORT_FOREIGN: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9]{4,11}$").unwrap();
        }

        // 检查各种护照格式
        PASSPORT_NORMAL.is_match(cert)
            || PASSPORT_OFFICIAL.is_match(cert)
            || PASSPORT_HK.is_match(cert)
            || PASSPORT_MACAU.is_match(cert)
            || PASSPORT_TAIWAN.is_match(cert)
            || PASSPORT_E.is_match(cert)
            || (cert.len() >= 5 && cert.len() <= 12 && PASSPORT_FOREIGN.is_match(cert))
    }

    /// 判断是否是统一社会信用代码
    fn is_credit_code(cert: &str) -> bool {
        if cert.len() != 18 {
            return false;
        }

        // 基本格式：18位，包含数字和大写字母（排除I,O,Z,V,S）
        lazy_static! {
            static ref CREDIT_CODE_PATTERN: Regex = Regex::new(
                r"^[0-9A-HJ-NP-RT-UW-Y]{2}[0-9]{6}[0-9A-HJ-NP-RT-UW-Y]{9}[0-9A-HJ-NP-RT-UW-Y]$"
            )
            .unwrap();
        }

        if !CREDIT_CODE_PATTERN.is_match(cert) {
            return false;
        }
        true
    }
}
