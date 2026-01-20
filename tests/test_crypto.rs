#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;
    use fuck_tools_rs::utils::tools_crypto::CryptoTools;
    use fuck_tools_rs::utils::tools_sm::SmTools;

    #[test]
    fn test_sha_1() {
        println!("{}", CryptoTools::sha_1("Hello world"));
    }

    #[test]
    fn test_sha_256() {
        println!("{}", CryptoTools::sha_256("Hello world"));
    }

    #[test]
    fn test_sm3() {
        let data = SmTools::sm3("123456".as_bytes());
        let data = hex::decode(data).unwrap_or_default();
        println!("{}", BASE64_STANDARD.encode(data));
    }

    #[test]
    fn tst_sm2() {
        let private_key = "72d44a9e91a1dd3055540aafe16d8bd2a4ba88d3";
        let dat = SmTools::sm2_sign(private_key, "123456");
        println!("{dat}");
    }

    #[test]
    fn test_sm4() {
        // let key = "20513ef4ae9c3fbdb4e0ed6117863d10";
        let key = "26760c58d91f4d9518d74b0a95c8d387";
        let hex_key = CryptoTools::hex_decode(key).unwrap();

        // let text = "{\"transSerialNo\":\"0fd3ebaf-368b-411d-a2f3-41067d5a50b4\",\"partnerName\":\"P_XWB_PE_GP\",\"partnerCode\":\"P_XWB_PE_GP\",\"departmentCode\":\"_2351416_0001\",\"transferCode\":\"APPLY\",\"schemeName\":\"政企（单三者）\",\"contract\":{\"baseInfo\":{\"productCode\":\"MP13000097\",\"inputNetworkFlag\":\"internet\",\"dataSource\":\"openApi\",\"insuranceBeginDate\":\"2026-01-28 00:00:01\",\"insuranceEndDate\":\"2026-01-28 23:59:59\"},\"attachmentGroupList\":[{\"documentGroupId\":\"P_XWB_PE_GP-20260116151210-da2d665895ea4e3f94cdbd4f00a7c0c9\",\"documentGroupType\":\"GRZJ/TTZJ01\"}],\"extendInfo\":{\"isPolicyBeforePayfee\":\"1\"},\"applicantInfoList\":[{\"name\":\"成都拓域跃腾科技有限责任公司\",\"linkManName\":\"张三\",\"certificateNo\":\"91510107MA697XGQ9A\",\"mobileTelephone\":\"13980489834\",\"province\":\"510000\",\"city\":\"510100\",\"county\":\"510101\",\"address\":\"成都市武侯区电信南街高速大厦301室\",\"personnelType\":\"0\",\"certificateType\":\"03\",\"birthday\":\"\",\"age\":\"0\",\"nationality\":\"156\",\"sexCode\":\"M\"}],\"insurantInfoList\":[{\"name\":\"成都拓域跃腾科技有限责任公司\",\"linkManName\":\"张三\",\"certificateNo\":\"91510107MA697XGQ9A\",\"mobileTelephone\":\"13980489834\",\"province\":\"510000\",\"city\":\"510100\",\"county\":\"510101\",\"address\":\"成都市武侯区电信南街高速大厦301室\",\"personnelType\":\"0\",\"certificateType\":\"03\",\"birthday\":\"\",\"age\":\"0\",\"nationality\":\"156\",\"sexCode\":\"M\"}],\"riskGroupInfoList\":[{\"riskPropertyInfoList\":[{\"riskPropertyMap\":{\"insrUhbtPlneInfo\":[{\"uhbtPlneBrnd\":\"大疆\",\"uhbtPlneMdl\":\"DJIM300RTK\",\"uhbtPlneMachType\":\"1\",\"uhbtPlnePwrType\":\"0\",\"plneBodyCode\":\"A19045-10001\",\"mainCtlCode\":\"A19045-10001\",\"uhbtPlneInvcTm\":\"2022-08-09\",\"plneBodyInsrAmt\":\"0\",\"plneBodyFeeRate\":\"0\",\"thrdInsrAmt\":\"500000.00\",\"thrdDutyFeeRate\":\"0.2\",\"uhbtPlneMainPurpCatg\":\"09\",\"uhbtPlneMaxTakeOffWt\":\"100KG\"}],\"crryMass\":[]}}]}]}}";
        // let data = SmTools::sm4_ecb(&hex_key, text.as_bytes(), true);
        // let data_str = BASE64_STANDARD.encode(data);

        // let data_str = "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";

        let data_str = "YXTodkjwIIS/XVVcvrnV6Ac19dOOS706KGC3q+Wn/VaTPZEndaQ2pGMg7HhZIgi4f6ovfo1PrA0s6i+ufgKeP/kccQ7ykiRV4LPYb1dDNAXA9GbVcbuuW7/zeVJr94X3PGUJMd8r8RdAgfDqRGXcyjacNwRG+o71MQ2XFlbSt1U1T7PzJBnK27o/D4tFeQOS+R9L7qDQcro4LQTFYKfVHDeadqbmltiAM9FHPgrxASOCR1gBPtaK+8GVGAUZ0Eg4F/Ppf7AEE+kUy0Fr/QEyh73m7bRTbOYw+my15+KNzhBt723O0ZYveAPfrX2xvykBRpg/bizF9mF0n/dTj+1CQmXydZS0KOnKYjx3e4yUpofVt2h7JU5BeqxI2jH2CcTFlC5OKpr7dPg1IWPYTrJVk/phf0TDiuWYezcYQg+1G+OF8VhXKD21y+k6n+QB2A78JOiPjhnaX+IKVN7mrcjoCL/+x55oInBOISt5hRvVmMX+1KebDEFyFqS09gv+ZAIMkDh7rq6gpnSqF3N3NMjsa6LPwUndmb8EEOZwKB0++zyrmqHqo22X0WrJvJs3y4BK0/DR5OVtgUhMVKZ0nXUr8g==";

        println!("{data_str}");
        let text = BASE64_STANDARD.decode(data_str).unwrap();
        let str = SmTools::sm4_ecb(&hex_key, &text, false);
        println!("{}", String::from_utf8(str).unwrap());
    }

    #[test]
    fn test_hmac_sha256() {
        let data =
            CryptoTools::hmac_sha256_base64("e1K9gGrN27wZGPu4JVZEKlnRHKF6DcFq", "HmacSHA256");
        println!("{data}")
    }
}
