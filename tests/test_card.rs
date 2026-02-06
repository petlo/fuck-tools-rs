#[cfg(test)]
mod tests {
    use fuck_tools_rs::card_util::card::CardUtil;
    use fuck_tools_rs::card_util::certificate::CertificateUtil;

    #[test]
    fn test_valid_18_card() {
        // 测试有效的18位身份证（示例号码，非真实）
        assert!(CardUtil::is_valid_card("612124197504112975"));
        assert!(CardUtil::is_valid_card("43313020001020633X"));
    }

    #[test]
    fn test_valid_15_card() {
        // 测试有效的15位身份证（示例号码，非真实）
        assert!(CardUtil::is_valid_card("110101900307387"));
    }

    #[test]
    fn test_invalid_card() {
        // 测试无效身份证
        assert!(!CardUtil::is_valid_card(""));
        assert!(!CardUtil::is_valid_card("123"));
        assert!(!CardUtil::is_valid_card("11010119900307387")); // 17位
        assert!(!CardUtil::is_valid_card("1101011990030738761")); // 19位
        assert!(!CardUtil::is_valid_card("11010119900307387Y")); // 非法字符
        assert!(!CardUtil::is_valid_card("110101199013073876")); // 无效月份13
    }

    #[test]
    fn test_get_birthday() {
        assert_eq!(
            CardUtil::get_birthday("612124197504112975"),
            Some("1975-04-11".to_string())
        );
        assert_eq!(
            CardUtil::get_birthday("43313020001020633X"),
            Some("2000-10-20".to_string())
        );
    }

    #[test]
    fn test_get_gender() {
        assert_eq!(
            CardUtil::get_gender("350581200008153514"),
            Some("M".to_string())
        );
        assert_eq!(
            CardUtil::get_gender("430524198909218189"),
            Some("F".to_string())
        );
    }

    #[test]
    fn test_mask_card() {
        assert_eq!(CardUtil::mask_card("110101199003073876"), "110101****3876");
        assert_eq!(CardUtil::mask_card("110101900307387"), "110101****7387");
    }

    #[test]
    fn test_certificate() {
        let cert_type = CertificateUtil::identify_certificate("91500113MAEWU1HL8D");
        println!("{:?}", cert_type);
    }
}
