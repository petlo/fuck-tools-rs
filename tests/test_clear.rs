#[cfg(test)]
mod test_clear {
    use fuck_tools_rs::utils::tools_clear::ClearTools;

    #[test]
    fn test_clean_str() {
        // 基本功能测试
        assert_eq!(ClearTools::clean_str(r#""hello""#), "hello");
        assert_eq!(ClearTools::clean_str(r#""world""#), "world");

        // 多个引号
        assert_eq!(ClearTools::clean_str(r#""""test""""#), "test");

        // 带空格的引号
        assert_eq!(ClearTools::clean_str(r#"  "test"  "#), "test");

        // 混合引号
        assert_eq!(
            ClearTools::clean_str(r#""hello" "world""#),
            r#"hello world"#
        );

        // 没有引号的情况
        assert_eq!(ClearTools::clean_str("hello world"), "hello world");

        // 空字符串
        assert_eq!(ClearTools::clean_str(""), "");
        assert_eq!(ClearTools::clean_str(r#""""#), "");

        // 只有空格和引号
        assert_eq!(ClearTools::clean_str(r#"  ""  "#), "");
    }

    #[test]
    fn test_remove_whitespace() {
        // 普通空格
        assert_eq!(ClearTools::remove_whitespace("hello world"), "helloworld");

        // 各种空白字符
        assert_eq!(
            ClearTools::remove_whitespace("hello\tworld\n\r"),
            "helloworld"
        );

        // 多个连续空格
        assert_eq!(
            ClearTools::remove_whitespace("hello    world"),
            "helloworld"
        );

        // 字符串开头和结尾的空格
        assert_eq!(
            ClearTools::remove_whitespace("  hello world  "),
            "helloworld"
        );

        // 包含所有常见空白字符
        assert_eq!(
            ClearTools::remove_whitespace("\t\n\r hello \u{3000}world \u{2003}"),
            "helloworld"
        );

        // 空字符串
        assert_eq!(ClearTools::remove_whitespace(""), "");

        // 只有空白字符
        assert_eq!(ClearTools::remove_whitespace(" \t\n\r "), "");
    }

    #[test]
    fn test_strip_html_tags() {
        // 简单标签
        assert_eq!(ClearTools::strip_html_tags("<p>hello</p>"), "hello");

        // 多个标签
        assert_eq!(
            ClearTools::strip_html_tags("<div><p>hello</p><span>world</span></div>"),
            "helloworld"
        );

        // 自闭合标签
        assert_eq!(ClearTools::strip_html_tags("hello<br/>world"), "helloworld");

        // 带属性的标签
        assert_eq!(
            ClearTools::strip_html_tags(r#"<a href="http://example.com">link</a>"#),
            "link"
        );

        // 混合内容
        assert_eq!(
            ClearTools::strip_html_tags("text before <b>bold</b> text after"),
            "text before bold text after"
        );

        // 没有标签的情况
        assert_eq!(ClearTools::strip_html_tags("plain text"), "plain text");

        // 嵌套标签
        assert_eq!(
            ClearTools::strip_html_tags("<div><span><b>nested</b></span></div>"),
            "nested"
        );

        // 空标签
        assert_eq!(ClearTools::strip_html_tags("<>"), "");

        // 不完整的标签
        assert_eq!(ClearTools::strip_html_tags("hello < world"), "hello ");

        // 空字符串
        assert_eq!(ClearTools::strip_html_tags(""), "");
    }

    #[test]
    fn test_remove_control_chars() {
        // 控制字符
        assert_eq!(
            ClearTools::remove_control_chars("hello\x00world\x1F"),
            "helloworld"
        );

        // 换行符、制表符
        assert_eq!(
            ClearTools::remove_control_chars("hello\n\t\rworld"),
            "helloworld"
        );

        // Unicode 控制字符
        assert_eq!(
            ClearTools::remove_control_chars("hello\u{0007}world\u{001B}"),
            "helloworld"
        );

        // 普通文本（无控制字符）
        assert_eq!(
            ClearTools::remove_control_chars("正常文本 123 abc"),
            "正常文本 123 abc"
        );

        // 混合内容
        assert_eq!(
            ClearTools::remove_control_chars("test\x08\x0Ctext"),
            "testtext"
        );

        // 空字符串
        assert_eq!(ClearTools::remove_control_chars(""), "");

        // 只有控制字符
        assert_eq!(ClearTools::remove_control_chars("\x00\x01\x02\x03"), "");
    }

    #[test]
    fn test_normalize_whitespace() {
        // 多个连续空格
        assert_eq!(
            ClearTools::normalize_whitespace("hello    world"),
            "hello world"
        );

        // 混合空白字符
        assert_eq!(
            ClearTools::normalize_whitespace("hello\t\n\r world"),
            "hello world"
        );

        // 开头和结尾的空格
        assert_eq!(
            ClearTools::normalize_whitespace("  hello world  "),
            "hello world"
        );

        // 只有空格
        assert_eq!(ClearTools::normalize_whitespace("     "), "");

        // 制表符和空格混合
        assert_eq!(
            ClearTools::normalize_whitespace("hello\t\t\tworld"),
            "hello world"
        );

        // 换行符处理
        assert_eq!(
            ClearTools::normalize_whitespace("hello\n\n\nworld"),
            "hello world"
        );

        // 中文和空格混合
        assert_eq!(
            ClearTools::normalize_whitespace("你好    世界"),
            "你好 世界"
        );

        // 空字符串
        assert_eq!(ClearTools::normalize_whitespace(""), "");

        // 复杂案例
        assert_eq!(
            ClearTools::normalize_whitespace(
                "  \t  multiple \n\r\n  spaces  \t between  \r\n words  "
            ),
            "multiple spaces between words"
        );
    }

    #[test]
    fn test_method_chaining() {
        // 测试方法组合使用
        let input = r#"  <p>"hello"    world  </p>  "#;

        // 先移除HTML标签
        let step1 = ClearTools::strip_html_tags(input);
        assert_eq!(step1, r#"  "hello"    world    "#);

        // 再清理引号
        let step2 = ClearTools::clean_str(&step1);
        assert_eq!(step2, r#"hello    world"#);

        // 最后规范化空格
        let result = ClearTools::normalize_whitespace(&step2);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_edge_cases() {
        // 非常长的字符串
        let long_string = "a".repeat(1000);
        assert_eq!(ClearTools::clean_str(&long_string), long_string);

        // Unicode字符
        assert_eq!(ClearTools::clean_str(r#""🎉庆祝🎉""#), "🎉庆祝🎉");

        // 特殊Unicode空格
        assert_eq!(
            ClearTools::remove_whitespace("hello\u{2003}world"),
            "helloworld"
        );

        // HTML实体（应该保留）
        assert_eq!(ClearTools::strip_html_tags("a &amp; b"), "a &amp; b");
    }

    #[test]
    fn test_performance() {
        // 性能测试（简单的基准测试）
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            ClearTools::clean_str(r#""test""#);
        }

        let duration = start.elapsed();
        println!("clean_str 1000次耗时: {:?}", duration);

        // 断言执行时间在合理范围内
        assert!(duration < std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_empty_and_whitespace_only() {
        // 各种空和空白情况
        let cases = vec![
            "",
            " ",
            "\t",
            "\n",
            "\r",
            " \t\n\r ",
            "\"\"",
            " \"\" ",
            "<></>",
            "\x00\x01\x02",
        ];

        for case in cases {
            // 所有方法都应能安全处理空输入
            assert_eq!(ClearTools::clean_str(case), case.replace("\"", "").trim());
            assert_eq!(
                ClearTools::remove_whitespace(case),
                case.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
            );
            assert_eq!(ClearTools::strip_html_tags(case), {
                let mut result = String::new();
                let mut in_tag = false;
                for c in case.chars() {
                    match c {
                        '<' => in_tag = true,
                        '>' => in_tag = false,
                        _ => {
                            if !in_tag {
                                result.push(c);
                            }
                        }
                    }
                }
                result
            });
        }
    }
}
